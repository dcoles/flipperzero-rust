//! Various tools provided by the Flipper Zero SDK.
//!
//! Some of these tools have common pure-Rust equivalents in the crate ecosystem; these
//! are documented on each tool. The types provided here enable application developers to
//! choose their trade-offs:
//!
//! - Using a type in this module means re-using the implementation embedded into the
//!   Flipper Zero firmware. This reduces the size of the application binary, but requires
//!   calls into the Flipper Zero SDK that the Rust compiler cannot optimize away.
//!
//! - Using an equivalent pure-Rust type enables the Rust compiler to optimize the
//!   application more effectively, at the cost of larger binary size.

pub(crate) mod crc32;
pub use self::crc32::Crc32;
