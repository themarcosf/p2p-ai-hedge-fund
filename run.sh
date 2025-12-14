#!/usr/bin/env bash
# run.sh - Development helper script for the Rust minigrep project

# usage instructions for the script
show_help() {
    cat << EOF
Usage: ./run.sh [OPTIONS]

A convenient wrapper around common cargo commands for the minigrep project.

OPTIONS:
    --help              Show this help message and exit
    --build             Build in debug mode
    --check             Check compilation without producing artifacts
    --doc               Generate documentation
                            :: click project name (minigrep) in the browser
    --release           Build in release mode
    --run [args...].    Build and run the binary with optional arguments
                            :: ./run.sh --run to poem.txt
                            :: IGNORE_CASE=True ./run.sh --run to poem.txt
    --shell             Start the Rust REPL
    --test              Run unit tests from the library crate
    --update            Update dependencies
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

        --build)
            echo "Building the application in debug mode..."
            cargo build
        ;;

        --check)
            echo "To be implemented: checking syntax without building..."
        ;;

        --doc)
            echo "Generating documentation..."
            cargo doc --no-deps
            python3 -m http.server 8000 --bind 0.0.0.0 -d target/doc
        ;;

        --release)
            echo "Building the application in release mode..."
            cargo build --release
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
