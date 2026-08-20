use crate::instrumentation_profile::indexed_profile::*;
use crate::instrumentation_profile::raw_profile::*;
use crate::instrumentation_profile::text_profile::*;
use crate::instrumentation_profile::types::*;
use nom::{error::VerboseError, Err, IResult};
use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use tracing::trace;

pub mod indexed_profile;
pub mod raw_profile;
pub mod summary;
pub mod text_profile;
pub mod types;

pub type ParseResult<'a, T> = IResult<&'a [u8], T, VerboseError<&'a [u8]>>;

pub const fn get_num_padding_bytes(len: u64) -> u8 {
    7 & (8 - (len % 8) as u8)
}

pub fn parse(filename: impl AsRef<Path>) -> io::Result<InstrumentationProfile> {
    let mut buffer = Vec::new();
    let mut f = File::open(filename)?;
    f.read_to_end(&mut buffer)?;
    parse_bytes(buffer.as_slice())
}

pub fn parse_bytes(data: &[u8]) -> io::Result<InstrumentationProfile> {
    let nom_res = if IndexedInstrProf::has_format(data) {
        IndexedInstrProf::parse_bytes(data)
    } else if RawInstrProf64::has_format(data) {
        RawInstrProf64::parse_bytes(data)
    } else if RawInstrProf32::has_format(data) {
        RawInstrProf32::parse_bytes(data)
    } else if TextInstrProf::has_format(data) {
        TextInstrProf::parse_bytes(data)
    } else {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            "Unsupported instrumentation profile format",
        ));
    };
    nom_res.map(|(_bytes, res)| res).map_err(|e| {
        trace!("{}", e);
        let verbose_error_message = |err: VerboseError<&[u8]>| {
            err.errors
                .iter()
                .map(|(_, x)| format!("{:?}", x))
                .collect::<Vec<String>>()
                .join(" ")
        };
        let error_message = match e {
            Err::Error(e) => format!("parser error: {}", verbose_error_message(e)),
            Err::Failure(e) => format!("parser failure: {}", verbose_error_message(e)),
            Err::Incomplete(_) => unreachable!("llvm_profparsers works on complete data"),
        };
        io::Error::new(io::ErrorKind::Other, error_message)
    })
}

pub trait InstrProfReader {
    type Header;
    /// Parse the profile no lazy parsing here!
    fn parse_bytes(input: &[u8]) -> ParseResult<'_, InstrumentationProfile>;
    /// Parses a header
    fn parse_header(input: &[u8]) -> ParseResult<'_, Self::Header>;
    /// Detects that the bytes match the current reader format if it can't read the format it will
    /// return false
    fn has_format(input: impl Read) -> bool;
}

pub trait InstrProfWriter {
    fn write(&self, profile: &InstrumentationProfile, writer: &mut impl Write) -> io::Result<()>;
}
