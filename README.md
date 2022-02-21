# wasm-webrtc-demo

Start server
```sh
cargo watch -cx "run --bin server --features bevy/dynamic,use-webrtc"
```

Start client
```sh
cargo watch -cx "run --bin client --target wasm32-unknown-unknown --features wbindgen"
```
