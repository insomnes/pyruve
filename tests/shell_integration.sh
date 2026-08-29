#!/usr/bin/env bash
set -euo pipefail

binary=${1:-target/debug/pyruve}
binary_dir=$(cd "$(dirname "$binary")" && pwd)
test_root=$(mktemp -d "${TMPDIR:-/tmp}/pyruve-shell-test.XXXXXX")
project="$test_root/project's space"
nested_project="$project/nested"
outside="$test_root/outside"

mkdir -p "$project/src" "$nested_project/src" "$outside"
python3 -m venv "$project/.venv"
python3 -m venv "$nested_project/.venv"

export PATH="$binary_dir:$PATH"
export PYRUVE_TEST_PROJECT="$project"
export PYRUVE_TEST_NESTED_PROJECT="$nested_project"
export PYRUVE_TEST_OUTSIDE="$outside"

# These programs, not this script, expand the single-quoted command bodies.
# shellcheck disable=SC2016
env -u VIRTUAL_ENV bash --noprofile --norc -c '
set -euo pipefail
eval "$(pyruve shell bash)"

cd "$PYRUVE_TEST_PROJECT/src"
_pyruve_apply
test "$VIRTUAL_ENV" = "$PYRUVE_TEST_PROJECT/.venv"

cd "$PYRUVE_TEST_NESTED_PROJECT/src"
_pyruve_apply
test "$VIRTUAL_ENV" = "$PYRUVE_TEST_NESTED_PROJECT/.venv"

cd "$PYRUVE_TEST_OUTSIDE"
_pyruve_apply
test -z "${VIRTUAL_ENV-}"
'

# shellcheck disable=SC2016
env -u VIRTUAL_ENV bash --noprofile --norc -c '
set -euo pipefail
PROMPT_COMMAND="existing_command"
eval "$(pyruve shell bash)"
test "$PROMPT_COMMAND" = "existing_command; _pyruve_hook_on_prompt"
eval "$(pyruve shell bash)"
test "$PROMPT_COMMAND" = "existing_command; _pyruve_hook_on_prompt"

PATH=/nonexistent
_pyruve_hook_on_prompt
cd "$PYRUVE_TEST_OUTSIDE"
if _pyruve_hook_on_prompt 2>/dev/null; then
    exit 1
else
    test $? -eq 127
fi
'

# shellcheck disable=SC2016
env -u VIRTUAL_ENV bash --noprofile --norc -c '
set -euo pipefail
declare -a PROMPT_COMMAND=(existing_command)
eval "$(pyruve shell bash)"
test "${PROMPT_COMMAND[0]}" = existing_command
test "${PROMPT_COMMAND[1]}" = _pyruve_hook_on_prompt
test "${#PROMPT_COMMAND[@]}" -eq 2
'

# shellcheck disable=SC2016
env -u VIRTUAL_ENV fish -c '
pyruve shell fish | source

cd "$PYRUVE_TEST_PROJECT/src"
_pyruve_apply
test "$VIRTUAL_ENV" = "$PYRUVE_TEST_PROJECT/.venv"

cd "$PYRUVE_TEST_NESTED_PROJECT/src"
_pyruve_apply
test "$VIRTUAL_ENV" = "$PYRUVE_TEST_NESTED_PROJECT/.venv"

cd "$PYRUVE_TEST_OUTSIDE"
_pyruve_apply
test -z "$VIRTUAL_ENV"
'

# shellcheck disable=SC2016
env -u VIRTUAL_ENV tcsh -f -c '
set prompt = "% "
eval "`pyruve shell tcsh`"
_pyruve_apply

cd "$PYRUVE_TEST_PROJECT/src"
if ( "$VIRTUAL_ENV" != "$PYRUVE_TEST_PROJECT/.venv" ) exit 1

cd "$PYRUVE_TEST_NESTED_PROJECT/src"
if ( "$VIRTUAL_ENV" != "$PYRUVE_TEST_NESTED_PROJECT/.venv" ) exit 1

cd "$PYRUVE_TEST_OUTSIDE"
if ( $?VIRTUAL_ENV ) exit 1
'

# shellcheck disable=SC2016
env -u VIRTUAL_ENV tcsh -f -c '
set pyruve_test_cwdcmd_calls = 0
alias cwdcmd '@ pyruve_test_cwdcmd_calls++'
eval "`pyruve shell tcsh`"
eval "`pyruve shell tcsh`"

cd "$PYRUVE_TEST_OUTSIDE"
if ( $pyruve_test_cwdcmd_calls != 1 ) exit 1
'

# shellcheck disable=SC2016
env -u VIRTUAL_ENV zsh --no-rcs -c '
set -e
eval "$(pyruve shell zsh)"

cd "$PYRUVE_TEST_PROJECT/src"
_pyruve_apply
test "$VIRTUAL_ENV" = "$PYRUVE_TEST_PROJECT/.venv"

cd "$PYRUVE_TEST_NESTED_PROJECT/src"
_pyruve_apply
test "$VIRTUAL_ENV" = "$PYRUVE_TEST_NESTED_PROJECT/.venv"

cd "$PYRUVE_TEST_OUTSIDE"
_pyruve_apply
test -z "${VIRTUAL_ENV-}"
'

printf 'shell integration checks passed; temporary data: %s\n' "$test_root"
