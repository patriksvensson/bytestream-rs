This crate provides a convenient way of reading and writing bytes to a buffer
that implements the standard [`Read`] or [`Write`] traits.

Supported std types include [`u8`], [`u16`], [`u32`], [`u64`], [`i8`],
[`i16`], [`i32`] and [`i64`].

Reading and writing of these types is done using the [`byteorder`] crate.

# Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
bytestream = "0.4"
```

# Examples

```rust
use std::io::{Cursor, Read, Result, Write};
use bytestream::*;

#[derive(Debug, PartialEq)]
pub struct Foo {
    bar: bool,
    baz: u32,
}

impl StreamReader for Foo {
    fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self> {
        Ok(Self {
            bar: bool::read_from(buffer, order)?,
            baz: u32::read_from(buffer, order)?,
        })
    }
}

impl StreamWriter for Foo {
    fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> Result<()> {
        self.bar.write_to(buffer, order)?;
        self.baz.write_to(buffer, order)?;
        Ok(())
    }
}

// Create a buffer that implements the `Write` trait
let mut buffer = Vec::<u8>::new();

// Write some data to the buffer
let foo = Foo { bar: true, baz: 37 };
foo.write_to(&mut buffer, ByteOrder::BigEndian).unwrap();

// Read the data back from the buffer
// We wrap the buffer in a Cursor::<T> that implements the `Read` trait
let mut cursor = Cursor::new(buffer);
let other = Foo::read_from(&mut cursor, ByteOrder::BigEndian).unwrap();

assert_eq!(foo, other);
```

# Exclude streamable support for std types

If you do not wish to include out-of-the-box support for std types,
you can exclude the default feature in your
`Cargo.toml` file:

```toml
[dependencies]
bytestream = { Version = "0.4", default-features = false }
```

Exluding the default feature will also remove
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
[`Stevenarella`]: https://github.com/iceiix/stevenarella