 # `trie-alg`
 Implement a trie space optimized for character set in use
 ```
 let mut t = trie!();
 t.add("abc");           // true
 t.add("abc");           // false
 t.contains("abc");      // true
 t.contains("a");        // false
 
 let mut t = multi_trie!();
 t.add("abc");           // true
 t.count("abc")          // 1
 t.add("abc");           // false
 t.count("abc")          // 2
 t.contains("abc")       // true
 t.contains("a")         // false
 t.count("a")            // 0
 
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
 
 let mut t = multi_trie!(LowerCaseWithHash);
 ```
