#![feature(box_patterns, box_syntax)]

use std::cmp::{Ordering, max};
use std::collections::{HashMap, BinaryHeap};
use Node::*;

extern crate hashmap_ext;
use hashmap_ext::Update;

pub type EncodingTable<T> = HashMap<u8, T>;

pub trait Encodable {
    type Item;
    fn encoding(self) -> EncodingTable<Self::Item>;
}

#[derive(Debug)]
enum Node<T> {
    Branch { l: Box<Node<T>>
           , r: Box<Node<T>>
           , depth: usize
           }
  , Leaf { item: T
         , weight: usize
         }
}

/// Constructs a Huffman tree for a list of xs
fn huffman_tree<T>(xs: &[T]) -> Node<T>
where T: Eq + std::hash::Hash {
    let mut frequencies: HashMap<T, usize> = HashMap::new();
    for x in xs.iter() {
        frequencies.update(x, |v: &mut usize| { v += 1;}, 1);
    }
    unimplemented!()

}

impl<T> Node<T> {

    /// Returns the weight of the node
    #[inline] fn weight(&self) -> usize {
        match self {
            &Branch { box ref l, box ref r, .. } => l.weight() + r.weight()
          , &Leaf { ref weight, .. } => *weight
        }
    }

    /// Returns the depth of the node
    #[inline] fn depth(&self) -> usize {
        match self {
            &Branch { ref depth, .. } => *depth
          , &Leaf { .. } => 0
        }
    }

    /// Constructs a new Branch with two Nodes
    #[inline] fn branch(left: Node<T>, right: Node<T>) -> Node<T> {
        let depth = max(left.depth(), right.depth()) + 1;
        Branch { l: box left
               , r: box right
               , depth: depth
               }
    }

    /// Constructs a Leaf with a weight and an item
    #[inline] fn leaf(item: T, weight: usize) -> Node<T> {
        Leaf { item: item, weight: weight }
    }

}

impl<T> Eq for Node<T> {}
impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool { self.weight() == other.weight() }
}

impl<T> PartialOrd for Node<T> {
    fn partial_cmp(&self, other: &Node<T>) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl<T> Ord for Node<T> {
    /// We have to invert the compare function for Nodes, since the Rust
    /// standard library has no Minimum Priority Queue
    fn cmp(&self, other: &Node<T>) -> Ordering {
        match self.weight().cmp(&other.weight()) {
            Ordering::Less    => Ordering::Greater
          , Ordering::Equal   => self.depth()
                                     .cmp(&other.depth())
                                     .reverse()
          , Ordering::Greater => Ordering::Less
        }
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
