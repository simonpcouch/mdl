default:
    echo 'Hello, world!'

document:
    R -e 'rextendr::document()'

fmt:
    cargo fmt --manifest-path src/rust/Cargo.toml

doc:
    cargo doc --document-private-items --open --manifest-path src/rust/Cargo.toml

check:
    cargo check --manifest-path src/rust/Cargo.toml

update:
    cargo update --manifest-path src/rust/Cargo.toml

clean:
    cargo clean --manifest-path src/rust/Cargo.toml
