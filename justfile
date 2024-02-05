build:
    cargo clippy --target thumbv7em-none-eabihf --release
    cd tug-of-war && cargo build --target thumbv7em-none-eabihf --release

flash:
    cd tug-of-war && cargo embed --target thumbv7em-none-eabihf --release

debug:
    cd tug-of-war && cargo embed --target thumbv7em-none-eabihf

size:
    cd tug-of-war && cargo size --target thumbv7em-none-eabihf --release -- -A

test:
    cargo test --release

clean:
    cargo clean
