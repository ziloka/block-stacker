host=$(rustc -Vv | grep -oP "(?<=host: )[\w_-]+")

rustup toolchain install nightly
rustup component add rust-src --toolchain nightly
cargo +nightly build -Z build-std=std,panic_abort --target $host --release