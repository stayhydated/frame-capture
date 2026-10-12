set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

default:
    @just --list

fmt:
    cargo sort-derives
    cargo fmt
    cargo fmt --manifest-path examples/gpui/Cargo.toml
    taplo fmt
    rumdl fmt .

fmt-check:
    cargo fmt --all -- --check
    cargo fmt --manifest-path examples/gpui/Cargo.toml --all -- --check
    taplo fmt --check
    rumdl check .
    cargo sort-derives --check
    typos

clippy:
    cargo clippy --workspace --all-features --all-targets --locked -- -D warnings

check:
    cargo check --workspace --all-features --all-targets --locked

test:
    cargo test --workspace --all-features --all-targets --locked
    cargo test --workspace --all-features --doc --locked

cov:
    cargo llvm-cov clean --workspace
    cargo llvm-cov --workspace --all-features --all-targets --no-report --exclude xtask --exclude web
    FRAME_CAPTURE_ROUTE=bevy/dashboard FRAME_CAPTURE_PATH=target/coverage-dashboard.png FRAME_CAPTURE_SCENARIO=alert cargo llvm-cov --no-report run -p frame-capture-example-bevy
    FRAME_CAPTURE_ROUTE=bevy/detail FRAME_CAPTURE_PATH=target/coverage-detail.png FRAME_CAPTURE_FRAME=2 FRAME_CAPTURE_WIDTH=640 FRAME_CAPTURE_HEIGHT=360 cargo llvm-cov --no-report run -p frame-capture-example-bevy
    cargo llvm-cov report

check-gpui:
    cargo check --manifest-path examples/gpui/Cargo.toml --all-targets --locked

test-publish:
    cargo publish --workspace --dry-run --locked --allow-dirty

test-docs:
    cargo doc --workspace --all-features --no-deps --locked --open

example-captures:
    FRAME_CAPTURE_ROUTE=bevy/dashboard FRAME_CAPTURE_PATH=examples/bevy/captures/bevy/dashboard/current.png cargo run -p frame-capture-example-bevy
    FRAME_CAPTURE_ROUTE=bevy/detail FRAME_CAPTURE_PATH=examples/bevy/captures/bevy/detail/current.png cargo run -p frame-capture-example-bevy
    FRAME_CAPTURE_ROUTE=gpui/dashboard FRAME_CAPTURE_PATH=examples/gpui/captures/gpui/dashboard/current.png cargo run --manifest-path examples/gpui/Cargo.toml
    FRAME_CAPTURE_ROUTE=gpui/review FRAME_CAPTURE_PATH=examples/gpui/captures/gpui/review/current.png cargo run --manifest-path examples/gpui/Cargo.toml

example-gpui:
    cargo run --manifest-path examples/gpui/Cargo.toml

# Native capture is gated by the `capture` feature (on by default) and runs on
# the platform headless renderer: Metal on macOS, Wgpu on Linux.
example-gpui-captures:
    FRAME_CAPTURE_ROUTE=gpui/dashboard FRAME_CAPTURE_PATH=examples/gpui/captures/gpui/dashboard/current.png cargo run --manifest-path examples/gpui/Cargo.toml
    FRAME_CAPTURE_ROUTE=gpui/review FRAME_CAPTURE_PATH=examples/gpui/captures/gpui/review/current.png cargo run --manifest-path examples/gpui/Cargo.toml

ci: fmt check clippy test cov
    cargo machete

book:
    mdbook serve book

web-build:
    cargo xtask build book
    cargo xtask build llms-txt
    cargo xtask build web

web: web-build
    dx serve --package web

web-preview: web-build
    cargo xtask preview web
