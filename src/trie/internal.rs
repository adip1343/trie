use std::{default::Default, marker::PhantomData};

use crate::trie::charset::CharSet;
use crate::trie::member::MemberType;

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
pub struct _Trie<M, C, const SIZE: usize>
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
    pub fn new() -> Self {
        Self {
            _node: vec![Node::<M, SIZE>::new()],
            charset: PhantomData,
        }
    }

    pub fn add(&mut self, s: &str) -> bool {
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

    pub fn contains(&self, s: &str) -> bool {
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
    pub fn count(&self, s: &str) -> u32 {
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
