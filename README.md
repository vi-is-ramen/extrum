# extrum

**extrum** — a Rust macro that defines transparent wrapper types with enum‑like named constants, conversion methods, debug formatting, and cross‑type equality with the underlying integer type.

The generated type is a `#[repr(transparent)]` struct around an integer base type. It provides:

+ `pub const` variants (like enum arms)
+ `from_raw` / `into_raw` conversions
+ A `name(&self) -> &'static str` method (returns `"Unknown"` for unknown values)
+ `From` impls for both directions
+ `Debug` that prints known variant names (or the raw value)
+ `Eq` and cross‑type `PartialEq` with the base type

Additionally, the crate provides the `implement_display!` macro to add a `Display` implementation that uses the `name()` method.

Unlike a real `enum`, the wrapper can hold **any** value of the underlying type, not only the defined constants. This is useful for C interop, bitflags, or handling reserved / unknown values.

## Example

```rust
use extrum::{extrum, implement_display};

extrum! {
    #[derive(PartialEq, Clone, Copy)]
    pub enum HttpStatus: u16 {
        OK = 200,
        NOT_FOUND = 404,
    }
}

implement_display![HttpStatus];

let ok = HttpStatus::OK;
assert_eq!(ok.into_raw(), 200);
assert_eq!(HttpStatus::from_raw(404), HttpStatus::NOT_FOUND);
assert_eq!(format!("{:?}", ok), "HttpStatus::OK");
assert_eq!(ok.name(), "OK");
assert_eq!(format!("{}", ok), "OK");   // Display uses name()

// Works with unknown values too
let unknown = HttpStatus::from_raw(500);
assert_eq!(unknown.name(), "Unknown");
assert_eq!(format!("{:?}", unknown), "HttpStatus(500)");
assert_eq!(format!("{}", unknown), "Unknown");
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

## Helper macros

### `implement_display![Name]`

Adds a `core::fmt::Display` implementation to a type defined by `extrum!`. The implementation delegates to the `name()` method, printing the variant name for known values and `"Unknown"` otherwise.

```rust
use extrum::{extrum, implement_display};

extrum! { enum MyEnum: u8 { A = 1 } }
implement_display![MyEnum];

assert_eq!(format!("{}", MyEnum::A), "A");
```

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
impl core::fmt::Debug for Name { ... }
impl Eq for Name {}
impl PartialEq<BaseType> for Name { ... }
impl PartialEq<Name> for BaseType { ... }
```

After calling `implement_display![Name]`, the following impl is also generated:

```rust
impl core::fmt::Display for Name {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(self.name())
    }
}
```

## When to use this instead of a real `enum`

- You need to store values that are not in the predefined set (e.g., reserved or future enum values from a C library).
- You want a transparent wrapper for interop, but still want named constants for the known cases.
- You want `Debug` to print the variant name when possible, but fall back to the numeric value.

If you **only** need the named constants and the underlying type should **never** hold other values, use a standard Rust `enum` instead.

## `no_std` compatibility

The crate is `#![no_std]`-compatible. Enable the `no_std` feature to use it in embedded environments.

## License

Licensed under either of

- Apache License, Version 2.0
- MIT license

at your option.
