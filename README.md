# bytestream

This crate provides a convenient way of reading and writing bytes to a buffer
that implements the standard [`Read`] or [`Write`] traits.

## Examples

```rust
use std::io::{Cursor, Read, Result, Write};
use bytestream::Streamable;

#[derive(Debug, PartialEq)]
pub struct Foo {
    bar: String,
    baz: u32,
}

impl Streamable for Foo {
    fn read_from<R: Read>(buffer: &mut R) -> Result<Self> {
        Ok(Self {
            bar: String::read_from(buffer)?,
            baz: u32::read_from(buffer)?,
        })
    }

    fn write_to<W: Write>(&self, buffer: &mut W) -> Result<()> {
        self.bar.write_to(buffer)?;
        self.baz.write_to(buffer)?;
        Ok(())
    }
}

// Create a new instance of `Foo`
let foo = Foo {
    bar: "corgi".to_owned(),
    baz: 37
};

// Write it to a buffer that implements the `Write` trait
let mut buffer = Vec::<u8>::new();
foo.write_to(&mut buffer).unwrap();

// Read it back from the buffer
// We wrap the buffer in a Cursor::<T> that implements the `Read` trait
let mut cursor = Cursor::new(buffer);
let other = Foo::read_from(&mut cursor).unwrap();

assert_eq!(foo, other);
```

## The `std-types` feature

If the `std-types` feature is enabled (which it is by default),
byte conversion of foreign types is done using the [`byteorder`] crate and all
data is read and written as big endian. Supported std types
include [`u8`], [`u16`], [`u32`], [`u64`], [`i8`], [`i16`], [`i32`],
[`i64`], [`String`], [`Vec<T>`] and [`HashMap<T, V>`].

The reason for only supporting big endian data conversion
is that this crate was written with sending data over the network in mind.
It should be fairly easy to add support for little endian if anyone would
have use for it, but for now it's big endian only.

## Credits

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