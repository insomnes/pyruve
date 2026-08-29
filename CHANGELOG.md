# Changelog

## 0.2.0 - 2026-08-29

- Source activation scripts through shell-native commands instead of evaluating generated project paths.
- Support paths containing spaces and apostrophes.
- Activate the nearest nested virtual environment.
- Handle non-UTF-8 directory names without panicking on Unix.
- Avoid running the Bash hook again when `PWD` did not change.
- Preserve scalar and array forms of Bash `PROMPT_COMMAND`.
- Add Tcsh support without replacing an existing `cwdcmd` alias.
- Add `--help` and `--version` output.
- Remove the only runtime dependency.

Reload the shell configuration after upgrading from 0.1.0.
