build:
  cargo build --release

start:
  cargo run

checksum $file:
  cargo run --bin checksum $file

add_cover $file:
  cargo run --bin add_cover $file
