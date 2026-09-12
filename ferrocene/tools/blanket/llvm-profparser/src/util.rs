use flate2::read::ZlibDecoder;
use nom::{
    error::{ContextError, ErrorKind, ParseError},
    IResult,
};
use std::io::Read;
use std::path::{Path, PathBuf};
use tracing::debug;

pub fn parse_leb128<'a, E>(mut input: &'a [u8]) -> IResult<&'a [u8], u64, E>
where
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]>,
{
    let start = input;
    let x = leb128::read::unsigned(&mut input).map_err(|e| {
        use leb128::read::Error as LebError;
        let kind = match e {
            LebError::Overflow => ErrorKind::Satisfy,
            // Here our Read impl is a slice so only one error possible
            LebError::IoError(_) => ErrorKind::Eof,
        };
        nom::Err::Error(E::from_error_kind(start, kind))
    })?;
    Ok((input, x))
}

pub fn parse_string_ref<'a, E>(input: &'a [u8]) -> IResult<&'a [u8], String, E>
where
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]>,
{
    let (input, uncompressed_size) = parse_leb128::<E>(input)?;
    let (input, compressed_size) = parse_leb128::<E>(input)?;
    if compressed_size != 0 {
        if compressed_size as usize > input.len() {
            debug!("Unexpected EOF parsing a string ref");
            Err(nom::Err::Error(E::from_error_kind(input, ErrorKind::Eof)))
        } else {
            let compressed_size = compressed_size as usize;
            let mut decoder = ZlibDecoder::new(&input[..compressed_size]);
            let mut output = vec![];
            if decoder.read_to_end(&mut output).is_ok() {
                let name = String::from_utf8(output);
                let name = name.unwrap();
                Ok((&input[compressed_size..], name))
            } else {
                let inner = E::from_error_kind(input, ErrorKind::Satisfy);
                Err(nom::Err::Failure(E::add_context(
                    input,
                    "invalid deflate stream",
                    inner,
                )))
            }
        }
    } else {
        let uncompressed_size = uncompressed_size as usize;
        match String::from_utf8(input[..uncompressed_size].to_vec()) {
            Ok(name) => Ok((&input[uncompressed_size..], name)),
            Err(_e) => {
                debug!("Invalid UTF-8 string");
                let inner = E::from_error_kind(input, ErrorKind::Satisfy);
                Err(nom::Err::Error(E::add_context(
                    input,
                    "invalid utf-8 string",
                    inner,
                )))
            }
        }
    }
}

/// Parses a list of paths - this is currently only used in parsing the sections in an instrumented
/// object file, and due to CWD joining is different to the other string parsing implemented
pub fn parse_path_list<'a, E>(input: &'a [u8], version: u64) -> IResult<&'a [u8], Vec<PathBuf>, E>
where
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]>,
{
    let (input, list_length) = parse_leb128::<E>(input)?;

    if version < 3 {
        // read_uncompressed
        let (input, values) = parse_uncompressed_file_list(input, list_length, version)?;
        Ok((input, values))
    } else {
        let (input, uncompressed_size) = parse_leb128::<E>(input)?;
        let (input, compressed_size) = parse_leb128::<E>(input)?;

        let compressed_size = compressed_size as usize;
        let uncompressed_size = uncompressed_size as usize;

        if compressed_size == 0 {
            let (input, values) = parse_uncompressed_file_list::<E>(input, list_length, version)?;
            Ok((input, values))
        } else {
            let mut decoder = ZlibDecoder::new(&input[..compressed_size]);
            let mut output = Vec::with_capacity(uncompressed_size);
            decoder.read_to_end(&mut output).unwrap();
            // Use context error to
            let values = parse_uncompressed_string_list::<()>(&output)
                .map(|(_, v)| v.iter().map(PathBuf::from).collect())
                .map_err(|_| nom::Err::Failure(E::from_error_kind(input, ErrorKind::Fail)))?;
            Ok((&input[compressed_size..], values))
        }
    }
}

fn read_string<'a, E>(bytes: &'a [u8]) -> IResult<&'a [u8], String, E>
where
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]>,
{
    let (bytes, len) = parse_leb128::<E>(bytes)?;
    let len = len as usize;
    let string = String::from_utf8_lossy(&bytes[..len]).to_string();
    Ok((&bytes[len..], string))
}

fn parse_uncompressed_file_list<'a, E>(
    mut input: &'a [u8],
    list_length: u64,
    version: u64,
) -> IResult<&'a [u8], Vec<PathBuf>, E>
where
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]>,
{
    let mut res = vec![];
    if version < 5 {
        for _ in 0..list_length {
            let (bytes, string) = read_string(input)?;
            res.push(PathBuf::from(string));
            input = bytes;
        }
        Ok((input, res))
    } else {
        let (bytes, cwd) = read_string(input)?;
        let cwd = Path::new(&cwd);
        res.push(cwd.to_path_buf());
        input = bytes;
        for _ in 1..list_length {
            let (bytes, path) = read_string(input)?;
            input = bytes;
            let tmp = Path::new(&path);
            if tmp.is_absolute() {
                res.push(tmp.to_path_buf());
            } else {
                // NB here the llvm code sometimes has the compilation dir available and joins
                // paths onto that instead.
                res.push(cwd.join(tmp));
            }
        }
        Ok((input, res))
    }
}

fn parse_uncompressed_string_list<'a, E>(mut input: &'a [u8]) -> IResult<&'a [u8], Vec<String>, E>
where
    E: ParseError<&'a [u8]> + ContextError<&'a [u8]>,
{
    let mut res = vec![];
    while !input.is_empty() {
        let (bytes, len) = parse_leb128::<E>(input)?;
        let len = len as usize;
        let string = String::from_utf8_lossy(&bytes[..len]).to_string();
        res.push(string);

        input = &bytes[len..];
    }
    Ok((input, res))
}
