dir-dis-lib := "dis-rs"
dir-cdis-assemble := "cdis-assemble"
dir-cdis-gateway := "cdis-gateway"
dir-cdis-gateway-web := "templates"
dir-cdis-gateway-web-dist := "assets"

dummy:
    echo 'There is no default recipe for the dis-rs workspace.'

workspace-test:
    cargo test

workspace-clippy:
    cargo clippy

dis-build:
    cd {{dir-dis-lib}} && cargo build

dis-test:
    cd {{dir-dis-lib}} && cargo test

dis-release:
    cd {{dir-dis-lib}} && cargo build --release

dis-publish:
    cd {{dir-dis-lib}} && cargo publish

cdis-assemble-build:
    cd {{dir-cdis-assemble}} && cargo build

cdis-assemble-test:
    cd {{dir-cdis-assemble}} && cargo test

cdis-gateway-init:
    cargo install cargo-watch

cdis-gateway-build-tailwind:
    cd {{dir-cdis-gateway}} && tailwindcss -i {{dir-cdis-gateway-web}}/input.css -o {{dir-cdis-gateway-web-dist}}/styles.css --minify

cdis-gateway-dev-tailwind:
    cd {{dir-cdis-gateway}} && tailwindcss -i {{dir-cdis-gateway-web}}/input.css -o {{dir-cdis-gateway-web-dist}}/styles.css --watch=always

cdis-gateway-dev-gateway:
    cd {{dir-cdis-gateway}} && cargo watch -w src -w {{dir-cdis-gateway-web}} -w tailwind.config.js -w {{dir-cdis-gateway-web}}/input.css -x 'run ./config/localhost_config.toml'

cdis-gateway-dev:
    #!/bin/sh
    just cdis-gateway-dev-tailwind &
    pid1=$!
    just cdis-gateway-dev-gateway
    pid2=$!
    trap "kill $pid1 $pid2" EXIT
    wait $pid1 $pid2

