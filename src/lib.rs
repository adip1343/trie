#[macro_use]
pub mod trie;

#[cfg(test)]
mod tests {

    use crate::trie::*;

    #[test]
    fn trie_works() {
        let mut t = trie!();
        t.add("abc");
        assert!(t.contains("abc"));
        assert!(!t.contains("a"));
    }

    #[test]
    fn multi_trie_works() {
        let mut t = multi_trie!();
        t.add("abc");
        assert_eq!(t.count("abc"), 1);
        t.add("abc");
        assert_eq!(t.count("abc"), 2);
        assert!(t.contains("abc"));
        assert!(!t.contains("a"));
        assert_eq!(t.count("a"), 0);
    }

    #[test]
    fn multi_trie_works_uppercase() {
        let mut t = multi_trie!(charset::UpperCase);
        t.add("ABC");
        assert_eq!(t.count("ABC"), 1);
        t.add("ABC");
        assert_eq!(t.count("ABC"), 2);
        assert!(t.contains("ABC"));
        assert!(!t.contains("A"));
        assert_eq!(t.count("A"), 0);
    }
}
