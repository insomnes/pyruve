# This will help with new terminal instances
eval "$(pyruve)"
# This will help with cd
_pyruve_on_cd() {
  eval "$(pyruve)"
}
autoload -U add-zsh-hook
add-zsh-hook chpwd _pyruve_on_cd
