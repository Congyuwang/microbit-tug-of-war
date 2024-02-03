build:
    cargo clippy --target thumbv7em-none-eabihf --release
    cargo build --target thumbv7em-none-eabihf --release

flash:
    cargo embed --target thumbv7em-none-eabihf --release

size:
    cargo size --target thumbv7em-none-eabihf --release -- -A

clean:
    cargo clean
