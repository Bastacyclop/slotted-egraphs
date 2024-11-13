RUSTFLAGS=-Awarnings cargo build --release --features trace
./target/release/slotted-sdql $1