# pyruve

`pyruve` automatically activates the nearest Python virtual environment when you change directories.
It deactivates the environment after you leave its project tree.

Supported shells on Unix-like systems:

- Bash
- Fish
- Zsh

Windows shells are not supported.

## Install

Install the current release from crates.io:

```console
cargo install pyruve
```

Ensure that Cargo's binary directory is in `PATH`.
It is normally `$HOME/.cargo/bin`.

Then add one initializer to your shell configuration.

### Bash

Add this line to `.bashrc`:

```bash
eval "$(pyruve shell bash)"
```

### Fish

Add this line to `~/.config/fish/config.fish`:

```fish
pyruve shell fish | source
```

### Zsh

Add this line to `.zshrc`:

```zsh
eval "$(pyruve shell zsh)"
```

Restart the shell after installation or upgrade.

## Behavior

On each directory change, `pyruve` searches the current directory and its ancestors.
It uses the nearest matching virtual environment.

By default, these directories match:

- `venv`
- `.venv`
- `<project>-venv`
- `<project>_venv`
- `<project>-.venv`
- `<project>_.venv`

Nested projects can have independent environments.
Moving from an outer project into a nested project activates the nested environment.

Paths containing spaces or apostrophes are supported.
Activation paths containing newline characters are rejected.

## Configuration

`PYRUVE_VENV_DIRS` contains a comma-separated list of base directory names:

```console
export PYRUVE_VENV_DIRS='venv,.venv,virtenv,.virtenv'
```

`PYRUVE_DELIMITERS` contains delimiters for combined names:

```console
export PYRUVE_DELIMITERS='-,_'
```

`PYRUVE_COMBINE_DIRS` controls combined names such as `project-venv`.
The values `true`, `t`, `1`, and `on` enable this behavior.
Any other value disables it.

## Security

`pyruve` sources the activation script found in a matching environment directory.
Activation scripts are executable shell code.

Use `pyruve` only in project trees that you trust.
The tool verifies the script path, but it does not inspect or sandbox script contents.

## Compatibility

The repository tests Bash, Fish, and Zsh with real Python environments.
CI builds the crate with stable Rust; no minimum supported Rust version is promised.

Other shells need explicit integrations because environment mutation is shell-specific.
Tcsh could use the existing action protocol with a small adapter.
Nushell and PowerShell need structured environment updates, so they are not syntax-only additions.

## Development

Run the Rust checks locally:

```console
cargo fmt --all -- --check
cargo test --locked
cargo clippy --all-targets --locked -- -D warnings
```

The shell integration test also requires Bash, Fish, Zsh, and `python3-venv`:

```console
cargo build --locked
tests/shell_integration.sh target/debug/pyruve
```
