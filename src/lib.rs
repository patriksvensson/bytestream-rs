/*!
This crate provides a convenient way of reading and writing bytes to a buffer
that implements the standard [`Read`] or [`Write`] traits.

Supported std types include [`u8`], [`u16`], [`u32`], [`u64`], [`i8`],
[`i16`], [`i32`], [`i64`], [`String`], [`Vec<T>`] and [`HashMap<T, V>`].

Reading and writing of these types is done using the [`byteorder`]
crate as big endian.
The reason for reading and writing as big endian is that this crate was
written with sending data over the network in mind. It should be fairly
easy to add support for little endian if anyone would have use for it,
but for now it's big endian only.

# Installation

Add the following to your `Cargo.toml` file:

```toml
[dependencies]
bytestream = "0.*"
```

# Examples

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

// Create a buffer that implements the `Write` trait
let mut buffer = Vec::<u8>::new();

// Write some data to the buffer
let foo = Foo { bar: "corgi".to_owned(), baz: 37 };
foo.write_to(&mut buffer).unwrap();

// Read the data back from the buffer
// We wrap the buffer in a Cursor::<T> that implements the `Read` trait
let mut cursor = Cursor::new(buffer);
let other = Foo::read_from(&mut cursor).unwrap();

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
#![cfg_attr(not(feature = "batteries-included"), no_std)]

#[cfg(feature = "batteries-included")]
use std::collections::HashMap;
#[cfg(feature = "batteries-included")]
use std::hash::{BuildHasher, Hash};
use std::io::{Read, Result, Write};

#[cfg(feature = "batteries-included")]
use byteorder::{BigEndian, ReadBytesExt, WriteBytesExt};

/// The streamable trait allows for reading and writing
/// bytes to and from a buffer.
///
/// # Example
///
/// ```
/// use std::io::{Read, Result, Write};
/// use bytestream::Streamable;
///
/// pub struct Foo {
///     bar: String,
///     baz: u32,
/// }
///
/// impl Streamable for Foo {
///     fn read_from<R: Read>(buffer: &mut R) -> Result<Self> {
///         Ok(Self {
///             bar: String::read_from(buffer)?,
///             baz: u32::read_from(buffer)?,
///         })
///     }
///     fn write_to<W: Write>(&self, buffer: &mut W) -> Result<()> {
///         self.bar.write_to(buffer)?;
///         self.baz.write_to(buffer)?;
///         Ok(())
///     }
/// }
/// ```
pub trait Streamable: Sized {
    /// Reads something from the specified buffer.
    fn read_from<R: Read>(buffer: &mut R) -> Result<Self>;

    /// Writes something to the specified buffer.
    fn write_to<W: Write>(&self, buffer: &mut W) -> Result<()>;
}

//////////////////////////////////////////////////////////////////////////////
// Boolean

#[cfg(feature = "batteries-included")]
impl Streamable for bool {
    fn read_from<R: Read>(buffer: &mut R) -> Result<Self> {
        Ok(buffer.read_u8()? == 1)
    }

    fn write_to<W: Write>(&self, buffer: &mut W) -> Result<()> {
        buffer.write_u8(if *self { 1 } else { 0 })?;
        Ok(())
    }
}

//////////////////////////////////////////////////////////////////////////////
// Unsigned integers

#[cfg(feature = "batteries-included")]
impl Streamable for u64 {
    fn read_from<R: Read>(buffer: &mut R) -> Result<Self> {
        Ok(buffer.read_u64::<BigEndian>()?)
    }

    fn write_to<W: Write>(&self, buffer: &mut W) -> Result<()> {
        buffer.write_u64::<BigEndian>(*self)?;
        Ok(())
    }
}

#[cfg(feature = "batteries-included")]
impl Streamable for u32 {
    fn read_from<R: Read>(buffer: &mut R) -> Result<Self> {
        Ok(buffer.read_u32::<BigEndian>()?)
    }

    fn write_to<W: Write>(&self, buffer: &mut W) -> Result<()> {
        buffer.write_u32::<BigEndian>(*self)?;
        Ok(())
    }
}

#[cfg(feature = "batteries-included")]
impl Streamable for u16 {
    fn read_from<R: Read>(buffer: &mut R) -> Result<Self> {
        Ok(buffer.read_u16::<BigEndian>()?)
    }

    fn write_to<W: Write>(&self, buffer: &mut W) -> Result<()> {
        buffer.write_u16::<BigEndian>(*self)?;
        Ok(())
    }
}

#[cfg(feature = "batteries-included")]
impl Streamable for u8 {
    fn read_from<R: Read>(buffer: &mut R) -> Result<Self> {
        Ok(buffer.read_u8()?)
    }

    fn write_to<W: Write>(&self, buffer: &mut W) -> Result<()> {
        buffer.write_u8(*self)?;
        Ok(())
    }
}

//////////////////////////////////////////////////////////////////////////////
// Signed integers

#[cfg(feature = "batteries-included")]
impl Streamable for i64 {
    fn read_from<R: Read>(buffer: &mut R) -> Result<Self> {
        Ok(buffer.read_i64::<BigEndian>()?)
    }

    fn write_to<W: Write>(&self, buffer: &mut W) -> Result<()> {
        buffer.write_i64::<BigEndian>(*self)?;
        Ok(())
    }
}

#[cfg(feature = "batteries-included")]
impl Streamable for i32 {
    fn read_from<R: Read>(buffer: &mut R) -> Result<Self> {
        Ok(buffer.read_i32::<BigEndian>()?)
    }

    fn write_to<W: Write>(&self, buffer: &mut W) -> Result<()> {
        buffer.write_i32::<BigEndian>(*self)?;
        Ok(())
    }
}

#[cfg(feature = "batteries-included")]
impl Streamable for i16 {
    fn read_from<R: Read>(buffer: &mut R) -> Result<Self> {
        Ok(buffer.read_i16::<BigEndian>()?)
    }

    fn write_to<W: Write>(&self, buffer: &mut W) -> Result<()> {
        buffer.write_i16::<BigEndian>(*self)?;
        Ok(())
    }
}

#[cfg(feature = "batteries-included")]
impl Streamable for i8 {
    fn read_from<R: Read>(buffer: &mut R) -> Result<Self> {
        Ok(buffer.read_i8()?)
    }

    fn write_to<W: Write>(&self, buffer: &mut W) -> Result<()> {
        buffer.write_i8(*self)?;
        Ok(())
    }
}

//////////////////////////////////////////////////////////////////////////////
// String

#[cfg(feature = "batteries-included")]
impl Streamable for String {
    fn read_from<R: std::io::Read>(buffer: &mut R) -> Result<Self> {
        let len = u16::read_from(buffer)?; // TODO: Use 7-bit encoded size
        let mut bytes = Vec::<u8>::new();
        buffer.take(len as u64).read_to_end(&mut bytes)?;
        let ret = String::from_utf8(bytes).unwrap();
        Ok(ret)
    }
    fn write_to<W: std::io::Write>(&self, buffer: &mut W) -> Result<()> {
        let bytes = self.as_bytes();
        (bytes.len() as u16).write_to(buffer)?; // TODO: Use 7-bit encoded size
        buffer.write_all(bytes)?;
        Ok(())
    }
}

//////////////////////////////////////////////////////////////////////////////
// Collections

#[cfg(feature = "batteries-included")]
impl<T: Streamable> Streamable for Vec<T> {
    fn read_from<R: Read>(buffer: &mut R) -> Result<Self> {
        let count = buffer.read_u16::<BigEndian>()?; // TODO: Use 7-bit encoded size
        let mut vec = Vec::<T>::with_capacity(count as usize);
        for _ in 0..count {
            vec.push(T::read_from(buffer)?);
        }
        Ok(vec)
    }

    fn write_to<W: Write>(&self, buffer: &mut W) -> Result<()> {
        buffer.write_u16::<BigEndian>(self.len() as u16)?; // TODO: Use 7-bit encoded size
        for item in self.iter() {
            item.write_to(buffer)?;
        }
        Ok(())
    }
}

#[cfg(feature = "batteries-included")]
impl<T: Streamable + Eq + Hash, V: Streamable, S: BuildHasher + Default> Streamable
    for HashMap<T, V, S>
{
    fn read_from<R: Read>(buffer: &mut R) -> Result<Self> {
        let len = u32::read_from(buffer)?; // TODO: Use 7-bit encoded size
        let mut map = HashMap::with_capacity_and_hasher(len as usize, Default::default());
        for _ in 0..len {
            map.insert(T::read_from(buffer)?, V::read_from(buffer)?);
        }
        Ok(map)
    }

    fn write_to<W: Write>(&self, buffer: &mut W) -> Result<()> {
        (self.len() as u32).write_to(buffer)?; // TODO: Use 7-bit encoded size
        for (key, value) in self.iter() {
            key.write_to(buffer)?;
            value.write_to(buffer)?;
        }
        Ok(())
    }
}

//////////////////////////////////////////////////////////////////////////////
// Tests

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::Cursor;

    // https://stackoverflow.com/a/27582993/936
    macro_rules! map(
        { $($key:expr => $value:expr),+ } => {
            {
                let mut m = ::std::collections::HashMap::new();
                $(
                    m.insert($key, $value);
                )+
                m
            }
         };
    );

    #[derive(Debug, PartialEq)]
    pub struct Foo {
        pub foo: u32,
        pub bar: u16,
        pub baz: Baz,
        pub corgi: Vec<u8>,
        pub waldo: HashMap<i32, String>,
    }

    #[derive(Debug, PartialEq)]
    pub struct Baz {
        pub baz: u32,
    }

    impl Streamable for Foo {
        fn read_from<R: Read>(buffer: &mut R) -> Result<Self> {
            Ok(Self {
                foo: u32::read_from(buffer)?,
                bar: u16::read_from(buffer)?,
                baz: Baz::read_from(buffer)?,
                corgi: Vec::<u8>::read_from(buffer)?,
                waldo: HashMap::<i32, String>::read_from(buffer)?,
            })
        }

        fn write_to<W: Write>(&self, buffer: &mut W) -> Result<()> {
            self.foo.write_to(buffer)?;
            self.bar.write_to(buffer)?;
            self.baz.write_to(buffer)?;
            self.corgi.write_to(buffer)?;
            self.waldo.write_to(buffer)?;
            Ok(())
        }
    }

    impl Streamable for Baz {
        fn read_from<R: Read>(buffer: &mut R) -> Result<Self> {
            Ok(Self {
                baz: u32::read_from(buffer)?,
            })
        }
        fn write_to<W: Write>(&self, buffer: &mut W) -> Result<()> {
            self.baz.write_to(buffer)?;
            Ok(())
        }
    }

    #[test]
    pub fn should_serialize_custom_struct() {
        let foo = Foo {
            foo: 31,
            bar: 7,
            baz: Baz { baz: 23 },
            corgi: vec![1, 2, 3, 4],
            waldo: map! { 1 => "A".to_owned(), 2 => "B".to_owned() },
        };

        let mut buffer = Vec::<u8>::new();
        foo.write_to(&mut buffer).unwrap();

        let mut cursor = Cursor::new(buffer);
        let result = Foo::read_from(&mut cursor).unwrap();

        assert_eq!(foo, result);
    }
}
