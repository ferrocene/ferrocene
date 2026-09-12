use crate::instrumentation_profile::{types::*, ParseResult};
use indexmap::IndexMap;
use nom::{
    error::{ErrorKind, ParseError, VerboseError, VerboseErrorKind},
    number::complete::*,
};
use std::borrow::Cow;
use std::mem::size_of;
use tracing::debug;

#[derive(Copy, Clone, Debug)]
struct KeyDataLen {
    key_len: u64,
    data_len: u64,
}

#[derive(Clone, Debug)]
pub(crate) struct HashTable(pub IndexMap<(u64, String), InstrProfRecord>);

fn read_key_data_len(input: &[u8]) -> ParseResult<'_, KeyDataLen> {
    let (bytes, key_len) = le_u64(input)?;
    let (bytes, data_len) = le_u64(bytes)?;
    let res = KeyDataLen { key_len, data_len };
    Ok((bytes, res))
}

fn read_key(input: &[u8], key_len: usize) -> ParseResult<'_, Cow<'_, str>> {
    if key_len > input.len() {
        Err(nom::Err::Failure(VerboseError::from_error_kind(
            &input[input.len()..],
            ErrorKind::Eof,
        )))
    } else {
        let res = String::from_utf8_lossy(&input[..key_len]);
        Ok((&input[key_len..], res))
    }
}

fn read_value(
    version: u64,
    mut input: &[u8],
    data_len: usize,
) -> ParseResult<'_, (u64, InstrProfRecord)> {
    if data_len % 8 != 0 {
        // Element is corrupted, it should be aligned
        let errors = vec![(
            input,
            VerboseErrorKind::Context("table data length is not 8 byte aligned"),
        )];
        return Err(nom::Err::Failure(VerboseError { errors }));
    }
    if input.len() < data_len {
        tracing::trace!("{} < {}", input.len(), data_len);
        return Err(nom::Err::Failure(VerboseError::from_error_kind(
            &input[input.len()..],
            ErrorKind::Eof,
        )));
    }
    let mut result = vec![];
    let end_len = input.len() - data_len;

    let expected_end = &input[data_len..];
    let mut last_hash = 0;

    while input.len() > end_len {
        let mut counts = vec![];
        let (bytes, hash) = le_u64(input)?;
        last_hash = hash;
        if bytes.len() <= end_len {
            break;
        }
        // This is only available for versions > v1. But as rust won't be going backwards to legacy
        // versions it's a safe assumption.
        let (bytes, counts_len) = le_u64(bytes)?;
        if bytes.len() <= end_len {
            break;
        }
        input = bytes;
        if size_of::<u64>().saturating_mul(counts_len as usize) > input.len() {
            let errors = vec![(
                input,
                VerboseErrorKind::Context("hash_table value count length exceeds length of input"),
            )];
            return Err(nom::Err::Failure(VerboseError { errors }));
        }
        for _ in 0..counts_len {
            let (bytes, count) = le_u64(input)?;
            input = bytes;
            counts.push(count);
        }
        result.push((hash, InstrProfRecord { counts, data: None }));
        if input.len() <= end_len {
            break;
        }

        if version > 10 {
            let (bytes, bitmap_bytes) = le_u64(input)?;
            input = bytes;
            for _i in 0..bitmap_bytes {
                let (bytes, _) = le_u64(input)?;
                input = bytes;
            }
        }
        // This should always be true
        if version > 2 {
            // If the version is > v2 then there can also be value profiling data so lets try and parse
            // that now
            let (bytes, total_size) = le_u32(input)?;
            if bytes.len() <= end_len {
                break;
            }
            let (bytes, num_value_kinds) = le_u32(bytes)?;
            // Here it's just less than because we don't need to read anything else so if it's equal to
            // we're good
            if bytes.len() < end_len {
                break;
            }
            input = bytes;
            let value_prof_data = ValueProfData {
                total_size,
                num_value_kinds,
            };
            if value_prof_data.num_value_kinds > 0 && version > 2 {
                // If we actually want to change data in future get result.last_mut() and change it
                // there
                break;
            }
        }
    }
    if result.is_empty() {
        result.push((last_hash, InstrProfRecord::default()));
    }
    input = expected_end;
    assert!(!result.is_empty());
    Ok((input, result.remove(0)))
}

impl HashTable {
    fn new() -> Self {
        Self(IndexMap::new())
    }

    /// buckets is the data the hash table buckets start at - the start of the `HashTable` in memory.
    /// hash. offset shows the offset from the base address to the start of the `HashTable` as this
    /// will be used to correct any offsets
    pub(crate) fn parse<'a>(
        version: u64,
        input: &'a [u8],
        _offset: usize,
        bucket_start: usize,
    ) -> ParseResult<'a, Self> {
        let (bytes, num_buckets) = le_u64(&input[bucket_start..])?;
        debug!("Number of hashtable buckets: {}", num_buckets);
        let (_bytes, mut num_entries) = le_u64(bytes)?;
        debug!("Number of entries: {}", num_entries);

        let mut payload = input;
        let mut result = Self::new();
        //TODO is this change right?
        while num_entries > 0 {
            let (bytes, entries) = result.parse_bucket(version, payload, num_entries)?;
            payload = bytes;
            num_entries = entries;
        }
        Ok((payload, result))
    }

    fn parse_bucket<'a>(
        &mut self,
        version: u64,
        input: &'a [u8],
        mut num_entries: u64,
    ) -> ParseResult<'a, u64> {
        let (bytes, num_items_in_bucket) = le_u16(input)?;
        debug!(
            "Number of items in bucket: {}, input remaining: {}",
            num_items_in_bucket,
            input.len()
        );
        let mut remaining = bytes;
        for _i in 0..num_items_in_bucket {
            let (bytes, _hash) = le_u64(remaining)?;
            debug!("Hash(?): {}", _hash);
            let (bytes, lens) = read_key_data_len(bytes)?;
            let (bytes, key) = read_key(bytes, lens.key_len as usize)?;
            debug!("lengths: {:?} and key: {}", lens, key);
            let (bytes, (hash, value)) = read_value(version, bytes, lens.data_len as usize)?;
            debug!("hash: {}, value: {:?}", hash, value);
            self.0.insert((hash, key.to_string()), value);
            assert!(num_entries > 0);
            num_entries -= 1;

            remaining = bytes;
        }
        Ok((remaining, num_entries))
    }
}
