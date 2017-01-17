#![deny(missing_docs)]
//! A Skip List
use std::cmp::{Ord, Ordering};
use std::rc::Rc;

struct Link<K:Ord, V> {
   width: i32,
   next: Option<Rc<SkiplistNode<K,V>>>
}
struct SkiplistNode<K: Ord, V> {
    key: K,
    val: V,
    forward: Vec<Link<K,V>>
}

///A struct used to iterate over the skiplist
pub struct Iter<'a, K: Ord + 'a, V: 'a> {
    //TODO
    v1: &'a K,
    v2: &'a V
}

///A map based on a randomized skiplist.  Allows for insertion, deletion, search, and indexing in
///O(log n) time
pub struct SkipList<K: Ord, V> (Vec<Link<K,V>>);

impl<K: Ord, V> SkipList<K,V> {

    ///Creates an empty SkipList
    pub fn new() -> Self {
        SkipList(Vec::new())
    }

    /// Inserts a key,value pair into the list. Returns None if the key was
    /// not present in the list already. If the key was present, then the key is updated
    /// with the new value and the old value is returned.
    ///
    /// #Examples
    ///
    /// ```
    /// use skiplist::SkipList
    ///
    /// let mut list = SkipList::new();
    ///
    /// assert_eq!(list.insert(37, "a"), None);
    /// assert_eq!(list.is_empty(), false);
    ///
    /// list.insert(37, "b");
    /// assert_eq!(list.insert(37, "c"), Some("b"));
    /// assert_eq!(list.get(&37), Some(&"c"));
    pub fn insert(&mut self, key: K, val: V) -> Option<V> {
        None
    }
   

    /// checks if the list is empty.
    ///
    ///  #Examples
    ///
    ///  ```
    ///  use skiplist::SkipList
    ///
    ///  let mut list = SkipList::new();
    ///
    ///  assert!(list.is_empty());
    ///
    ///  list.insert("hello","world");
    ///
    ///  assert!(!list.is_empty());
    pub fn is_empty(&self) -> bool {
        self.0.len() == 0
    }

    /// Takes a reference to something of type Key and 
    /// returns None if the key is not present, or a reference to the
    /// value if the key is present
    ///
    /// #Examples
    ///
    ///  ```
    ///  use skiplist::SkipList
    ///
    ///  let mut list = SkipList::new();
    ///  list.insert(37, "b");
    ///  list.insert(1,"a");
    ///
    ///  assert_eq!(list.get(&1), Some(&"a"));
    ///  assert_eq!(list.get(&2), None);
    ///
    ///  ```
    pub fn get(&self, key: &K) -> Option<&V> {
        None
    }

    /// Takes a referenece to something of type Key and
    /// attempts to delete the key and its associated value
    /// from the list. Returns None if the key was not present,
    /// and returns the value if the key was present.
    ///
    /// #Examples
    ///
    ///  ```
    ///  use skiplist::SkipList
    ///
    ///  let mut list = SkipList::new();
    ///  list.insert(37, "b");
    ///  list.insert(1,"a");
    ///
    ///  assert_eq!(list.remove(&1), Some("a"));
    ///  assert!(!list.contains_key(&1));
    ///  assert_eq!(list.remove(&2), None);
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
    ///  use skiplist::SkipList
    ///
    ///  let mut list = SkipList::new();
    ///  list.insert(37, "b");
    ///  list.insert(1,"a");
    ///
    ///  assert!(list.contains_key(&1));
    ///  assert!(!list.contains_key(&2));
    ///
    pub fn contains_key(&self, key: &K) -> bool {
        false
    }

    /// Gives an iterator over the key-value pairs in the list, sorted by key.
    ///
    /// #Examples
    ///
    ///  ```
    ///  use skiplist::SkipList
    ///
    ///  let mut list = SkipList::new();
    ///  list.insert(37, "b");
    ///  list.insert(1,"a");
    ///
    ///  assert_eq!(list.iter().next().unwrap(), (&1, &"a"));
    ///
    pub fn iter(&self) -> Iter<K,V> {
        panic!("Not Done");
    }

    /// Gives an iterator over the key-value pairs in the list, sorted by key, in reverse order.
    ///
    /// #Examples
    ///
    ///  ```
    ///  use skiplist::SkipList
    ///
    ///  let mut list = SkipList::new();
    ///  list.insert(37, "b");
    ///  list.insert(1,"a");
    ///
    ///  assert_eq!(list.reverse_iter().next().unwrap(), (&37, &"b"));
    ///
    pub fn reverse_iter(&self) -> Iter<K,V> {
        panic!("Not Done");

    }


    /// Gives an iterator over the key-value pairs in the list that fall within the given start and
    /// end points (inclusive) in sorted order.  If None is given, then that side is unbounded.
    ///
    /// #Examples
    ///
    ///  ```
    ///  use skiplist::SkipList
    ///
    ///  let mut list = SkipList::new();
    ///  list.insert(1,"a");
    ///  list.insert(2,"a");
    ///  list.insert(3,"a");
    ///  list.insert(4,"a");
    ///  list.insert(5,"a");
    ///
    ///  assert_eq!(list.range_iter(Some(&2), None).next().unwrap(), (&2, &"a"));
    ///
    pub fn range_iter(&self, start: Option<&K>, end: Option<&K> ) -> Iter<K,V> {
        panic!("Not Done");

    }

    /// Gives an iterator over the key-value pairs in the list that fall within the given start and
    /// end points (inclusive) in reverse sorted order.  If None is given, then that side is unbounded.
    ///
    /// #Examples
    ///
    ///  ```
    ///  use skiplist::SkipList
    ///
    ///  let mut list = SkipList::new();
    ///  list.insert(1,"a");
    ///  list.insert(2,"a");
    ///  list.insert(3,"a");
    ///  list.insert(4,"a");
    ///  list.insert(5,"a");
    ///
    ///  assert_eq!(list.reverse_range_iter(None, Some(&3)).next().unwrap(), (&3, &"a"));
    ///
    pub fn reverse_range_iter(&self, start: Option<&K>, end: Option<&K> ) -> Iter<K,V> {
        panic!("Not Done");

    }


    

}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
    }
}
