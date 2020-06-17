use std::collections::HashMap;
use std::hash::{BuildHasher, Hash};
use std::io::{Read, Result, Write};

use byteorder::{BigEndian, LittleEndian, ReadBytesExt, WriteBytesExt};

use crate::{ByteOrder, Streamable};

//////////////////////////////////////////////////////////////////////////////
// Boolean

impl Streamable for bool {
    fn read_from<R: Read>(buffer: &mut R, _order: ByteOrder) -> Result<Self> {
        Ok(buffer.read_u8()? == 1)
    }

    fn write_to<W: Write>(&self, buffer: &mut W, _order: ByteOrder) -> Result<()> {
        buffer.write_u8(if *self { 1 } else { 0 })?;
        Ok(())
    }
}

//////////////////////////////////////////////////////////////////////////////
// Unsigned integers

impl Streamable for u64 {
    fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self> {
        match order {
            ByteOrder::BigEndian => Ok(buffer.read_u64::<BigEndian>()?),
            ByteOrder::LittleEndian => Ok(buffer.read_u64::<LittleEndian>()?),
        }
    }

    fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> Result<()> {
        match order {
            ByteOrder::BigEndian => buffer.write_u64::<BigEndian>(*self)?,
            ByteOrder::LittleEndian => buffer.write_u64::<LittleEndian>(*self)?,
        }
        Ok(())
    }
}

impl Streamable for u32 {
    fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self> {
        match order {
            ByteOrder::BigEndian => Ok(buffer.read_u32::<BigEndian>()?),
            ByteOrder::LittleEndian => Ok(buffer.read_u32::<LittleEndian>()?),
        }
    }

    fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> Result<()> {
        match order {
            ByteOrder::BigEndian => buffer.write_u32::<BigEndian>(*self)?,
            ByteOrder::LittleEndian => buffer.write_u32::<LittleEndian>(*self)?,
        }
        Ok(())
    }
}

impl Streamable for u16 {
    fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self> {
        match order {
            ByteOrder::BigEndian => Ok(buffer.read_u16::<BigEndian>()?),
            ByteOrder::LittleEndian => Ok(buffer.read_u16::<LittleEndian>()?),
        }
    }

    fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> Result<()> {
        match order {
            ByteOrder::BigEndian => buffer.write_u16::<BigEndian>(*self)?,
            ByteOrder::LittleEndian => buffer.write_u16::<LittleEndian>(*self)?,
        }
        Ok(())
    }
}

impl Streamable for u8 {
    fn read_from<R: Read>(buffer: &mut R, _order: ByteOrder) -> Result<Self> {
        Ok(buffer.read_u8()?)
    }

    fn write_to<W: Write>(&self, buffer: &mut W, _order: ByteOrder) -> Result<()> {
        buffer.write_u8(*self)?;
        Ok(())
    }
}

//////////////////////////////////////////////////////////////////////////////
// Signed integers

impl Streamable for i64 {
    fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self> {
        match order {
            ByteOrder::BigEndian => Ok(buffer.read_i64::<BigEndian>()?),
            ByteOrder::LittleEndian => Ok(buffer.read_i64::<LittleEndian>()?),
        }
    }

    fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> Result<()> {
        match order {
            ByteOrder::BigEndian => buffer.write_i64::<BigEndian>(*self)?,
            ByteOrder::LittleEndian => buffer.write_i64::<LittleEndian>(*self)?,
        }
        Ok(())
    }
}

impl Streamable for i32 {
    fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self> {
        match order {
            ByteOrder::BigEndian => Ok(buffer.read_i32::<BigEndian>()?),
            ByteOrder::LittleEndian => Ok(buffer.read_i32::<LittleEndian>()?),
        }
    }

    fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> Result<()> {
        match order {
            ByteOrder::BigEndian => buffer.write_i32::<BigEndian>(*self)?,
            ByteOrder::LittleEndian => buffer.write_i32::<LittleEndian>(*self)?,
        }
        Ok(())
    }
}

impl Streamable for i16 {
    fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self> {
        match order {
            ByteOrder::BigEndian => Ok(buffer.read_i16::<BigEndian>()?),
            ByteOrder::LittleEndian => Ok(buffer.read_i16::<LittleEndian>()?),
        }
    }

    fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> Result<()> {
        match order {
            ByteOrder::BigEndian => buffer.write_i16::<BigEndian>(*self)?,
            ByteOrder::LittleEndian => buffer.write_i16::<LittleEndian>(*self)?,
        }
        Ok(())
    }
}

impl Streamable for i8 {
    fn read_from<R: Read>(buffer: &mut R, _order: ByteOrder) -> Result<Self> {
        Ok(buffer.read_i8()?)
    }

    fn write_to<W: Write>(&self, buffer: &mut W, _order: ByteOrder) -> Result<()> {
        buffer.write_i8(*self)?;
        Ok(())
    }
}

//////////////////////////////////////////////////////////////////////////////
// String

impl Streamable for String {
    fn read_from<R: std::io::Read>(buffer: &mut R, order: ByteOrder) -> Result<Self> {
        let len = u16::read_from(buffer, order)?; // TODO: Use 7-bit encoded size
        let mut bytes = Vec::<u8>::new();
        buffer.take(len as u64).read_to_end(&mut bytes)?;
        let ret = String::from_utf8(bytes).unwrap();
        Ok(ret)
    }
    fn write_to<W: std::io::Write>(&self, buffer: &mut W, order: ByteOrder) -> Result<()> {
        let bytes = self.as_bytes();
        (bytes.len() as u16).write_to(buffer, order)?; // TODO: Use 7-bit encoded size
        buffer.write_all(bytes)?;
        Ok(())
    }
}

//////////////////////////////////////////////////////////////////////////////
// Collections

impl<T: Streamable> Streamable for Vec<T> {
    fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self> {
        let len = u16::read_from(buffer, order)?; // TODO: Use 7-bit encoded size
        let mut vec = Vec::<T>::with_capacity(len as usize);
        for _ in 0..len {
            vec.push(T::read_from(buffer, order)?);
        }
        Ok(vec)
    }

    fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> Result<()> {
        (self.len() as u16).write_to(buffer, order)?; // TODO: Use 7-bit encoded size
        for item in self.iter() {
            item.write_to(buffer, order)?;
        }
        Ok(())
    }
}

impl<T: Streamable + Eq + Hash, V: Streamable, S: BuildHasher + Default> Streamable
    for HashMap<T, V, S>
{
    fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self> {
        let len = u16::read_from(buffer, order)?; // TODO: Use 7-bit encoded size
        let mut map = HashMap::with_capacity_and_hasher(len as usize, Default::default());
        for _ in 0..len {
            map.insert(T::read_from(buffer, order)?, V::read_from(buffer, order)?);
        }
        Ok(map)
    }

    fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> Result<()> {
        (self.len() as u16).write_to(buffer, order)?; // TODO: Use 7-bit encoded size
        for (key, value) in self.iter() {
            key.write_to(buffer, order)?;
            value.write_to(buffer, order)?;
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
        fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self> {
            Ok(Self {
                foo: u32::read_from(buffer, order)?,
                bar: u16::read_from(buffer, order)?,
                baz: Baz::read_from(buffer, order)?,
                corgi: Vec::<u8>::read_from(buffer, order)?,
                waldo: HashMap::<i32, String>::read_from(buffer, order)?,
            })
        }

        fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> Result<()> {
            self.foo.write_to(buffer, order)?;
            self.bar.write_to(buffer, order)?;
            self.baz.write_to(buffer, order)?;
            self.corgi.write_to(buffer, order)?;
            self.waldo.write_to(buffer, order)?;
            Ok(())
        }
    }

    impl Streamable for Baz {
        fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self> {
            Ok(Self {
                baz: u32::read_from(buffer, order)?,
            })
        }
        fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> Result<()> {
            self.baz.write_to(buffer, order)?;
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
        foo.write_to(&mut buffer, ByteOrder::BigEndian).unwrap();

        let mut cursor = Cursor::new(buffer);
        let result = Foo::read_from(&mut cursor, ByteOrder::BigEndian).unwrap();

        assert_eq!(foo, result);
    }
}
