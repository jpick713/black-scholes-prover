## Available Scripts

In the root directory, you can run:

### ```
cd program
cargo prove build
```

this builds the elf file

then run:
###```
cd ../scripts
RUST_LOG=info RUSTFLAGS='-C target-cpu=native' cargo run --release
```
