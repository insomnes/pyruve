# PROMPT_COMMAND can be either a scalar or an array.
# shellcheck disable=SC2128,SC2178

_pyruve_apply() {
    local activation_script pyruve_status

    if activation_script="$(command pyruve hook bash)"; then
        # pyruve returns the discovered activation script.
        # shellcheck disable=SC1090
        source "$activation_script"
        return
    else
        pyruve_status=$?
    fi

    case $pyruve_status in
        10)
            if declare -F deactivate >/dev/null; then
                deactivate
            fi
            ;;
        11)
            return 0
            ;;
        *)
            return "$pyruve_status"
            ;;
    esac
}

_pyruve_hook_on_prompt() {
    local pyruve_status

    if [[ ${_PYRUVE_LAST_PWD-} == "$PWD" ]]; then
        return 0
    fi

    _pyruve_apply
    pyruve_status=$?
    if [[ $pyruve_status == 0 ]]; then
        _PYRUVE_LAST_PWD=$PWD
    fi
    return "$pyruve_status"
}

if _pyruve_apply; then
    _PYRUVE_LAST_PWD=$PWD
fi

if [[ $(declare -p PROMPT_COMMAND 2>/dev/null) == "declare -a"* ]]; then
    _pyruve_hook_registered=false
    for _pyruve_prompt_command in "${PROMPT_COMMAND[@]}"; do
        if [[ $_pyruve_prompt_command == _pyruve_hook_on_prompt ]]; then
            _pyruve_hook_registered=true
            break
        fi
    done
    if [[ $_pyruve_hook_registered == false ]]; then
        PROMPT_COMMAND+=(_pyruve_hook_on_prompt)
    fi
    unset _pyruve_hook_registered _pyruve_prompt_command
else
    case ";${PROMPT_COMMAND-};" in
        *";_pyruve_hook_on_prompt;"*) ;;
        *) PROMPT_COMMAND="${PROMPT_COMMAND:+${PROMPT_COMMAND}; }_pyruve_hook_on_prompt" ;;
    esac
fi
