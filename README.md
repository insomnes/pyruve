# pyruve
Tool to automatically activate your python virtual environment inside project folder.

Supported shells:
- `bash`
- `fish`
- `zsh`

# Install
You will need [cargo](https://www.rust-lang.org/tools/install):
```
cargo install pyruve
```
Add `$HOME/.cargo/bin` to your path.

## bash
Add to your `.bashrc` (commonly in `$HOME/.bashrc`):
```
eval "$(pyruve shell bash)"
```

## fish
Add to your `config.fish` (commonly in `$HOME/.config/fish/config.fish`):
```
pyruve shell fish | source
```

## zsh
Add to your `.zshrc` (commonly in `$HOME/.zshrc`):
```
eval "$(pyruve shell zsh)"
```
# How it works?
Binary will return specific command on new instances of shell or change directory
based on current venv status.

## No venv is active
If you have any of: `venv`,`.venv` subdirs with available 
`bin/activate` script => it will be activated.
Dir list is configurable by `PYRUVE_VENV_DIRS` env var,
list should be delimited by commas like:
```
venv,.venv,virtenv,.virtenv
```
It will also try to search combinations of `projectdir-{iter of venv dirs}` or 
`projectdir_{iter of venv dirs}` like `pyruve-venv` or `pyruve_virtenv`.
This can be disabled via `PYRUVE_COMBINE_DIRS` env var with any value except:
```
"true" | "t" | "1" | "on"
```

Delimiters are configured via `PYRUVE_DELIMITERS` env var,
list should be delimited by commas like:
```
-,_
```

## Venv is active
Going to any child of directory which contains active virtual environment dir will not
lead to deactivation. Only parent or completely another dir in tree
will cause deactivation.
Going to dir with new virtual environment available will also cause 
activation of new virtual environment.
