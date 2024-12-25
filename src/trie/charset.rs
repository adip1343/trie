pub trait CharSet {
    const SIZE: usize;
    fn map(ch: char) -> usize;
    fn unmap(hash: usize) -> char;
}

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
