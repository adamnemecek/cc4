# cc4 
Implementation of four_cc in Rust as a `const fn`. Supports both little and big endian systems.

```rust
four_cc(b"code");
```

Since `four_cc` is a `const fn`, you can also use it to define `enum` values for `enums` represented by `u32`.

```rust
#[repr(u32)]
pub enum Id {
    Apple = four_cc(b"appl"),
}
```