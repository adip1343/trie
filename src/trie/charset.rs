//! contains trait definitions for space optimizing your trie to optimize for space. This trait is also re-exported for better usability

/// Implement this trait to space optimize your trie according to your character set
pub trait CharSet {
    /// number of characters in the CharSet
    const SIZE: usize;
    /// provide one to one mapping from `char` to `usize` 
    fn map(ch: char) -> usize;
    /// provide one to one mapping from `usize` to `char`
    fn unmap(hash: usize) -> char;
}

/// Set of ASCII lowercase alphabets
#[derive(Default)]
pub struct LowerCase();

impl CharSet for LowerCase {
    const SIZE: usize = 26;
    fn map(ch: char) -> usize {
        ch as usize - 'a' as usize
    }
    fn unmap(hash: usize) -> char {
        (b'a' + hash as u8) as char
    }
}

/// Set of ASCII uppercase alphabets
#[derive(Default)]
#[allow(unused_imports)]
pub struct UpperCase();

impl CharSet for UpperCase {
    const SIZE: usize = 26;
    fn map(ch: char) -> usize {
        ch as usize - 'A' as usize
    }
    fn unmap(hash: usize) -> char {
        (b'A' + hash as u8) as char
    }
}

/// Set of ASCII characters
#[derive(Default)]
#[allow(unused_imports)]
pub struct Ascii();

impl CharSet for Ascii {
    const SIZE: usize = 256;
    fn map(ch: char) -> usize {
        ch as usize
    }
    fn unmap(hash: usize) -> char {
        (hash as u8) as char
    }
}
