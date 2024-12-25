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

    struct LowerCaseWithHash();
    impl CharSet for LowerCaseWithHash {
        const SIZE: usize = 27;
        fn map(ch: char) -> usize {
            if ch.is_ascii_lowercase() {
               ch as usize - 'a' as usize 
            } else {
                26
            }
        }

        fn unmap(hash: usize) -> char {
            if hash < 26 {
                (b'a'+hash as u8) as char
            }else {
                '#'
            }
        }
    }

    #[test]
    fn custom_charset_works(){
        let mut t = multi_trie!(LowerCaseWithHash);
        t.add("abc");
        assert_eq!(t.count("abc"), 1);
        t.add("abc");
        assert_eq!(t.count("abc"), 2);
        t.add("ab#c");
        t.add("ab#c");
        assert_eq!(t.count("ab#c"), 2);
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
