//! A self balancing binary tree
use std::cmp::Ord;
use std::cmp::max;
use std::mem::replace;
use std::mem::swap;

struct AvlNode<K: Ord, V> {
    key: K,
    val: V,
    height: i32,
    left: AvlTree<K,V>,
    right: AvlTree<K,V>
}

/// A map based on a binary tree that self balances using the AVL algorithm
pub struct AvlTree<K: Ord, V> (Option<Box<AvlNode<K,V>>>);

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

    /// Returns the height of the tree.
    /// The height is defined as The number of Nodes
    /// on the longest path from the root to any leaf.
    /// Since the tree is self balancing, the height should
    /// be about log2(n) plus or minus 1
    ///
    /// #Examples
    /// ```
    /// use ds::avltree::AvlTree;
    ///
    /// let mut tree = AvlTree::new();
    ///
    /// assert_eq!(tree.height(),0);
    /// tree.insert(1,"a");
    /// assert_eq!(tree.height(),1);
    /// tree.insert(2,"b");
    /// assert_eq!(tree.height(),2);
    /// tree.insert(3,"c");
    /// assert_eq!(tree.height(),2);
    /// tree.insert(4,"d");
    /// assert_eq!(tree.height(),3);
    /// ```
    pub fn height(&self) -> i32 {
        match self.0 {
            None => 0,
            Some(ref node) => node.height
        }
    }
    ///checks the balance of the tree from this node.
    ///the magnitude of the result idicates how unbalanced the tree is,
    ///with 0 being balanced, 1 being unbalance of 1, and so on.
    ///the sign indicates which direction the balance leans.
    ///Negative isa left leaning tree, positive is right leaning.
    fn check_balance(&self) -> i32 {
        match self.0 {
            None => 0,
            Some(ref node) => node.right.height() - node.left.height()
        }
    }

    ///updates this subtree's heights. returns the height of the tree after updating
    fn update_height(&mut self) -> i32{
        match self.0 {
            None => 0,
            Some(ref mut node) => {
                node.height = max(node.left.update_height(),node.right.update_height()) + 1;
                node.height
            }

        }
    }
    ///updates this node's height.
    fn update_one_height(&mut self) {
        match self.0 {
            None => (),
            Some(ref mut node) => {
                node.height = max(node.left.height(),node.right.height()) + 1;
            }

        }
    }

    fn right_rot(&mut self) {
        //move gradparent out of self
        let mut grandparent_node = self.0.take();
        //move left child out of grandparent
        let mut parent_node = grandparent_node.as_mut().unwrap().left.0.take();
        //move parent's right to grandparent's left
                println!("here");
        grandparent_node.as_mut().unwrap().left.0 = parent_node.as_mut().unwrap().right.0.take();
        //move grandparent to parent's right
                println!("here");
        parent_node.as_mut().unwrap().right.0 = grandparent_node;
        //move parent into self
        self.0 = parent_node;
        //update heights based on new positions
        self.update_height();
    }

    fn left_rot(&mut self) {
        let mut grandparent_node = self.0.take();
        let mut parent_node = grandparent_node.as_mut().unwrap().right.0.take();
        grandparent_node.as_mut().unwrap().right.0 = parent_node.as_mut().unwrap().left.0.take();
        parent_node.as_mut().unwrap().left.0 = grandparent_node;
        self.0 = parent_node;
        self.update_height();
    }

    fn right_left_rot(&mut self) {
        self.0.as_mut().unwrap().right.right_rot();
        self.left_rot();
    }

    fn left_right_rot(&mut self) {
        self.0.as_mut().unwrap().left.left_rot();
        self.right_rot();

    }

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
        AvlTree(None as Option<Box<AvlNode<K,V>>>)
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
        let result = match self.0 {
            //if there is no data here, insert the key-value pair here and return None
            None => {
                self.0 = Some(Box::new(AvlNode {
                    key: key,
                    val: val,
                    height: 1,
                    left: AvlTree::new(),
                    right: AvlTree::new()
                }));
                None
            }
            Some(ref mut root) => {
                //if key exists, swap out values, return old value
                if key == root.key {
                    Some(replace(&mut root.val,val))
                }
                //go left
                else if key < root.key {
                    root.left.insert(key,val)
                }
                //go right
                else {
                    root.right.insert(key,val)
                }
            }
        };
        
        //check balance
        let balance = self.check_balance();
        
        //too left leaning
        if balance == -2 {
            //if left subtree is left leaning or balanced, then right rotate, else
            //left-right rotate
            if self.0.as_mut().unwrap().left.check_balance() <= 0 {
                self.right_rot();    
            }
            else {
                self.left_right_rot();
            }
        }
        //too right leaning
        else if balance == 2 {
            if self.0.as_mut().unwrap().right.check_balance() >= 0 {
                self.left_rot();    
            }
            else {
                self.right_left_rot();
            }

        };
        //else balanced, nothing to do

        //update heights, not using update_height method to not do unneccesary recursions
        self.update_one_height();

        result
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
        match self.0 {
            None => None,
            Some(ref node) => {
                if *key == node.key {
                    Some(&node.val)
                }
                else if *key < node.key {
                    node.left.get(key)
                }
                else {
                    node.right.get(key)
                }
            }
        }
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

#[cfg(test)]
mod test {

    use avltree::AvlTree;

    #[test]
    fn test_insert() {

        let mut tree = AvlTree::new();
        for num in 1..100 {
            assert_eq!(tree.insert(num, num),None);
        }
        //ensure self balancing is working.
        //ceil log2(100) == 7, so height must be <= 7
        assert!(tree.height() <= 7);

        assert_eq!(tree.insert(5,30), Some(5));


    }
}
