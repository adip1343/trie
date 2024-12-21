
pub mod trie;

#[cfg(test)]
mod tests {
    use crate::trie::{Trie, MultiTrie};

    #[test]
    fn trie_works() {
        let mut t = Trie::new();
        t.add("abc");
        assert!(t.contains("abc"));
        assert!(!t.contains("a"));
    }

    #[test]
    fn multi_trie_works() {
        let mut t = MultiTrie::new();
        t.add("abc");
        assert_eq!(t.count("abc"), 1);
        t.add("abc");
        assert_eq!(t.count("abc"), 2);
        assert!(t.contains("abc"));
        assert!(!t.contains("a"));
        assert_eq!(t.count("a"), 0);
    }
}
