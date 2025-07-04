//! Quality control for OpenType fonts

mod args;
mod configuration;
mod profiles;
mod reporters;

use std::{
    collections::HashMap,
    io::Write,
    path::PathBuf,
    time::{Duration, Instant},
};

use args::Args;
use clap::{CommandFactory, FromArgMatches};

use fontspector_checkapi::{
    Check, CheckResult, Context, FixResult, FixSourceFunction, HotfixFunction, Registry,
    SourceFile, StatusCode, Testable, TestableCollection, TestableType,
};

#[cfg(not(debug_assertions))]
use indicatif::ParallelProgressIterator;
#[cfg(debug_assertions)]
use indicatif::ProgressIterator;

use configuration::{load_configuration, UserConfigurationFile};
use itertools::Either;
use profiles::{register_and_return_toml_profile, register_core_profiles};

#[cfg(not(debug_assertions))]
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

use reporters::{process_reporter_args, terminal::TerminalReporter, Reporter, RunResults};
use serde_json::json;

use shadow_rs::shadow;

shadow!(build);

// As a special case for Google fonts, all files in an article/
// directory are associated with the parent's group.
const COLLAPSED_SUBDIRECTORIES: [&str; 1] = ["article"];

fn main() {
    let start_time = Instant::now();

    // Command line handling
    let mut cmd2 = Args::command();
    #[allow(clippy::const_is_empty)] // const is not const
    let short_version: String = format!(
        "{}{}",
        env!("CARGO_PKG_VERSION"),
        if build::TAG.is_empty() {
            " (".to_owned() + build::SHORT_COMMIT + ")"
        } else {
            "".to_owned()
        }
    );
    cmd2 = cmd2
        .version(short_version)
        .long_version(build::CLAP_LONG_VERSION);
    let mut matches = cmd2.get_matches();
    let mut args = Args::from_arg_matches_mut(&mut matches).unwrap_or_else(|e| {
        let _ = e.print();
        std::process::exit(1);
    });

    env_logger::init_from_env(env_logger::Env::default().filter_or(
        env_logger::DEFAULT_FILTER_ENV,
        match args.verbose {
            0 => "warn",
            1 => "info",
            _ => "debug",
        },
    ));

    let any_reports_to_stdout = reporters::any_stdout(&args).unwrap_or_else(|e| {
        print!("{e}");
        std::process::exit(1);
    });

    #[cfg(not(debug_assertions))]
    if let Some(threads) = args.jobs {
        let mut builder = rayon::ThreadPoolBuilder::new().num_threads(threads);
        if threads == 1 {
            builder = builder.use_current_thread();
        }
        builder.build_global().expect("Could not set thread count");
    }

    // Set up the check registry
    let mut registry = Registry::new();

    register_core_profiles(&args, &mut registry);

    for plugin_path in args.plugins.iter() {
        if let Err(err) = registry.load_plugin(plugin_path) {
            log::error!("Could not load plugin {plugin_path:}: {err:}");
        }
    }

    // Load the relevant profile - maybe it's a file?
    let profile_name = if args.profile.ends_with(".toml") {
        register_and_return_toml_profile(&args, &mut registry)
    } else {
        args.profile.clone()
    };

    let profile = registry.get_profile(&profile_name).unwrap_or_else(|| {
        log::error!("Could not find profile {:}", args.profile);
        std::process::exit(1);
    });

    if args.list_checks || args.list_checks_json {
        list_checks(&args, &registry, profile);
    }
    // We create one collection for each set of testable files in a directory.
    // So let's group the inputs per directory, and then map them into a FontCollection
    let grouped_inputs = group_inputs(&mut args);

    if grouped_inputs.is_empty() {
        log::error!("No input files");
        std::process::exit(1);
    }

    let testables: Vec<TestableType> = grouped_inputs
        .iter()
        .flat_map(|x| x.collection_and_files())
        .collect();

    if testables.is_empty() {
        log::error!("No input files");
        std::process::exit(1);
    }

    // Load configuration
    let configuration: UserConfigurationFile = load_configuration(&args);
    let overrides = configuration.overrides.clone().unwrap_or_default();
    let mut includes = args.checkid.clone().unwrap_or_default();
    let mut excludes = args.exclude_checkid.clone().unwrap_or_default();
    if let Some(more_includes) = configuration.explicit_checks.as_ref() {
        includes.extend(more_includes.iter().cloned());
    }
    if let Some(more_excludes) = configuration.exclude_checks.as_ref() {
        excludes.extend(more_excludes.iter().cloned());
    }
    let mut source_map = configuration.source_map.clone();
    for (key, val) in args.source_map.clone() {
        source_map.insert(key, val);
    }

    // Establish a check order
    let checkorder: Vec<(String, &TestableType, &Check, Context)> = profile.check_order(
        &includes,
        &excludes,
        &registry,
        Context {
            skip_network: args.skip_network,
            network_timeout: Some(10), // XXX
            configuration: HashMap::new(),
            check_metadata: serde_json::Value::Null,
            full_lists: args.full_lists,
            cache: Default::default(),
            overrides,
        },
        &configuration.per_check_config,
        &testables,
    );

    // The testables are the collection object plus the files; only count the files.
    let count_of_files = testables.iter().filter(|x| x.is_single()).count();
    let count_of_families = testables.len() - count_of_files;

    if !any_reports_to_stdout {
        let _ = writeln!(
            std::io::stdout(),
            "Running {:} check{} on {} file{} in {} famil{}",
            checkorder.len(),
            if checkorder.len() == 1 { "" } else { "s" },
            count_of_files,
            if count_of_files == 1 { "" } else { "s" },
            count_of_families,
            if count_of_families == 1 { "y" } else { "ies" }
        );
    }

    // Run all the things! Check all the fonts!

    // Do this in parallel for release, serial for debug
    #[cfg(debug_assertions)]
    let checkorder_iterator = if args.quiet {
        Either::Left(checkorder.iter())
    } else {
        Either::Right(checkorder.iter().progress())
    };
    #[cfg(not(debug_assertions))]
    let checkorder_iterator = if checkorder.clone().len() > 100_000 && !args.quiet {
        Either::Left(checkorder.par_iter().progress())
    } else {
        Either::Right(checkorder.par_iter())
    };

    let mut results: RunResults = checkorder_iterator
        .map(|(sectionname, testable, check, context)| {
            (
                testable,
                check,
                check.run(testable, context, Some(sectionname)),
            )
        })
        .filter_map(|(_, _, result)| result)
        .collect::<Vec<CheckResult>>()
        .into();

    if args.hotfix || args.fix_sources {
        try_fixing_stuff(&mut results, &args, &registry, &source_map);
    }

    let worst_status = results.worst_status();

    let mut reporters: Vec<Box<dyn Reporter>> = vec![];
    if !args.quiet && !any_reports_to_stdout {
        reporters.push(Box::new(TerminalReporter::new(args.succinct)));
    }
    process_reporter_args(&args, &mut reporters);

    for reporter in reporters {
        reporter.report(&results, &args, &registry);
    }

    if !args.quiet && !any_reports_to_stdout {
        let _ = writeln!(
            std::io::stdout(),
            "Ran {} checks in {:.3}s",
            checkorder.len(),
            start_time.elapsed().as_secs_f32()
        );
        let _ = TerminalReporter::summary_report(results.summary());
    }

    if args.verbose >= 1 {
        let mut per_test_time = HashMap::new();
        for result in results.iter() {
            let time = per_test_time
                .entry(result.check_id.clone())
                .or_insert(Duration::default());
            *time += result.time;
        }
        let mut times: Vec<_> = per_test_time.iter().collect();
        times.sort_by_key(|(_, time)| -(time.as_micros() as i128));
        log::info!("\nTop 10 slowest checks:");
        for (check_id, time) in times.iter().take(10) {
            log::info!("{:}: {:.3}s", check_id, time.as_secs_f32());
        }
    }

    if worst_status >= args.error_code_on {
        std::process::exit(1);
    }
}

fn list_checks(args: &Args, registry: &Registry<'static>, profile: &fontspector_checkapi::Profile) {
    let mut checks_per_section = HashMap::new();
    for (section, checks) in profile.sections.iter() {
        let checks: Vec<_> = checks
            .iter()
            .flat_map(|check| registry.checks.get(check))
            .map(|check| json!({ "id": check.id, "title": check.title }))
            .collect();
        if checks.is_empty() {
            continue;
        }
        checks_per_section.insert(section.clone(), checks);
    }
    if args.list_checks_json {
        let _ = writeln!(
            std::io::stdout(),
            "{}",
            serde_json::to_string_pretty(&checks_per_section).unwrap_or("{}".to_string())
        );
    } else {
        for (section, checks) in checks_per_section.iter() {
            termimad::print_text(&format!("\n# {section:}\n\n"));
            let mut table = "|Check ID|Title|\n|---|---|\n".to_string();
            for check in checks {
                #[allow(clippy::unwrap_used)] // We know these keys are present, we made them
                table.push_str(&format!(
                    "|{}|{}|\n",
                    check.get("id").unwrap().as_str().unwrap(),
                    check.get("title").unwrap().as_str().unwrap()
                ));
            }
            termimad::print_text(&table);
        }
    }
    std::process::exit(0);
}

// Group each file into a set per directory, and wrap that in a TestableCollection.
// It feels like this takes an inordinately long time, but remember that this also
// reads the input files.
fn group_inputs(args: &mut Args) -> Vec<TestableCollection> {
    // As a fun Easter egg, if the input file is a single source file, we will
    // pass it to fontc, compile it and stick it in the source map.
    #[cfg(feature = "fontc")]
    #[allow(clippy::indexing_slicing)] // We know this is a single file
    if args.inputs.len() == 1
        && (args.inputs[0].ends_with(".glyphs")
            // || args.inputs[0].ends_with(".designspace")
            // || args.inputs[0].ends_with(".ufo")
            || args.inputs[0].ends_with(".glyphspackage"))
    {
        let path = PathBuf::from(&args.inputs[0]);
        if let Ok(input) = fontc::Input::new(&path) {
            use fontc::Input;

            log::info!("Compiling {}", &path.display());
            let flags = fontc::Flags::default();
            #[allow(clippy::expect_used)] // You are on your own
            let source = match input {
                // Input::DesignSpacePath(path) => Ok(Box::new(DesignSpaceIrSource::new(path)?)),
                Input::GlyphsPath(path) => {
                    use fontir::source::Source as _;

                    Ok(Box::new(
                        glyphs2fontir::source::GlyphsIrSource::new(&path)
                            .expect("Could not create GlyphsIrSource from Glyphs file"),
                    ))
                }
                // Input::FontraPath(path) => Ok(Box::new(FontraIrSource::new(path)?)),
                _ => Err(format!(
                    "Input file {} has unsupported format for compilation",
                    path.display()
                )),
            }
            .expect("Could not create fontir source from input file");
            match fontc::generate_font(
                source,
                &PathBuf::from("build/"),
                Some(&PathBuf::from("font.ttf")),
                flags,
                false,
            ) {
                Err(e) => {
                    log::error!("Could not compile font: {e}");
                    std::process::exit(1);
                }
                Ok(font) => {
                    let ttf_filename = path.with_extension("ttf");
                    let testable = Testable::new_with_contents(&ttf_filename, font);
                    return vec![TestableCollection::from_testables(vec![testable], None)];
                }
            }
        }
    }

    let inputs = args
        .inputs
        .iter()
        .map(PathBuf::from)
        .filter(|x| x.is_file())
        .filter(|x| x.parent().is_some());
    inputs
        .map(|file| {
            #[allow(clippy::unwrap_used)] // We tested for parent
            if COLLAPSED_SUBDIRECTORIES
                .iter()
                .any(|subdir| file.parent().unwrap().ends_with(subdir))
            {
                (file.parent().unwrap().parent().unwrap().to_owned(), file)
            } else {
                (file.parent().unwrap().to_owned(), file)
            }
        })
        .fold(
            HashMap::new(),
            |mut acc: HashMap<PathBuf, Vec<PathBuf>>, (directory, file)| {
                acc.entry(directory).or_default().push(file);
                acc
            },
        )
        .into_iter()
        .map(|(directory, group)| {
            TestableCollection::from_filenames(&group, directory.to_str()).unwrap_or_else(|e| {
                log::error!(
                    "Could not load files from {:?}: {:}",
                    group.first().map(|p| p.parent()),
                    e
                );
                std::process::exit(1)
            })
        })
        .collect()
}

// We have to consider sources and binaries together, because we're borrowing the
// CheckResult mutably, and we can't have two mutable borrows of the same thing.
struct FixJob<'a> {
    hotfix: Option<&'a HotfixFunction>, // The hotfix function, if any
    source_fix: Option<&'a FixSourceFunction>, // The source fix function, if any
    result: &'a mut CheckResult,        // The result to fix
}

fn try_fixing_stuff(
    results: &mut RunResults,
    args: &Args,
    registry: &Registry,
    source_map: &HashMap<String, String>,
) {
    let failed_checks = results
        .iter_mut()
        .filter(|x| x.worst_status() >= StatusCode::Warn)
        .collect::<Vec<_>>();
    // Group the fixes by filename because we want to provide testables
    let mut fix_jobs: HashMap<String, Vec<FixJob>> = HashMap::new();

    for result in failed_checks.into_iter() {
        let Some(check) = registry.checks.get(&result.check_id) else {
            log::warn!(
                "A check called {} just mysteriously vanished",
                result.check_id
            );
            continue;
        };
        let Some(result_file) = result.filename.clone() else {
            continue;
        };
        let fix_job = FixJob {
            hotfix: if args.hotfix { check.hotfix } else { None },
            source_fix: if args.fix_sources {
                check.fix_source
            } else {
                None
            },
            result,
        };
        fix_jobs
            .entry(result_file.clone())
            .or_default()
            .push(fix_job);
    }

    for (file, fixes) in fix_jobs.into_iter() {
        let mut testable = Testable::new(&file).unwrap_or_else(|e| {
            log::error!("Could not load files from {file:?}: {e:}");
            std::process::exit(1)
        });
        let mut source: Option<SourceFile> = if !args.fix_sources {
            None
        } else {
            find_source(&file, source_map)
        };
        let mut source_modified = false;
        let mut binary_modified = false;
        for fix_job in fixes.into_iter() {
            if let Some(hotfix_fn) = fix_job.hotfix {
                log::info!("Trying to hoxfix {file} with {}", fix_job.result.check_id);
                fix_job.result.hotfix_result = match hotfix_fn(&mut testable) {
                    Ok(hotfix_behaviour) => {
                        binary_modified |= hotfix_behaviour;
                        Some(FixResult::Fixed)
                    }
                    Err(e) => Some(FixResult::FixError(e.to_string())),
                }
            }
            if let Some(sourcefix_fn) = fix_job.source_fix {
                if let Some(ref mut source) = &mut source {
                    log::info!(
                        "Trying to fix source {} with {}",
                        source.filename(),
                        fix_job.result.check_id
                    );
                    fix_job.result.hotfix_result = match sourcefix_fn(source) {
                        Ok(hotfix_behaviour) => {
                            source_modified |= hotfix_behaviour;
                            Some(FixResult::Fixed)
                        }
                        Err(e) => Some(FixResult::FixError(e.to_string())),
                    }
                }
            }
        }
        if binary_modified {
            // save it
            testable.save().unwrap_or_else(|e| {
                log::error!("Could not save file {file:?}: {e:}");
                std::process::exit(1)
            });
        }
        if source_modified {
            // save it
            #[allow(clippy::unwrap_used)] // We know this is Some
            source.unwrap().save().unwrap_or_else(|e| {
                log::error!("Could not save file {file:?}: {e:}");
                std::process::exit(1)
            });
        }
    }
}

fn find_source(file: &str, source_map: &HashMap<String, String>) -> Option<SourceFile> {
    // Find the source in the source map
    let source_path_str = if let Some(source) = source_map.get(file) {
        source
    } else {
        log::warn!("No source file found for {file:?} in source map; cannot fix sources");
        log::warn!("Specify --source_map=binary_file.ttf:source.glyphs on command line or in configuration file to fix sources");
        return None;
    };
    let source_path = PathBuf::from(source_path_str);
    // Now load it
    Some(SourceFile::new(&source_path).unwrap_or_else(|e| {
        log::error!("Could not load source file {source_path:?}: {e:}");
        std::process::exit(1)
    }))
}
