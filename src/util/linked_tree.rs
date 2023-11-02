use std::cell::RefCell;
use std::rc::Rc;

pub trait LinkedTreeOperation<T> {
    fn new_tree(val: T) -> Self;
    fn add_child(&self, val: T) -> Self;
    fn size(&self) -> usize;
    fn deepth(&self) -> usize;
    fn val(&self) -> T;
    fn parent(&self) -> Option<Self>
    where
        Self: Sized;
    fn child(&self, n: usize) -> Option<Self>
    where
        Self: Sized;
    fn child_len(&self) -> usize;
    fn ptr(&self) -> Self;
    fn list_parents(&self) -> Vec<T>;
}

pub type LinkedTree<T> = Rc<RefCell<TreeNode<T>>>;

#[derive(Debug)]
pub struct TreeNode<T> {
    parent: Option<Rc<RefCell<TreeNode<T>>>>,
    children: Vec<Rc<RefCell<TreeNode<T>>>>,
    val: T,

    // count of current node with all children recursive
    size: usize,
    // current node deepth to root, root is 0
    deepth: usize,
}

impl<T> TreeNode<T> {
    fn new(val: T) -> TreeNode<T> {
        TreeNode {
            parent: None,
            children: vec![],
            val,

            size: 1,
            deepth: 0,
        }
    }
    fn inc_size(&mut self) {
        self.size += 1;
        if let Some(parent) = &self.parent {
            parent.borrow_mut().inc_size();
        }
    }
}

impl<T: Clone> LinkedTreeOperation<T> for LinkedTree<T> {
    fn new_tree(val: T) -> Self {
        let node = TreeNode::new(val);
        Rc::new(RefCell::new(node))
    }
    fn add_child(&self, val: T) -> Self {
        let mut node = TreeNode::new(val);
        node.deepth = Rc::clone(&self).borrow_mut().deepth + 1;
        node.parent = Some(Rc::clone(&self));

        let rc_node = Rc::new(RefCell::new(node));
        self.borrow_mut().inc_size();
        self.borrow_mut().children.push(Rc::clone(&rc_node));
        rc_node
    }
    fn val(&self) -> T {
        Rc::clone(&self).borrow().val.clone()
    }
    fn size(&self) -> usize {
        self.borrow().size
    }
    fn deepth(&self) -> usize {
        self.borrow().deepth
    }
    fn parent(&self) -> Option<Self> {
        match &self.borrow().parent {
            None => None,
            Some(parent) => Some(parent.clone()),
        }
    }
    fn child_len(&self) -> usize {
        self.borrow().children.len()
    }
    fn child(&self, n: usize) -> Option<Self> {
        if n >= self.child_len() {
            return None;
        }
        Some(self.borrow().children[n].clone())
    }
    fn ptr(&self) -> Self {
        Rc::clone(&self)
    }
    fn list_parents(&self) -> Vec<T> {
        match &self.borrow().parent {
            None => vec![self.val()],
            Some(parent) => {
                let mut ret = parent.list_parents();
                ret.push(self.val());
                return ret;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_build() {
        let n1 = LinkedTree::new_tree(String::from("1"));
        let n2 = n1.add_child(String::from("2"));
        let n3 = n1.add_child(String::from("3"));
        let n4 = n2.add_child(String::from("4"));
        assert!(n1.size() == 4);
        assert!(n2.size() == 2);
        assert!(n3.size() == 1);
        assert!(n4.size() == 1);
        assert!(n1.deepth() == 0);
        assert!(n2.deepth() == 1);
        assert!(n3.deepth() == 1);
        assert!(n4.deepth() == 2);
        assert!(n1.val().eq("1"));
        assert!(n2.val().eq("2"));
        assert!(n3.val().eq("3"));
        assert!(n4.val().eq("4"));
        assert!(n1.child_len() == 2);
        assert!(n2.child_len() == 1);
        assert!(n3.child_len() == 0);
        assert!(n4.child_len() == 0);
        assert!(n1.child(0).unwrap().val().eq("2"));
        assert!(n1.child(1).unwrap().val().eq("3"));
        assert!(n2.child(0).unwrap().val().eq("4"));
        assert!(n1.child(2).is_none());
        assert!(n3.child(0).is_none());
        assert!(n1.parent().is_none());
        assert!(n2.parent().unwrap().val().eq("1"));
        assert!(n3.parent().unwrap().val().eq("1"));
        assert!(n4.parent().unwrap().val().eq("2"));
        assert!(n4.parent().unwrap().parent().unwrap().val().eq("1"));
        let list = n4.list_parents();
        assert!(list.len() == 3);
        assert!(list[0].eq("1"));
        assert!(list[1].eq("2"));
        assert!(list[2].eq("4"));
    }
}
