# RUSTFLAGS='-C target-cpu=native' cargo build
rustc --target=x86_64-unknown-linux-gnu test.rs -o main_x86

