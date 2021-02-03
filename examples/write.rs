use bytestream::*;
use std::io::{Read, Result, Write};

pub struct Foo {
    pub bar: u32,
}

impl StreamReader for Foo {
    fn read_from<R: Read>(buffer: &mut R, order: ByteOrder) -> Result<Self> {
        Ok(Self {
            bar: u32::read_from(buffer, order)?,
        })
    }
}

impl StreamWriter for Foo {
    fn write_to<W: Write>(&self, buffer: &mut W, order: ByteOrder) -> Result<()> {
        self.bar.write_to(buffer, order)?;
        Ok(())
    }
}

fn main() {
    let mut buffer = Vec::<u8>::new();

    let foo = Foo { bar: 31 };
    foo.write_to(&mut buffer, ByteOrder::LittleEndian).unwrap();

    println!("Written buffer: {:?}", buffer);
}
