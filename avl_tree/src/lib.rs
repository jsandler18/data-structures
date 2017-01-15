//! A self balancing binary tree
use std::cmp::{Ord, Ordering, max};
use std::mem::replace;
use std::collections::VecDeque;


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
    queue: VecDeque<(&'a K,&'a V)>,
}

impl<'a, K: 'a + Ord, V: 'a> Iter<'a,K, V> {

    fn new(tree: &'a AvlTree<K,V>) -> Self {
        let mut iter = Iter { 
            queue: VecDeque::new(),
        };
        iter.init(tree);
        iter
    }

    fn init(&mut self, tree: &'a AvlTree<K,V>) {
        tree.0.as_ref().map(|node| {
            self.init(&node.left);
            self.queue.push_back((&node.key,&node.val));
            self.init(&node.right);
            0
        });
    }
}
impl<'a, K: 'a + Ord , V: 'a> Iterator for Iter<'a,K, V> {
    type Item = (&'a K, &'a V);

    fn next(&mut self) -> Option<(&'a K, &'a V)> {
       self.queue.pop_front()
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
    /// use avltree_map::AvlTree;
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
        grandparent_node.as_mut().unwrap().left.0 = parent_node.as_mut().unwrap().right.0.take();
        //move grandparent to parent's right
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

    fn rebalance(&mut self) {

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
    }

    /// Creates a new empty AvlTree
    ///
    /// #Examples
    ///
    /// ```
    /// use avltree_map::AvlTree;
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
    /// use avltree_map::AvlTree;
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
        
        self.rebalance();

        result
    }
   

    /// checks if the tree is empty.
    ///
    ///  #Examples
    ///
    ///  ```
    ///  use avltree_map::AvlTree;
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
    ///  use avltree_map::AvlTree;
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
    ///  use avltree_map::AvlTree;
    ///
    ///  let mut tree = AvlTree::new();
    ///  tree.insert(37, "b");
    ///  tree.insert(1,"a");
    ///
    ///  assert_eq!(tree.remove(&1), Some("a"));
    ///  assert!(!tree.contains_key(&1));
    ///  assert_eq!(tree.remove(&2), None);
    ///
    ///  ```
    pub fn remove(&mut self, key: &K) -> Option<V> {
        let (result, replacement) = match self.0.take() {
            None => (None, AvlTree(None) as AvlTree<K,V>),
            Some(mut node) => {
                match key.cmp(&node.key) {
                    Ordering::Less => (node.left.remove(key), AvlTree(Some(node))),
                    Ordering::Greater => (node.right.remove(key), AvlTree(Some(node))),
                    Ordering::Equal => {
                        let node = *node;
                        let replacement = match (node.left.0, node.right.0) {
                            //no subtrees, replacement is nothing
                            (None,None) => AvlTree(None) as AvlTree<K,V>,
                            //one subtree, replacement is the one subtree
                            (Some(left),None) => AvlTree(Some(left)),
                            (None,Some(right)) => AvlTree(Some(right)),
                            //two subtrees, must move some thigns around to find a suitable
                            //replacement
                            (Some(left),Some(right)) => {
                                    let mut newright = AvlTree(Some(right));
                                    let mut min = newright.take_min();
                                    min.right = newright;
                                    min.left = AvlTree(Some(left));
                                    AvlTree(Some(min))
                            }
                        };
                        self.update_height();
                        (Some(node.val),replacement)
                    }
                }
            }
        };
        *self = replacement;

        self.rebalance();
        result
    }

    fn take_min(&mut self) -> Box<AvlNode<K,V>> {
        let result = if self.0.as_ref().unwrap().left.is_empty() {
            let mut res = self.0.take().unwrap();
            self.0 = res.right.0.take();
            res
        } else {
            self.0.as_mut().unwrap().left.take_min()
        };

        self.update_one_height();
        result
    }

    /// Takes a referenece to something of type Key and
    /// checks if the key is present. 
    ///
    /// #Examples
    ///
    ///  ```
    ///  use avltree_map::AvlTree;
    ///
    ///  let mut tree = AvlTree::new();
    ///  tree.insert(37, "b");
    ///  tree.insert(1,"a");
    ///
    ///  assert!(tree.contains_key(&1));
    ///  assert!(!tree.contains_key(&2));
    ///
    pub fn contains_key(&self, key: &K) -> bool {
        match self.0 {
            None => false,
            Some(ref node) => {
                if *key == node.key {
                    true
                }
                else if *key < node.key {
                    node.left.contains_key(key)
                }
                else {
                    node.right.contains_key(key)
                }
            }
        }
    }

    /// Gives an iterator over the key-value pairs in the tree, sorted by key.
    ///
    /// #Examples
    ///
    ///  ```
    ///  use avltree_map::AvlTree;
    ///
    ///  let mut tree = AvlTree::new();
    ///  tree.insert(37, "b");
    ///  tree.insert(1,"a");
    ///
    ///  assert_eq!(tree.iter().next().unwrap(), (&1, &"a"));
    ///
    pub fn iter(&self) -> Iter<K,V> {
       Iter::new(self)
    }
}

#[cfg(test)]
mod test {
    use AvlTree;


    #[test]
    fn test_insert() {

        let mut tree = AvlTree::new();
        for num in 1..100 {
            assert_eq!(tree.insert(num, num),None);
        }
        //ensure self balancing is working.
        //ceil log2(100) == 7, so height must be <= 7
        assert!(tree.height() <= 7);

        //check reinserting returns right thing
        assert_eq!(tree.insert(5,30), Some(5));
        assert_eq!(tree.insert(5,35), Some(30));
        assert_eq!(tree.insert(99,35), Some(99));
        assert_eq!(tree.insert(101,35), None);

    }


    #[test]
    fn test_remove() {
        let mut tree = AvlTree::new();
        for num in 1..1000 {
            assert_eq!(tree.insert(num, num),None);
        }
        assert!(tree.height() <= 10);

        for num in 100..900 {
            assert_eq!(tree.remove(&num), Some(num));
        }

        println!("{}",tree.height());
        assert!(tree.height() <= 8);

        assert_eq!(tree.remove(&50), Some(50));
        assert_eq!(tree.remove(&500), None);
        assert!(!tree.contains_key(&400));

    }

    #[test]
    fn test_iter() {
        let mut tree = AvlTree::new();
        for num in 1..1000 {
            assert_eq!(tree.insert(num, num),None);
        }

        let mut c = 1;
        //ensures the keys come back in order
        for (key,val) in tree.iter() {
            assert!(key == val);
            assert!(*key == c);
            c += 1;
        }

    }
}
