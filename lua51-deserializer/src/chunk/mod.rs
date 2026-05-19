use nom::IResult;

pub use header::Header;

use crate::{
    chunk::header::{Endianness, Format},
    function::Function,
};

pub mod header;

#[derive(Debug)]
pub struct Chunk<'a> {
    pub function: Function<'a>,
}

impl<'a> Chunk<'a> {
    pub fn parse(input: &'a [u8]) -> IResult<&[u8], Self> {
        let (input, header) = Header::parse(input)?;

        assert_eq!(header.version_number, 0x51);
        assert_eq!(header.format, Format::Official);
        assert_eq!(header.endianness, Endianness::Little);
        assert!(header.size_t_width == 4 || header.size_t_width == 8);

        let (input, function) = Function::parse(input, header.size_t_width)?;

        Ok((input, Self { function }))
    }
}
