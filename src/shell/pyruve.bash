# This will help with new terminal instances
eval "$(pyruve)"

# This will help with cd
_pyruve_hook_on_cd() {
    eval "$(pyruve)"
}

case $PROMPT_COMMAND in
    *_pyruve_hook_on_cd*)
        ;;
    *)
        PROMPT_COMMAND="${PROMPT_COMMAND:+$(echo "${PROMPT_COMMAND}" | awk '{gsub(/; *$/,"")}1') ; }_pyruve_hook_on_cd"
        ;;
esac
