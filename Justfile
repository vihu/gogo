# Define the default settings.toml file path
default-settings := "settings.toml"

default:
    just --list --unsorted

# Build the entire project
build:
    cargo build --release

# Run clippy
clippy:
    cargo clippy -- -Dclippy::all -D warnings
