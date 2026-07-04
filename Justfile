_default:
    cargo just --list -u

init:
    cargo tool --install

lint: lint-check

lint-check:
    cargo clippy --no-deps --all-targets --all-features -- -D warnings

lint-fix:
    cargo clippy --no-deps --all-targets --all-features --fix

format: format-fix

format-check:
    cargo fmt --all -- --check

format-fix:
    cargo fmt --all

fix:
    cargo just format-fix
    cargo just lint-fix

check:
    cargo just format
    cargo just lint

doc:
    cargo doc --all-features --no-deps --open --lib

semver-checks:
    # No --baseline-version pin: a hardcoded prerelease baseline goes stale on every alpha bump
    # and, worse, can silently re-resolve a loose dependency requirement in the baseline against a
    # newer breaking prerelease (see grixy's alpha.8 CHANGELOG). Letting cargo-semver-checks
    # auto-select the baseline compares against the latest published release instead.
    cargo tool cargo-semver-checks

msrv:
    cargo tool cargo-hack check --rust-version --workspace --all-targets --ignore-private

doc-gen:
    cargo clean --doc
    cargo doc --no-deps
    echo '<meta http-equiv="refresh" content="0;url=ixy/index.html">' > target/doc/index.html
    rm target/doc/.lock

test *ARGS:
    cargo tool cargo-nextest run {{ARGS}}

test-doc *ARGS:
    cargo test {{ARGS}} --doc

test-all:
    cargo just test --all-features
    cargo just test-doc --all-features
    
coverage *ARGS:
    cargo tool cargo-llvm-cov --lib --open

coverage-gen:
    cargo tool cargo-llvm-cov --lib --lcov --output-path lcov.info
