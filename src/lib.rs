//! # Extrum
//!
//! A macro for defining enum-like types with a specified underlying integer type.
//! It generates a transparent wrapper struct with associated constants, conversion
//! methods, debug formatting, and equality comparisons with the base type.
//!
//! ## Example
//!
//! ```
//! use extrum::extrum;
//!
//! extrum! {
//!     #[derive(PartialEq)]
//!     enum MyEnum: u8 {
//!         FOO = 1,
//!         BAR = 2,
//!     }
//! }
//!
//! assert_eq!(MyEnum::FOO, MyEnum::from_raw(1));
//! assert_eq!(MyEnum::BAR.into_raw(), 2);
//! assert_eq!(MyEnum::FOO.name(), "FOO");
//! ```

/// Defines a strongly-typed wrapper around an integer base type.
///
/// The macro generates:
/// - A transparent `struct` with the given visibility and name.
/// - `pub const` associated constants for each variant.
/// - `from_raw` / `into_raw` conversion methods.
/// - A `name` method returning the variant name as a string.
/// - `From` implementations for conversions to/from the base type.
/// - `Debug` implementation that prints variant names or unknown values.
/// - `Eq` and cross-type `PartialEq` implementations with the base type.
///
/// # Syntax
///
/// ```text
/// extrum! {
///     $(#[$struct_attrs:meta])*
///     $vis enum $name:ident : $base_ty:ty {
///         $variant:ident = $value:expr, *
///     }
/// }
/// ```
///
/// # Example
///
/// ```
/// # use extrum::extrum;
/// extrum! {
///     #[derive(PartialEq, Clone, Copy)]
///     pub enum StatusCode: u16 {
///         OK = 200,
///         NOT_FOUND = 404,
///     }
/// }
///
/// let ok = StatusCode::OK;
/// assert_eq!(ok.into_raw(), 200);
/// assert_eq!(StatusCode::from_raw(404), StatusCode::NOT_FOUND);
/// assert_eq!(format!("{:?}", ok), "StatusCode::OK");
/// ```
#[macro_export]
macro_rules! extrum
{
    (
        $(#[$struct_attrs:meta])*
        $vis:vis enum $name:ident : $base_ty:ty
        {
            $(
                $variant:ident = $value:expr
            ),* $(,)?
        }
    )
    =>
    {
        $(#[$struct_attrs])*
        #[repr(transparent)]
        $vis struct $name($base_ty);

        impl $name
        {
            $(
                pub const $variant: Self = $name($value);
            )*

            /// Creates a new instance from a raw underlying value.
            pub const fn from_raw(value: $base_ty) -> Self
            {
                $name(value)
            }

            /// Returns the raw underlying value.
            pub const fn into_raw(self) -> $base_ty
            {
                self.0
            }

            /// Returns the name of the variant as a string slice.
            ///
            /// If the raw value does not match any known variant, `"Unknown"` is returned.
            pub fn name(&self) -> &'static str
            {
                match self.0
                {
                    $($value => stringify!($variant),)*
                    _ => "Unknown",
                }
            }
        }

        impl From<$base_ty> for $name
        {
            fn from(value: $base_ty) -> Self
            {
                $name(value)
            }
        }

        impl From<$name> for $base_ty
        {
            fn from(val: $name) -> Self
            {
                val.0
            }
        }

        impl std::fmt::Debug for $name
        {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result
            {
                match self.0
                {
                    $($value => f.write_str(concat!(stringify!($name), "::", stringify!($variant))),)*
                    other => write!(f, "{}({})", stringify!($name), other),
                }
            }
        }

        impl Eq for $name {}

        impl PartialEq<$base_ty> for $name
        {
            fn eq(&self, other: &$base_ty) -> bool
            {
                &self.0 == other
            }
        }

        impl PartialEq<$name> for $base_ty
        {
            fn eq(&self, other: &$name) -> bool
            {
                self == &other.0
            }
        }
    };
}

#[cfg(test)]
mod test
{
    extrum!
    {
        #[derive(PartialEq)]
        enum Extrum: u8
        {
            FOO = 1,
            BAR = 2,
            BAZ = 3,
        }
    }

    #[test] fn test_1() { assert_eq!(Extrum::FOO, Extrum::from_raw(1)); }
    #[test] fn test_2() { assert_eq!(Extrum::BAR, Extrum::from_raw(2)); }
    #[test] fn test_3() { assert_eq!(Extrum::BAZ, Extrum::from_raw(3)); }

    #[test] fn test_4() { assert_eq!(Extrum::FOO, 1); }
    #[test] fn test_5() { assert_eq!(Extrum::BAR, 2); }
    #[test] fn test_6() { assert_eq!(Extrum::BAZ, 3); }

    #[test] fn test_7() { assert_eq!(Extrum::from_raw(1), 1); }
    #[test] fn test_8() { assert_eq!(Extrum::from_raw(2), 2); }
    #[test] fn test_9() { assert_eq!(Extrum::from_raw(3), 3); }

    #[test] fn test_10() { assert_eq!(Extrum::FOO.into_raw(), 1); }
    #[test] fn test_11() { assert_eq!(Extrum::BAR.into_raw(), 2); }
    #[test] fn test_12() { assert_eq!(Extrum::BAZ.into_raw(), 3); }

    #[test] fn test_13() { assert_eq!("Extrum::".to_string() + Extrum::FOO.name(), format!("{:?}", Extrum::FOO)); }
    #[test] fn test_14() { assert_eq!("Extrum::".to_string() + Extrum::BAR.name(), format!("{:?}", Extrum::BAR)); }
    #[test] fn test_15() { assert_eq!("Extrum::".to_string() + Extrum::BAZ.name(), format!("{:?}", Extrum::BAZ)); }

    #[test] fn test_16() { assert_eq!(Extrum::FOO.name(), "FOO"); }
    #[test] fn test_17() { assert_eq!(Extrum::BAR.name(), "BAR"); }
    #[test] fn test_18() { assert_eq!(Extrum::BAZ.name(), "BAZ"); }
}
