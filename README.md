# wasm-webrtc-demo

Start server
```sh
cargo watch -cx "run --bin server --features bevy/dynamic"
```

Start client
```sh
cargo watch -cx "run --bin client --target wasm32-unknown-unknown"
```
