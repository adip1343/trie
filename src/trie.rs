use std::default::Default;

mod member;

#[allow(unused_imports)]
pub mod charset;
use charset::CharSet;

mod internal;
use internal::_Trie;

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
        Self {
            internal: _Trie::<_, _, SIZE>::new(),
        }
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
        Self {
            internal: _Trie::<_, _, SIZE>::new(),
        }
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
        Trie::<
            self::charset::LowerCase,
            { <self::charset::LowerCase as self::charset::CharSet>::SIZE },
        >::new()
    };
    ($C:ty) => {
        Trie::<$C, { <$C as self::charset::CharSet>::SIZE }>::new()
    };
}

#[macro_export]
macro_rules! multi_trie {
    () => {
        MultiTrie::<
            self::charset::LowerCase,
            { <self::charset::LowerCase as self::charset::CharSet>::SIZE },
        >::new()
    };
    ($C:ty) => {
        MultiTrie::<$C, { <$C as self::charset::CharSet>::SIZE }>::new()
    };
}

#[cfg(test)]
mod tests {
    use self::charset::LowerCase;
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
