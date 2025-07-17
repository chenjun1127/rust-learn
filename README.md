## Rust-Learn

```rust
cargo run --bin xx # 运行xx，不用带.rs
```

## 创建库
```rust
cargo new rary --lib # 创建一个库
```
根目录 Cargo.toml：

```
[workspace]
members = ["rary", "."]

[dependencies]
rary = { path = "rary" }
```