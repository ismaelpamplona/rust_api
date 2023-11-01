Install cargo-watch
```sh
cargo install cargo-watch
```

Run the server:
```sh
cargo watch -q -c -w src/ -x run
```
- `-q`: quiet
- `-c`: clear
- `-w`: watch only the `src/` folder
- `-x`: execute the following command