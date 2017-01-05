//! A self balancing binary tree
use std::cmp::Ord;

struct AvlNode<K: Ord, V> {
    key: K,
    val: V,
    height: u32,
    left: Option<Box<AvlNode<K,V>>>,
    right: Option<Box<AvlNode<K,V>>>
}

/// A map based on a binary tree that self balances using the AVL algorithm
pub struct AvlTree<K: Ord, V> (Option<AvlNode<K,V>>);

/// A struct used to iterate over values of the AvlTree
pub struct Iter<'a, K: 'a, V: 'a> {
    //TODO
    t1: &'a K,
    t2: &'a V
}

impl<'a, K: 'a, V: 'a> Iterator for Iter<'a,K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
       Some((self.t1, self.t2))
    }
}

impl<K: Ord, V> AvlTree<K,V> {

    /// Creates a new empty AvlTree
    ///
    /// #Examples
    ///
    /// ```
    /// use ds::avltree::AvlTree;
    ///
    /// let mut tree = AvlTree::new();
    ///
    /// tree.insert(1,"a");
    /// ```
    pub fn new() -> Self {
        AvlTree(None as Option<AvlNode<K,V>>)
    }


    /// Inserts a key,value pair into the tree. Returns None if the key was
    /// not present in the tree already. If the key was present, then the key is updated
    /// with the new value and the old value is returned.
    ///
    /// #Examples
    ///
    /// ```
    /// use ds::avltree::AvlTree;
    ///
    /// let mut tree = AvlTree::new();
    ///
    /// assert_eq!(tree.insert(37, "a"), None);
    /// assert_eq!(tree.is_empty(), false);
    ///
    /// tree.insert(37, "b");
    /// assert_eq!(tree.insert(37, "c"), Some("b"));
    /// assert_eq!(tree.get(&37), Some(&"c"));
    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        None
    }

    /// checks if the tree is empty.
    ///
    ///  #Examples
    ///
    ///  ```
    ///  use ds::avltree::AvlTree;
    ///
    ///  let mut tree = AvlTree::new();
    ///
    ///  assert!(tree.is_empty());
    ///
    ///  tree.insert("hello","world");
    ///
    ///  assert!(!tree.is_empty());
    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    /// Takes a reference to something of type Key and 
    /// returns None if the key is not present, or a reference to the
    /// value if the key is present
    ///
    /// #Examples
    ///
    ///  ```
    ///  use ds::avltree::AvlTree;
    ///
    ///  let mut tree = AvlTree::new();
    ///  tree.insert(37, "b");
    ///  tree.insert(1,"a");
    ///
    ///  assert_eq!(tree.get(&1), Some(&"a"));
    ///  assert_eq!(tree.get(&2), None);
    ///
    ///  ```
    pub fn get(&self, key: &K) -> Option<&V> {
        None
    }

    /// Takes a referenece to something of type Key and
    /// attempts to delete the key and its associated value
    /// from the tree. Returns None if the key was not present,
    /// and returns the value if the key was present.
    ///
    /// #Examples
    ///
    ///  ```
    ///  use ds::avltree::AvlTree;
    ///
    ///  let mut tree = AvlTree::new();
    ///  tree.insert(37, "b");
    ///  tree.insert(1,"a");
    ///
    ///  assert_eq!(tree.remove(&1), Some("a"));
    ///  assert_eq!(tree.remove(&2), None);
    ///
    ///  ```
    pub fn remove(&mut self, key: &K) -> Option<V> {
        None
    }

    /// Takes a referenece to something of type Key and
    /// checks if the key is present. 
    ///
    /// #Examples
    ///
    ///  ```
    ///  use ds::avltree::AvlTree;
    ///
    ///  let mut tree = AvlTree::new();
    ///  tree.insert(37, "b");
    ///  tree.insert(1,"a");
    ///
    ///  assert!(tree.contains_key(&1));
    ///  assert!(!tree.contains_key(&2));
    ///
    pub fn contains_key(&self, key: &K) -> bool {
        false
    }

    /// Gives an iterator over the key-value pairs in the tree, sorted by key.
    ///
    /// #Examples
    ///
    ///  ```
    ///  use ds::avltree::AvlTree;
    ///
    ///  let mut tree = AvlTree::new();
    ///  tree.insert(37, "b");
    ///  tree.insert(1,"a");
    ///
    ///  assert_eq!(tree.iter().next().unwrap(), (&1, &"a"));
    ///
    pub fn iter(&self) -> Iter<K,V> {
       panic!("Not Done!!!")
    }
}
