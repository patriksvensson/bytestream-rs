/*!
This crate provides a convenient way of reading and writing bytes to a buffer
that implements the standard [`Read`] or [`Write`] traits.

Supported std types include [`u8`], [`u16`], [`u32`], [`u64`], [`i8`],
[`i16`], [`i32`], [`i64`], [`String`], [`Vec<T>`] and [`HashMap<T, V>`].

Reading and writing of these types is done using the [`byteorder`] crate.

# Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
bytestream = "0.*"
```

# Examples

```rust
use std::io::{Cursor, Read, Result, Write};
use bytestream::*;

#[derive(Debug, PartialEq)]
pub struct Foo {
    bar: String,
    baz: u32,
}

impl Streamable for Foo {
    fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self> {
        Ok(Self {
            bar: String::read_from(buffer, order)?,
            baz: u32::read_from(buffer, order)?,
        })
    }

    fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> Result<()> {
        self.bar.write_to(buffer, order)?;
        self.baz.write_to(buffer, order)?;
        Ok(())
    }
}

// Create a buffer that implements the `Write` trait
let mut buffer = Vec::<u8>::new();

// Write some data to the buffer
let foo = Foo { bar: "corgi".to_owned(), baz: 37 };
foo.write_to(&mut buffer, ByteOrder::BigEndian).unwrap();

// Read the data back from the buffer
// We wrap the buffer in a Cursor::<T> that implements the `Read` trait
let mut cursor = Cursor::new(buffer);
let other = Foo::read_from(&mut cursor, ByteOrder::BigEndian).unwrap();

assert_eq!(foo, other);
```

# Exclude `Streamable` support for std types

If you do not wish to include out-of-the-box support for std types,
you can exclude the default `batteries-included` feature in your
`Cargo.toml` file:

```toml
[dependencies]
bytestream = { Version = "0.*", default-features = false }
```

Exluding the `batteries-included` feature will also remove
the `byteorder` crate dependency.

# Credits

The inspiration from this crate came from the [`Stevenarella`] Minecraft client.

[`Read`]: https://doc.rust-lang.org/std/io/trait.Read.html
[`Write`]: https://doc.rust-lang.org/std/io/trait.Write.html
[`byteorder`]: https://github.com/BurntSushi/byteorder
[`u8`]: https://doc.rust-lang.org/std/primitive.u8.html
[`u16`]: https://doc.rust-lang.org/std/primitive.u16.html
[`u32`]: https://doc.rust-lang.org/std/primitive.u32.html
[`u64`]: https://doc.rust-lang.org/std/primitive.u64.html
[`i8`]: https://doc.rust-lang.org/std/primitive.i8.html
[`i16`]: https://doc.rust-lang.org/std/primitive.i16.html
[`i32`]: https://doc.rust-lang.org/std/primitive.i32.html
[`i64`]: https://doc.rust-lang.org/std/primitive.i64.html
[`String`]: https://doc.rust-lang.org/std/string/struct.String.html
[`Vec<T>`]: https://doc.rust-lang.org/std/vec/struct.Vec.html
[`HashMap<T, V>`]: https://doc.rust-lang.org/std/collections/struct.HashMap.html
[`Stevenarella`]: https://github.com/iceiix/stevenarella
*/

#![deny(missing_docs)]

use std::io::{Read, Result, Write};

#[cfg(feature = "batteries-included")]
mod optional;
#[cfg(feature = "batteries-included")]
pub use optional::*;

/// `ByteOrder` describes what order to write bytes to the buffer.
#[derive(Copy, Clone)]
pub enum ByteOrder {
    /// Represents big endian byte order (also called network endian).
    /// This is the default order if none is specified.
    BigEndian,
    /// Represents little endian byte order.
    LittleEndian,
}

/// The streamable trait allows for reading and writing
/// bytes to and from a buffer.
///
/// # Example
///
/// ```
/// use std::io::{Read, Result, Write};
/// use bytestream::*;
///
/// pub struct Foo {
///     bar: String,
///     baz: u32,
/// }
///
/// impl Streamable for Foo {
///     fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self> {
///         Ok(Self {
///             bar: String::read_from(buffer, order)?,
///             baz: u32::read_from(buffer, order)?,
///         })
///     }
///     fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> Result<()> {
///         self.bar.write_to(buffer, order)?;
///         self.baz.write_to(buffer, order)?;
///         Ok(())
///     }
/// }
/// ```
pub trait Streamable: Sized {
    /// Reads something from the specified buffer using the specified byte order.
    fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self>;

    /// Writes something to the specified buffer using the specified byte order.
    fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> Result<()>;
}
