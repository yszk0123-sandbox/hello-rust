[Babel プラグインを Rust (SWC) に移植して、JavaScript のコンパイルを爆速にする 〜プラグイン作成編〜](https://www.wantedly.com/companies/wantedly/post_articles/386347)

```shell
rustup target add wasm32-wasi wasm32-unknown-unknown
cargo install swc_cli
swc plugin new hello_world --target-type wasm32-wasi
cargo build --release --target wasm32-wasi

cp hello_world_plugin/target/wasm32-wasi/release/hello_world.wasm hello_world/tmp/
```
