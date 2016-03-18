use std::cmp::Ordering;

pub struct Node<T> { weight: usize
                   , tree: Tree<T> }

pub enum Tree<T> { Branch(Box<Node<T>>, Box<Node<T>>)
                 , Leaf(T) }

impl<T> Eq for Node<T> {}
impl<T> PartialEq for Node<T> {
    fn eq(&self, other: &Node<T>) -> bool { self.weight == other.weight }
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
        match self.weight.cmp(&other.weight) {
            Ordering::Less    => Ordering::Greater
          , Ordering::Equal   => Ordering::Equal
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
