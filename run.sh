#!/usr/bin/env bash
# run.sh - Development helper script for the Rust minigrep project

# exit on error, unset vars, pipe failures
set -euo pipefail

# define project root
PROJECT_ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
cd "$PROJECT_ROOT"

# usage instructions for the script
show_help() {
    cat << EOF
Usage: ./run.sh [OPTIONS] [-- <extra cargo arguments>]

A convenient wrapper around common cargo commands for the minigrep project.

OPTIONS:
    --help              Show this help message and exit
    --check             Check compilation without producing artifacts (cargo check)
    --build             Build in debug mode (cargo build)
    --release           Build in release mode (cargo build --release)
    --run   [args...]   Build and run the binary with optional arguments
                        Example: ./run.sh --run needle haystack.txt
    --shell             Start the Rust REPL (evcxr)
    --test              Run unit tests from the library crate
    --update            Update dependencies (cargo update)

Any arguments after -- are passed directly to the binary when using --run
or to cargo for other commands when it makes sense.

Examples:
    ./run.sh --run needle myfile.txt
    ./run.sh --run -- --ignore-case needle file.txt
    ./run.sh --clippy -- -D warnings

EOF
}

# default exit code
EXIT_CODE=0

# parse command line arguments
# shellcheck disable=SC1009
while [[ $# -gt 0 ]]; do
    case $1 in
        --help)
            show_help
            exit 0
        ;;

        --check)
            echo "To be implemented: checking syntax without building..."
        ;;

        --build)
            echo "To be implemented: building the application..."
        ;;

        --release)
            echo "To be implemented: building in release mode..."
        ;;

        --run)
            shift
            echo -e "Building and running minigrep..."
            cargo run --bin minigrep -- "$@"
            EXIT_CODE=$?
            exit $EXIT_CODE
        ;;

        --shell)
            echo -e "Starting evcxr (Rust REPL)..."
            command -v evcxr >/dev/null || { echo "evcxr not found. Install with: cargo install evcxr_repl"; exit 1; }
            evcxr
        ;;

        --test)
            echo "Running unit tests from the library crate..."
            cargo test
        ;;

        --update)
            echo "To be implemented: updating dependencies..."
        ;;

        *)
            echo "Unknown option: $1"
            show_help
            exit 1
        ;;
    esac
    shift
done

# show help if no arguments were given
if [[ $EXIT_CODE -eq 0 && ! -t 0 ]]; then
    echo "No command specified."
    show_help
    exit 1
fi

# exit with the last exit code
exit $EXIT_CODE
