// SPDX-License-Identifier: MIT OR Apache-2.0
// SPDX-FileCopyrightText: The Ferrocene Developers

#[cfg(test)]
macro_rules! hashmap {
    ($($key:expr => $value:expr),*$(,)?) => {{
        let mut map = HashMap::new();
        $(map.insert($key, $value);)*
        map
    }}
}

#[cfg(test)]
macro_rules! hashset {
    ($($value:expr),*$(,)?) => {{
        let mut set = HashSet::new();
        $(set.insert($value);)*
        set
    }}
}

#[cfg(test)]
pub(crate) use {hashmap, hashset};

/// Convenience function to chain two iterators. This is used rather than the stdlib .chain()
/// because it produces better formatted code.
pub(crate) fn chain<T, I1, I2>(i1: I1, i2: I2) -> std::iter::Chain<I1, I2>
where
    I1: Iterator<Item = T>,
    I2: Iterator<Item = T>,
{
    i1.chain(i2)
}

pub(crate) fn capitalize(input: &str) -> String {
    let mut chars = input.chars();
    match chars.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(chars).collect(),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_capitalize() {
        assert_eq!("", capitalize(""));
        assert_eq!("Hello", capitalize("hello"));
        assert_eq!("Hello world", capitalize("hello world"));
        assert_eq!("HELLO WORLD", capitalize("HELLO WORLD"));
        assert_eq!("SSello", capitalize("ßello"));
    }
}
