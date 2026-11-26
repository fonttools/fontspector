// Code to make GSUB tables easier to work with
use skrifa::raw::{
    tables::{
        gsub::{
            AlternateSubst, ExtensionSubstFormat1, LigatureSubst, MultipleSubst,
            ReverseChainSingleSubst, SingleSubst, SubstitutionSubtables,
        },
        layout::Subtables,
    },
    types::{GlyphId, GlyphId16, GlyphId24},
    ReadError,
};

/// A map of substitutions, input glyphs on the left, output glyphs on the right
pub type SubstitutionMap = Vec<(Vec<GlyphId16>, Vec<GlyphId16>)>;

/// A trait to get a substitution map from a GSUB subtable
pub trait GetSubstitutionMap {
    /// Get the substitution map from the subtable
    fn substitutions(&self) -> Result<SubstitutionMap, ReadError>;
}

/// This type isn't public in `read-fonts` so copy it in
type SubSubtables<'a, T> = Subtables<'a, T, ExtensionSubstFormat1<'a, T>>;

fn gid16_from(gid: impl Into<GlyphId>) -> Option<GlyphId16> {
    GlyphId16::try_from(gid.into()).ok()
}

fn gid16_from_24(gid: GlyphId24) -> Option<GlyphId16> {
    GlyphId16::try_from(GlyphId::from(gid.to_u32())).ok()
}

impl<'a> GetSubstitutionMap for SubSubtables<'a, SingleSubst<'a>> {
    fn substitutions(&self) -> Result<Vec<(Vec<GlyphId16>, Vec<GlyphId16>)>, ReadError> {
        let mut result = vec![];
        for subtable in self.iter() {
            match subtable? {
                SingleSubst::Format1(table_ref) => {
                    let delta = table_ref.delta_glyph_id() as i32;
                    for old_gid in table_ref.coverage()?.iter() {
                        let Some(old) = gid16_from(old_gid) else {
                            continue;
                        };
                        let new_gid = GlyphId16::from((old.to_u32() as i32 + delta) as u16);
                        result.push((vec![old], vec![new_gid]));
                    }
                }
                SingleSubst::Format2(table_ref) => {
                    for (old_gid, new_gid) in table_ref
                        .coverage()?
                        .iter()
                        .zip(table_ref.substitute_glyph_ids().iter().map(|gid| gid.get()))
                    {
                        let Some(old) = gid16_from(old_gid) else {
                            continue;
                        };
                        let Some(new) = gid16_from(new_gid) else {
                            continue;
                        };
                        result.push((vec![old], vec![new]));
                    }
                }
                // Format3/4 use 24-bit glyph IDs; unused by checks today.
                _ => {}
            }
        }
        Ok(result)
    }
}

impl<'a> GetSubstitutionMap for SubSubtables<'a, MultipleSubst<'a>> {
    fn substitutions(&self) -> Result<Vec<(Vec<GlyphId16>, Vec<GlyphId16>)>, ReadError> {
        let mut result = vec![];
        for subtable in self.iter() {
            match subtable? {
                MultipleSubst::Format1(subtable) => {
                    for (old_gid, sequence) in subtable
                        .coverage()?
                        .iter()
                        .zip(subtable.sequences().iter().flatten())
                    {
                        let Some(old) = gid16_from(old_gid) else {
                            continue;
                        };
                        let new_gids = sequence
                            .substitute_glyph_ids()
                            .iter()
                            .filter_map(|gid| gid16_from(gid.get()))
                            .collect();
                        result.push((vec![old], new_gids));
                    }
                }
                MultipleSubst::Format2(subtable) => {
                    for (old_gid, sequence) in subtable
                        .coverage()?
                        .iter()
                        .zip(subtable.sequences().iter().flatten())
                    {
                        let Some(old) = gid16_from(old_gid) else {
                            continue;
                        };
                        let new_gids = sequence
                            .substitute_glyph_ids()
                            .iter()
                            .filter_map(|gid| gid16_from_24(gid.get()))
                            .collect();
                        result.push((vec![old], new_gids));
                    }
                }
            }
        }
        Ok(result)
    }
}

impl<'a> GetSubstitutionMap for SubSubtables<'a, AlternateSubst<'a>> {
    fn substitutions(&self) -> Result<Vec<(Vec<GlyphId16>, Vec<GlyphId16>)>, ReadError> {
        let mut result = vec![];
        for subtable in self.iter() {
            match subtable? {
                AlternateSubst::Format1(subtable) => {
                    for (old_gid, sequence) in subtable
                        .coverage()?
                        .iter()
                        .zip(subtable.alternate_sets().iter().flatten())
                    {
                        let Some(old) = gid16_from(old_gid) else {
                            continue;
                        };
                        let new_gids = sequence
                            .alternate_glyph_ids()
                            .iter()
                            .filter_map(|gid| gid16_from(gid.get()))
                            .collect();
                        result.push((vec![old], new_gids));
                    }
                }
                AlternateSubst::Format2(subtable) => {
                    for (old_gid, sequence) in subtable
                        .coverage()?
                        .iter()
                        .zip(subtable.alternate_sets().iter().flatten())
                    {
                        let Some(old) = gid16_from(old_gid) else {
                            continue;
                        };
                        let new_gids = sequence
                            .alternate_glyph_ids()
                            .iter()
                            .filter_map(|gid| gid16_from_24(gid.get()))
                            .collect();
                        result.push((vec![old], new_gids));
                    }
                }
            }
        }
        Ok(result)
    }
}

impl<'a> GetSubstitutionMap for SubSubtables<'a, LigatureSubst<'a>> {
    fn substitutions(&self) -> Result<Vec<(Vec<GlyphId16>, Vec<GlyphId16>)>, ReadError> {
        let mut result = vec![];
        for subtable in self.iter() {
            match subtable? {
                LigatureSubst::Format1(subtable) => {
                    for (first_gid, sequence) in subtable
                        .coverage()?
                        .iter()
                        .zip(subtable.ligature_sets().iter().flatten())
                    {
                        let Some(first) = gid16_from(first_gid) else {
                            continue;
                        };
                        for ligature in sequence.ligatures().iter().flatten() {
                            let remaining_gids = ligature
                                .component_glyph_ids()
                                .iter()
                                .filter_map(|gid| gid16_from(gid.get()));
                            let new_gid = ligature.ligature_glyph();
                            let mut lhs = vec![first];
                            lhs.extend(remaining_gids);
                            result.push((lhs, vec![new_gid]));
                        }
                    }
                }
                LigatureSubst::Format2(subtable) => {
                    for (first_gid, sequence) in subtable
                        .coverage()?
                        .iter()
                        .zip(subtable.ligature_sets().iter().flatten())
                    {
                        let Some(first) = gid16_from(first_gid) else {
                            continue;
                        };
                        for ligature in sequence.ligatures().iter().flatten() {
                            let remaining_gids = ligature
                                .component_glyph_ids()
                                .iter()
                                .filter_map(|gid| gid16_from_24(gid.get()));
                            let Some(new_gid) = gid16_from_24(ligature.ligature_glyph()) else {
                                continue;
                            };
                            let mut lhs = vec![first];
                            lhs.extend(remaining_gids);
                            result.push((lhs, vec![new_gid]));
                        }
                    }
                }
            }
        }
        Ok(result)
    }
}

impl<'a> GetSubstitutionMap for SubSubtables<'a, ReverseChainSingleSubst<'a>> {
    fn substitutions(&self) -> Result<Vec<(Vec<GlyphId16>, Vec<GlyphId16>)>, ReadError> {
        let mut result = vec![];
        for subtable in self.iter() {
            match subtable? {
                ReverseChainSingleSubst::Format1(subtable) => {
                    for (lgid, rgid) in subtable
                        .coverage()?
                        .iter()
                        .zip(subtable.substitute_glyph_ids().iter())
                    {
                        let Some(left) = gid16_from(lgid) else {
                            continue;
                        };
                        let Some(right) = gid16_from(rgid.get()) else {
                            continue;
                        };
                        result.push((vec![left], vec![right]));
                    }
                }
                ReverseChainSingleSubst::Format2(subtable) => {
                    for (lgid, rgid) in subtable
                        .coverage()?
                        .iter()
                        .zip(subtable.substitute_glyph_ids().iter())
                    {
                        let Some(left) = gid16_from(lgid) else {
                            continue;
                        };
                        let Some(right) = gid16_from_24(rgid.get()) else {
                            continue;
                        };
                        result.push((vec![left], vec![right]));
                    }
                }
            }
        }
        Ok(result)
    }
}
impl GetSubstitutionMap for SubstitutionSubtables<'_> {
    fn substitutions(&self) -> Result<Vec<(Vec<GlyphId16>, Vec<GlyphId16>)>, ReadError> {
        match self {
            SubstitutionSubtables::Single(st) => st.substitutions(),
            SubstitutionSubtables::Multiple(st) => st.substitutions(),
            SubstitutionSubtables::Alternate(st) => st.substitutions(),
            SubstitutionSubtables::Ligature(st) => st.substitutions(),
            SubstitutionSubtables::Reverse(st) => st.substitutions(),
            _ => Ok(vec![]),
        }
    }
}
