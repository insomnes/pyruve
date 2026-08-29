_pyruve_apply() {
    local activation_script pyruve_status

    if activation_script="$(command pyruve hook zsh)"; then
        source "$activation_script"
        return
    else
        pyruve_status=$?
    fi

    case $pyruve_status in
        10)
            if (( $+functions[deactivate] )); then
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

_pyruve_on_cd() {
    _pyruve_apply
}

_pyruve_apply
autoload -U add-zsh-hook
add-zsh-hook -d chpwd _pyruve_on_cd 2>/dev/null
add-zsh-hook chpwd _pyruve_on_cd
