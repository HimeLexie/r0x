#!/usr/bin/env bash
build_linux() {
    echo "Building for Linux..."
    cargo build -r -j $(nproc) --target="x86_64-unknown-linux-gnu"
    if [[ $? != 0 ]]; then
        echo "Build for Linux failed, possibly check build instructions in README.md"
        echo "Exiting."
        return
    fi
    mv target/x86_64-unknown-linux-gnu/release/rox target/x86_64-unknown-linux-gnu/release/rox.x86_64
}
build_windows() {
    echo "Building for Windows..."
    cargo build -r -j $(nproc) --target="x86_64-pc-windows-msvc"
    if [[ $? != 0 ]]; then
        echo "Build for Windows failed, possibly check build instructions in README.md"
        echo "Exiting."
        return
    fi
    mv target/x86_64-pc-windows-msvc/release/rox.exe target/x86_64-pc-windows-msvc/release/rox-x86_64.exe
}
print_help() {
    printf "Usage: build.sh [OPTION] [TARGETS]\nBuilds an r0x executable.\n\nOptions:\n  -c|--clean  clean build\n  -h|--help   display this help message\n\nValid targets:\n  windows\n  linux\nTargets should be space seperated if multiple are to be specified.\n\nExample:\nbuild.sh -c windows linux"
}

for arg in "$@"; do
    case $arg in
        -c|--clean)
        cargo clean
        shift
        ;;
        linux)
        build_linux
        shift
        ;;
        windows)
        build_windows
        shift
        ;;
        *)
        echo "Unknown option: $arg"
        print_help
        ;;
        -h|--help)
        print_help
        ;;
    esac
done
if [ -z $@ ]; then
    print_help
fi
