# type_enum

A Rust macro to generate zero-sized types with a common trait for Typestate
APIs.

## Example

```rust
use type_enum::type_enum;

type_enum! {
    pub State { Ready, Working, Complete }
}
```
