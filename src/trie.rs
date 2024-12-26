//! Provides `Trie` and `MultiTrie` implementations space optimized for set of characters in use.

use std::default::Default;

mod member;

#[allow(unused_imports)]
pub mod charset;
pub use self::charset::CharSet;

mod internal;
use self::internal::_Trie;

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
    /// Creates new Trie
    pub fn new() -> Self {
        Self {
            internal: _Trie::<_, _, SIZE>::new(),
        }
    }

    /// Add a string to Trie
    /// 
    /// Returns `true` if Trie didn't contain the string earlier
    pub fn add(&mut self, s: &str) -> bool {
        self.internal.add(s)
    }

    /// Check if trie contains the string
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
    /// Creates a new MultiTrie
    pub fn new() -> Self {
        Self {
            internal: _Trie::<_, _, SIZE>::new(),
        }
    }

    /// Adds a new string to the MultiTrie
    /// 
    /// Returns `true`` if MultiTrie didn't contain the string earlier
    pub fn add(&mut self, s: &str) -> bool {
        self.internal.add(s)
    }

    /// Check if MultiTrie contains given string
    pub fn contains(&self, s: &str) -> bool {
        self.internal.contains(s)
    }

    /// Retuns number of strings in the MultiTrie
    pub fn count(&self, s: &str) -> u32 {
        self.internal.count(s)
    }
}

#[macro_export]
/// Create new `Trie`
/// 
/// `trie!(C:CharSet)` creates new trie with given `CharSet`
/// 
/// `trie!()` defaults to `LowerCase`,set of lowercase alphabets
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
/// Create new `MultiTrie`
/// 
/// `multi_trie!(C:CharSet)` creates new trie with given `CharSet`
/// 
/// `multi_trie!()` defaults to `LowerCase`,set of lowercase alphabets
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
