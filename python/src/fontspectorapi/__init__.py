#!/usr/bin/env python3
"""Python helper API for subprocess fontspector plugins.

This module provides a small FontBakery-inspired authoring experience for
fontspector subprocess plugins:

- Status code constants (PASS, FAIL, WARN, ...)
- A `Message` helper object
- A `@check(...)` decorator for check metadata
- A plugin runtime that serves `--metadata` and `--check`
"""

from __future__ import annotations

import argparse
import json
import sys
import traceback
from dataclasses import dataclass, field
from typing import Any, Callable, Dict, Iterator, List, Optional, Sequence, Tuple, Union

SKIP = "SKIP"
INFO = "INFO"
PASS = "PASS"
WARN = "WARN"
FAIL = "FAIL"
FATAL = "FATAL"
ERROR = "ERROR"

VALID_SEVERITIES = {SKIP, INFO, PASS, WARN, FAIL, FATAL, ERROR}


@dataclass
class Message:
    """A FontBakery-like check message payload."""

    code: str
    message: str
    metadata: List[Dict[str, Any]] = field(default_factory=list)


@dataclass
class CheckContext:
    """Context object passed to check functions."""

    check_id: str
    check_metadata: Dict[str, Any]
    files: List[str]
    cache: Dict[str, Any] = field(default_factory=dict)


StatusYield = Union[
    str,
    Tuple[str, str],
    Tuple[str, Message],
    Tuple[str, str, str],
]

CheckStatuses = Iterator[StatusYield]


@dataclass
class CheckDefinition:
    id: str
    title: str
    rationale: str
    proposal: List[str]
    applies_to: str
    runs_on_collection: bool
    metadata: Dict[str, Any]
    hotfix_available: bool = False
    sourcefix_available: bool = False
    func: Optional[Callable[..., CheckStatuses]] = None


@dataclass
class ProfileDefinition:
    sections: Dict[str, List[str]]
    include_profiles: List[str] = field(default_factory=list)
    exclude_checks: List[str] = field(default_factory=list)
    overrides: Dict[str, Any] = field(default_factory=dict)
    configuration_defaults: Dict[str, Any] = field(default_factory=dict)


class Plugin:
    """In-memory registry used by Python plugins."""

    def __init__(self, plugin_name: str = "python-plugin") -> None:
        self.plugin_name = plugin_name
        self._checks: Dict[str, CheckDefinition] = {}
        self._profiles: Dict[str, ProfileDefinition] = {}
        self._filetypes: Dict[str, str] = {}

    def register_filetype(self, name: str, pattern: str) -> None:
        self._filetypes[name] = pattern

    def register_check(self, fn: Callable[..., CheckStatuses]) -> None:
        meta = getattr(fn, "_fontspector_check", None)
        if meta is None:
            raise ValueError("Function is missing @check metadata")
        check_def = CheckDefinition(
            id=meta["id"],
            title=meta["title"],
            rationale=meta["rationale"],
            proposal=meta["proposal"],
            applies_to=meta["applies_to"],
            runs_on_collection=meta["runs_on_collection"],
            metadata=meta["metadata"],
            func=fn,
        )
        self._checks[check_def.id] = check_def

    def register_simple_profile(
        self,
        profile_name: str,
        check_fns: Sequence[Callable[..., CheckStatuses]],
        section_name: Optional[str] = None,
    ) -> None:
        for fn in check_fns:
            self.register_check(fn)
        section = section_name or profile_name
        self.register_profile(
            profile_name,
            ProfileDefinition(
                sections={section: [fn._fontspector_check["id"] for fn in check_fns]}
            ),
        )

    def register_profile(self, profile_name: str, profile: ProfileDefinition) -> None:
        for check_ids in profile.sections.values():
            for check_id in check_ids:
                if check_id not in self._checks:
                    raise ValueError(
                        f"Profile {profile_name} references unknown check id: {check_id}"
                    )
        self._profiles[profile_name] = profile

    def metadata_payload(self) -> Dict[str, Any]:
        return {
            "api_version": 1,
            "plugin_name": self.plugin_name,
            "profiles": {
                name: {
                    "sections": profile.sections,
                    "include_profiles": profile.include_profiles,
                    "exclude_checks": profile.exclude_checks,
                    "overrides": profile.overrides,
                    "configuration_defaults": profile.configuration_defaults,
                }
                for name, profile in self._profiles.items()
            },
            "checks": [
                {
                    "id": c.id,
                    "title": c.title,
                    "rationale": c.rationale,
                    "proposal": c.proposal,
                    "applies_to": c.applies_to,
                    "runs_on_collection": c.runs_on_collection,
                    "metadata": c.metadata,
                    "hotfix_available": c.hotfix_available,
                    "sourcefix_available": c.sourcefix_available,
                }
                for c in self._checks.values()
            ],
            "filetypes": self._filetypes,
        }

    def run_check(self, check_id: str, files: Sequence[str]) -> Dict[str, Any]:
        if check_id not in self._checks:
            raise ValueError(f"Unknown check id: {check_id}")

        check_def = self._checks[check_id]
        if check_def.func is None:
            raise ValueError(f"Check {check_id} has no implementation")

        if not check_def.runs_on_collection and len(files) != 1:
            raise ValueError("Single-file checks require exactly one file")

        context = CheckContext(
            check_id=check_id,
            check_metadata=check_def.metadata,
            files=list(files),
        )

        statuses = []
        filename = None if check_def.runs_on_collection else str(files[0])
        try:
            if check_def.runs_on_collection:
                yielded = check_def.func(list(files), context)
            else:
                yielded = check_def.func(str(files[0]), context)

            while True:
                try:
                    statuses.append(next(yielded))
                except StopIteration:
                    break
                except Exception as e:
                    statuses.append(
                        dict(
                            severity=ERROR,
                            code=e.__class__.__name__,
                            message=f"{e}\n{traceback.print_exc()}",
                        )
                    )
                    # ...but keep running
        except Exception as e:
            statuses.append(
                dict(
                    severity=ERROR,
                    code=e.__class__.__name__,
                    message=f"unable to run check: {e}\n{traceback.print_exc()}",
                )
            )
        worst = worst_status(statuses)

        return {
            "check_id": check_def.id,
            "check_name": check_def.title,
            "check_rationale": check_def.rationale,
            "filename": filename,
            "section": "plugin",
            "subresults": statuses,
            "worst_status": worst,
            "hotfix_available": False,
            "sourcefix_available": False,
        }


def check(
    *,
    id: str,
    title: str,
    rationale: str,
    proposal: Union[str, Sequence[str], None] = None,
    applies_to: str = "TTF",
    runs_on_collection: bool = False,
    metadata: Optional[Dict[str, Any]] = None,
) -> Callable[[Callable[..., CheckStatuses]], Callable[..., CheckStatuses]]:
    """Attach check metadata to a function."""

    if proposal is None:
        proposal_list: List[str] = []
    elif isinstance(proposal, str):
        proposal_list = [proposal]
    else:
        proposal_list = list(proposal)

    def decorator(fn: Callable[..., CheckStatuses]) -> Callable[..., CheckStatuses]:
        fn._fontspector_check = {
            "id": id,
            "title": title,
            "rationale": rationale,
            "proposal": proposal_list,
            "applies_to": applies_to,
            "runs_on_collection": runs_on_collection,
            "metadata": metadata or {},
        }
        return fn

    return decorator


def to_status(item: StatusYield) -> Dict[str, Any]:
    if isinstance(item, str):
        severity = normalize_severity(item)
        return {"severity": severity}

    if isinstance(item, tuple):
        if len(item) == 2:
            severity = normalize_severity(item[0])
            payload = item[1]
            if isinstance(payload, Message):
                status: Dict[str, Any] = {
                    "severity": severity,
                    "code": payload.code,
                    "message": payload.message,
                }
                if payload.metadata:
                    status["metadata"] = payload.metadata
                return status
            if isinstance(payload, str):
                return {"severity": severity, "message": payload}

        if len(item) == 3:
            severity = normalize_severity(item[0])
            code = str(item[1])
            message = str(item[2])
            return {"severity": severity, "code": code, "message": message}

    raise ValueError(
        "Check yielded an invalid status item. Expected STATUS, "
        "(STATUS, message), (STATUS, Message), or (STATUS, code, message)."
    )


def normalize_severity(value: str) -> str:
    sev = str(value).upper()
    if sev not in VALID_SEVERITIES:
        raise ValueError(f"Unknown severity {value!r}")
    return sev


def worst_status(statuses: Sequence[Dict[str, Any]]) -> str:
    order = {SKIP: 0, INFO: 1, PASS: 2, WARN: 3, FAIL: 4, FATAL: 5, ERROR: 6}
    return max(
        (s.get("severity", PASS) for s in statuses),
        key=lambda s: order.get(s, 6),
        default=PASS,
    )


def _build_parser() -> argparse.ArgumentParser:
    parser = argparse.ArgumentParser(prog="fontspector-python-plugin", add_help=True)
    parser.add_argument(
        "--metadata", action="store_true", help="Emit plugin metadata as JSON"
    )
    parser.add_argument("--check", dest="check_id", help="Run a check by id")
    parser.add_argument("files", nargs="*", help="Input files")
    return parser


def _parse_cli(argv: Sequence[str]) -> Tuple[str, Optional[str], List[str]]:
    if len(argv) >= 2 and argv[1] == "metadata":
        return ("metadata", None, [])

    if len(argv) >= 2 and argv[1] == "check":
        if len(argv) < 4:
            raise ValueError("Usage: check <CHECK_ID> <FILE> [<FILE> ...]")
        return ("check", argv[2], list(argv[3:]))

    parser = _build_parser()
    args = parser.parse_args(argv[1:])
    if args.metadata:
        return ("metadata", None, [])
    if args.check_id:
        return ("check", args.check_id, list(args.files))
    raise ValueError(
        "No plugin command provided. Use --metadata or --check <CHECK_ID> <FILE>... ."
    )


def plugin_main(
    register: Callable[[Plugin], None], plugin_name: str = "python-plugin"
) -> int:
    """Run a subprocess fontspector plugin main loop."""

    plugin = Plugin(plugin_name=plugin_name)
    register(plugin)

    try:
        mode, check_id, files = _parse_cli(sys.argv)
    except ValueError as e:
        print(str(e), file=sys.stderr)
        return 2

    if mode == "metadata":
        print(json.dumps(plugin.metadata_payload(), indent=2))
        return 0

    if not files:
        print("--check requires one or more file paths", file=sys.stderr)
        return 2

    try:
        result = plugin.run_check(check_id, files)
    except Exception as e:  # pragma: no cover - demo plugin boundary
        print(f"Error: {e}", file=sys.stderr)
        return 1

    print(json.dumps(result, indent=2))
    return 0


__all__ = [
    "SKIP",
    "INFO",
    "PASS",
    "WARN",
    "FAIL",
    "FATAL",
    "ERROR",
    "Message",
    "ProfileDefinition",
    "check",
    "plugin_main",
    "Plugin",
    "CheckContext",
]
