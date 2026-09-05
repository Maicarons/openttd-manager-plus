#!/usr/bin/env bash
# Release build script for OpenTTD Manager Plus
# Usage: ./scripts/release.sh [desktop|android|ios|web|all]

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
PROJECT_DIR="$(dirname "$SCRIPT_DIR")"
cd "$PROJECT_DIR"

echo "=== OpenTTD Manager Plus Release Build ==="
echo "Version: $(cargo metadata --format-version 1 --no-deps | python3 -c 'import sys,json; print(json.load(sys.stdin)["packages"][0]["version"])')"
echo ""

build_desktop() {
    echo "--- Building Desktop ---"
    cargo build --release -p otmp-desktop
    echo "Desktop build complete."
    echo "Binary: target/release/openttd-manager-plus"
    echo ""
}

bundle_desktop() {
    echo "--- Bundling Desktop ---"
    if command -v dx &> /dev/null; then
        dx bundle --platform desktop --release
        echo "Desktop bundle complete."
    else
        echo "Dioxus CLI (dx) not found. Install with: curl -fsSL https://dioxuslabs.com/install.sh | bash"
        build_desktop
    fi
    echo ""
}

build_android() {
    echo "--- Building Android ---"
    if command -v dx &> /dev/null; then
        dx build --platform android --release
        echo "Android build complete."
    else
        echo "Dioxus CLI (dx) required for Android builds."
        exit 1
    fi
    echo ""
}

build_ios() {
    echo "--- Building iOS ---"
    if command -v dx &> /dev/null; then
        dx build --platform ios --release
        echo "iOS build complete."
    else
        echo "Dioxus CLI (dx) required for iOS builds."
        exit 1
    fi
    echo ""
}

build_web() {
    echo "--- Building Web ---"
    if command -v dx &> /dev/null; then
        dx build --platform web --release
        echo "Web build complete."
    else
        echo "Dioxus CLI (dx) required for Web builds."
        exit 1
    fi
    echo ""
}

run_tests() {
    echo "--- Running Tests ---"
    cargo test --workspace
    echo "All tests passed."
    echo ""
}

case "${1:-all}" in
    desktop)
        bundle_desktop
        ;;
    android)
        build_android
        ;;
    ios)
        build_ios
        ;;
    web)
        build_web
        ;;
    all)
        run_tests
        bundle_desktop
        echo "=== All builds complete ==="
        ;;
    test)
        run_tests
        ;;
    *)
        echo "Usage: $0 [desktop|android|ios|web|all|test]"
        exit 1
        ;;
esac