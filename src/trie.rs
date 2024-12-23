use std::default::Default;

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

struct Node<M>
where
    M: MemberType,
{
    child: [usize; 26],
    member: M,
}

impl<M> Node<M>
where
    M: MemberType,
{
    fn new() -> Self {
        Self {
            child: [0usize; 26],
            member: Default::default(),
        }
    }
}

#[derive(Default)]
struct _Trie<M>
where
    M: MemberType,
{
    _node: Vec<Node<M>>,
}

impl<M> _Trie<M>
where
    M: MemberType,
{
    fn new() -> Self {
        Self {
            _node: vec![Node::new()],
        }
    }

    fn add(&mut self, s: &str) -> bool {
        let mut idx = 0usize;
        for c in s.chars() {
            let cidx = c as usize - 'a' as usize;
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
        for c in s.chars() {
            let cidx = c as usize - 'a' as usize;
            if self._node[idx].child[cidx] == 0 {
                return Default::default();
            }
            idx = self._node[idx].child[cidx]
        }
        self._node[idx].member.is_member()
    }
}

impl _Trie<u32> {
    fn count(&self, s: &str) -> u32 {
        let mut idx = 0usize;
        for c in s.chars() {
            let cidx = c as usize - 'a' as usize;
            if self._node[idx].child[cidx] == 0 {
                return 0;
            }
            idx = self._node[idx].child[cidx]
        }
        self._node[idx].member
    }
}

#[derive(Default)]
pub struct Trie(_Trie<bool>);

impl Trie {
    pub fn new() -> Self {
        Self(_Trie::<bool>::new())
    }

    pub fn add(&mut self, s: &str) -> bool {
        self.0.add(s)
    }

    pub fn contains(&self, s: &str) -> bool {
        self.0.contains(s)
    }
}

#[derive(Default)]
pub struct MultiTrie(_Trie<u32>);

impl MultiTrie {
    pub fn new() -> Self {
        Self(_Trie::<u32>::new())
    }

    pub fn add(&mut self, s: &str) -> bool {
        self.0.add(s)
    }

    pub fn contains(&self, s: &str) -> bool {
        self.0.contains(s)
    }

    pub fn count(&self, s: &str) -> u32 {
        self.0.count(s)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_single_add() {
        let mut trie = Trie::new();
        trie.add("string");
        assert!(trie.contains("string"));
    }

    #[test]
    fn test_single_eq() {
        let mut trie1 = Trie::new();
        assert!(trie1.add("string"));
        assert!(!trie1.add("string"));
    }

    #[test]
    fn test_multi_add() {
        let mut trie = MultiTrie::new();
        trie.add("string");
        assert!(trie.contains("string"));
    }

    #[test]
    fn test_multi_eq() {
        let mut trie = MultiTrie::new();
        assert!(trie.add("string"));
        assert!(!trie.add("string"));
        assert_eq!(trie.count("string"), 2);
    }
}
