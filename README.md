# extrum

**extrum** — a Rust macro that defines transparent wrapper types with enum‑like named constants, conversion methods, debug formatting, and cross‑type equality with the underlying integer type.

The generated type is a `#[repr(transparent)]` struct around an integer base type. It provides:

+ `pub const` variants (like enum arms)
+ `from_raw` / `into_raw` conversions
+ A `name(&self) -> &'static str` method (returns `"Unknown"` for unknown values)
+ `From` impls for both directions
+ `Debug` that prints known variant names (or the raw value)
+ `Eq` and cross‑type `PartialEq` with the base type

Unlike a real `enum`, the wrapper can hold **any** value of the underlying type, not only the defined constants. This is useful for C interop, bitflags, or handling reserved / unknown values.

## Example

```rust
use extrum::extrum;

extrum! {
    #[derive(PartialEq, Clone, Copy)]
    pub enum HttpStatus: u16 {
        OK = 200,
        NOT_FOUND = 404,
    }
}

let ok = HttpStatus::OK;
assert_eq!(ok.into_raw(), 200);
assert_eq!(HttpStatus::from_raw(404), HttpStatus::NOT_FOUND);
assert_eq!(format!("{:?}", ok), "HttpStatus::OK");
assert_eq!(ok.name(), "OK");

// Works with unknown values too
let unknown = HttpStatus::from_raw(500);
assert_eq!(unknown.name(), "Unknown");
assert_eq!(format!("{:?}", unknown), "HttpStatus(500)");
```

## Syntax

```rust
extrum! {
    // optional attributes for the generated struct
    #[derive(Debug, PartialEq)]
    pub enum Name: BaseType {
        Variant1 = value1,
        Variant2 = value2,
        // ...
    }
}
```

- `BaseType` - any integer type (`u8`, `i32`, `u64`, `usize`, etc.)
- Each `Variant` becomes a `pub const` associated constant.
- The generated struct is named `Name` and is `#[repr(transparent)]` over `BaseType`.

## Generated API

Given the invocation above, the macro expands to roughly:

```rust
#[repr(transparent)]
pub struct Name(BaseType);

impl Name {
    pub const Variant1: Self = Name(value1);
    pub const Variant2: Self = Name(value2);
    // ...

    pub const fn from_raw(value: BaseType) -> Self { Name(value) }
    pub const fn into_raw(self) -> BaseType { self.0 }
    pub fn name(&self) -> &'static str { ... }
}

impl From<BaseType> for Name { ... }
impl From<Name> for BaseType { ... }
impl std::fmt::Debug for Name { ... }
impl Eq for Name {}
impl PartialEq<BaseType> for Name { ... }
impl PartialEq<Name> for BaseType { ... }
```

## When to use this instead of a real `enum`

- You need to store values that are not in the predefined set (e.g., reserved or future enum values from a C library).
- You want a transparent wrapper for interop, but still want named constants for the known cases.
- You want `Debug` to print the variant name when possible, but fall back to the numeric value.

If you **only** need the named constants and the underlying type should **never** hold other values, use a standard Rust `enum` instead.

## License

Licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or http://apache.org/licenses/LICENSE-2.0)
- MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)

at your option.
