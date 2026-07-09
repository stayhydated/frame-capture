set windows-shell := ["pwsh.exe", "-NoLogo", "-Command"]

default:
    @just --list

fmt:
    cargo sort-derives
    cargo fmt
    taplo fmt
    rumdl fmt .

clippy:
    cargo clippy --workspace --all-features --all-targets --locked -- -D warnings

check:
    cargo check --workspace --all-features

test:
    cargo test --workspace --all-features --all-targets --locked

cov:
    cargo llvm-cov --workspace --all-features --all-targets

test-publish:
    cargo publish --workspace --dry-run --allow-dirty

test-docs:
    cargo doc --workspace --all-features --no-deps --open

example-captures:
    FRAME_CAPTURE_ROUTE=bevy/dashboard FRAME_CAPTURE_PATH=examples/bevy/captures/bevy/dashboard/current.png cargo run -p frame-capture-example-bevy
    FRAME_CAPTURE_ROUTE=bevy/detail FRAME_CAPTURE_PATH=examples/bevy/captures/bevy/detail/current.png cargo run -p frame-capture-example-bevy
    FRAME_CAPTURE_ROUTE=gpui/dashboard FRAME_CAPTURE_PATH=examples/gpui/captures/gpui/dashboard/current.png cargo run --manifest-path examples/gpui/Cargo.toml
    FRAME_CAPTURE_ROUTE=gpui/review FRAME_CAPTURE_PATH=examples/gpui/captures/gpui/review/current.png cargo run --manifest-path examples/gpui/Cargo.toml

example-gpui:
    cargo run --manifest-path examples/gpui/Cargo.toml

# Uses the headless renderer from the forked Zed/GPUI linux-headless-renderer branch.
example-gpui-captures:
    FRAME_CAPTURE_ROUTE=gpui/dashboard FRAME_CAPTURE_PATH=examples/gpui/captures/gpui/dashboard/current.png cargo run --manifest-path examples/gpui/Cargo.toml
    FRAME_CAPTURE_ROUTE=gpui/review FRAME_CAPTURE_PATH=examples/gpui/captures/gpui/review/current.png cargo run --manifest-path examples/gpui/Cargo.toml

ci: fmt check clippy test cov
    cargo machete
