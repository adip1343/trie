use std::{default::Default, marker::PhantomData};

trait MemberType
where
    Self: Default + Copy + PartialEq,
{
    fn add(&mut self) -> Self;
    fn is_member(&self) -> bool {
        *self != Default::default()
    }
}

impl MemberType for bool {
    fn add(&mut self) -> Self {
        let ret = *self;
        *self = true;
        ret
    }
}

impl MemberType for u32 {
    fn add(&mut self) -> Self {
        let ret = *self;
        *self += 1;
        ret
    }
}

pub trait CharSet {
    const SIZE: usize;
    fn map(ch: char) -> usize;
    fn unmap(hash: usize) -> char;
}

struct Node<M, const SIZE: usize>
where
    M: MemberType,
{
    child: [usize; SIZE],
    member: M,
}

impl<M, const SIZE: usize> Node<M, SIZE>
where
    M: MemberType,
{
    fn new() -> Self {
        Self {
            child: [0usize; SIZE],
            member: Default::default(),
        }
    }
}

#[derive(Default)]
struct _Trie<M, C, const SIZE: usize>
where
    M: MemberType,
    C: CharSet,
{
    _node: Vec<Node<M, SIZE>>,
    charset: PhantomData<C>,
}

impl<M, C, const SIZE: usize> _Trie<M, C, SIZE>
where
    M: MemberType,
    C: CharSet,
{
    fn new() -> Self {
        Self {
            _node: vec![Node::<M, SIZE>::new()],
            charset: PhantomData,
        }
    }

    fn add(&mut self, s: &str) -> bool {
        let mut idx = 0usize;
        for ch in s.chars() {
            let cidx = C::map(ch);
            if self._node[idx].child[cidx] == 0 {
                self._node[idx].child[cidx] = self._node.len();
                self._node.push(Node::new());
            }
            idx = self._node[idx].child[cidx];
        }
        self._node[idx].member.add() == Default::default()
    }

    fn contains(&self, s: &str) -> bool {
        let mut idx = 0usize;
        for ch in s.chars() {
            let cidx = C::map(ch);
            if self._node[idx].child[cidx] == 0 {
                return Default::default();
            }
            idx = self._node[idx].child[cidx]
        }
        self._node[idx].member.is_member()
    }
}

impl<C, const SIZE: usize> _Trie<u32, C, SIZE>
where
    C: CharSet,
{
    fn count(&self, s: &str) -> u32 {
        let mut idx = 0usize;
        for ch in s.chars() {
            let cidx = C::map(ch);
            if self._node[idx].child[cidx] == 0 {
                return 0;
            }
            idx = self._node[idx].child[cidx]
        }
        self._node[idx].member
    }
}

#[derive(Default)]
pub struct LowerCase();

impl CharSet for LowerCase {
    const SIZE: usize = 26;
    fn map(ch: char) -> usize {
        ch as usize - 'a' as usize
    }
    fn unmap(hash: usize) -> char {
        (b'a'+ hash as u8 ) as char
    }
}

#[derive(Default)]
pub struct UpperCase();

impl CharSet for UpperCase {
    const SIZE: usize = 26;
    fn map(ch: char) -> usize {
        ch as usize - 'A' as usize
    }
    fn unmap(hash: usize) -> char {
        (b'A'+ hash as u8 ) as char
    }
}

#[derive(Default)]
pub struct Ascii();

impl CharSet for Ascii {
    const SIZE: usize = 256;
    fn map(ch: char) -> usize {
        ch as usize
    }
    fn unmap(hash: usize) -> char {
        (hash as u8 ) as char
    }
}

#[derive(Default)]
pub struct Trie<C, const SIZE: usize>
where
    C: CharSet,
{
    internal: _Trie<bool, C, SIZE>,
}

impl<C, const SIZE: usize> Trie<C, SIZE>
where
    C: CharSet,
{
    pub fn new() -> Self {
        Self{internal : _Trie::<_, _, SIZE>::new()}
    }

    pub fn add(&mut self, s: &str) -> bool {
        self.internal.add(s)
    }

    pub fn contains(&self, s: &str) -> bool {
        self.internal.contains(s)
    }
}

#[derive(Default)]
pub struct MultiTrie<C, const SIZE: usize>
where
    C: CharSet,
{
    internal: _Trie<u32, C, SIZE>,
}

impl<C, const SIZE: usize> MultiTrie<C, SIZE>
where
    C: CharSet,
{
    pub fn new() -> Self {
        Self{internal : _Trie::<_, _, SIZE>::new()}
    }

    pub fn add(&mut self, s: &str) -> bool {
        self.internal.add(s)
    }

    pub fn contains(&self, s: &str) -> bool {
        self.internal.contains(s)
    }

    pub fn count(&self, s: &str) -> u32 {
        self.internal.count(s)
    }
}

#[macro_export]
macro_rules! trie {
    () => {
        Trie::<LowerCase, {<LowerCase>::SIZE}>::new()
    };
    ($C:ty) => {
        Trie::<$C, {<$C>::SIZE}>::new()
    };
}

#[macro_export]
macro_rules! multi_trie {
    () => {
        MultiTrie::<LowerCase, {LowerCase::SIZE}>::new()
    };
    ($C:ty) => {
        MultiTrie::<$C, {<$C>::SIZE}>::new()
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_add() {
        let mut trie = trie!();
        trie.add("string");
        assert!(trie.contains("string"));
    }

    #[test]
    fn test_single_eq() {
        let mut trie1 = trie!(LowerCase);
        assert!(trie1.add("string"));
        assert!(!trie1.add("string"));
    }

    #[test]
    fn test_multi_add() {
        let mut trie = multi_trie!();
        trie.add("string");
        assert!(trie.contains("string"));
    }

    #[test]
    fn test_multi_eq() {
        let mut trie = multi_trie!(LowerCase);
        assert!(trie.add("string"));
        assert!(!trie.add("string"));
        assert_eq!(trie.count("string"), 2);
    }
}
