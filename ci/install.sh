set -euxo pipefail

main() {
    if [ "$TARGET" != "x86_64-unknown-linux-gnu" ]; then
        echo "Installing $TARGET"
        rustup target add $TARGET
    fi
}

main
