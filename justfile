default:
    just --list

[group('lint')]
clippy-all:
    cargo clippy --profile dev --all-features --all-targets --workspace

[group('build')]
check-all:
    cargo check --all-features --all-targets --workspace

[group('test')]
test-all:
    cargo test --all-features --all-targets --workspace

[group('format')]
format-all:
    cargo fmt --verbose --all

[group('lint')]
clippy-dis:
    cargo clippy --profile dev --all-features --all-targets --package dis-rs

[group('build')]
check-dis:
    cargo check --all-features --all-targets --package dis-rs

[group('test')]
test-dis:
    cargo test --all-features --all-targets --package dis-rs

[group('release')]
publish-dis:
    cargo publish --all-features --package dis-rs

[group('release')]
publish-dry-dis:
    cargo publish --all-features --dry-run --package dis-rs

[group('lint')]
clippy-cdis:
    cargo clippy --profile dev --all-features --all-targets --package cdis-assemble

[group('build')]
check-cdis:
    cargo check --all-features --all-targets --package cdis-assemble

[group('test')]
test-cdis:
    cargo test --all-features --all-targets --package cdis-assemble

[group('build')]
build-debug-cdis-gateway:
    cargo build --package cdis-gateway --bin cdis-gateway

[group('build')]
build-release-cdis-gateway:
    cargo build --package cdis-gateway --bin cdis-gateway --release

[group('run')]
run-cdis-gateway-localhost:
    cargo run --package cdis-gateway --bin cdis-gateway -vv -- ./cdis-gateway/config/localhost_config.toml

[group('lint')]
clippy-gateway-core:
    cargo clippy --profile dev --all-features --all-targets --package gateway-core

[group('build')]
check-gateway-core:
    cargo check --all-features --all-targets --package gateway-core

[group('test')]
test-gateway-core:
    cargo test --all-features --all-targets --package gateway-core

[group('install')]
install-tools-coverage:
    cargo install cargo-tarpaulin

[group('install')]
install-tools-cdis-gateway:
    @echo 'todo'
    # bun
    # tailwindcss
    cd cdis-gateway
    bun install
