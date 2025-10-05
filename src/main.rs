use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, PartialEq, Eq)]
pub struct TreeNode {
  pub val: i32,
  pub left: Option<Rc<RefCell<TreeNode>>>,
  pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
  // initialize with inline parameters
  #[inline]
  pub fn new(val: i32) -> Self {
    TreeNode { val, left: None, right: None }
  }
}

// LeetCode provides `struct Solution;` usually.
// Add it if your file doesn't have it:
pub struct Solution;

impl Solution {
  pub fn inorder_traversal(
    root: Option<Rc<RefCell<TreeNode>>>,
  ) -> Vec<i32> {
    match root {
      Some(node_rc) => {
        let mut values = Vec::new();
        let node = node_rc.borrow();

        if (node.left.is_none() && node.right.is_none()) {
          return vec![node.val];
        }

        values.extend(Self::inorder_traversal(node.left));
        values.extend(vec![node.val]);
        values.extend(Self::inorder_traversal(node.right));

        return values;
      }
      None => return vec![],
    };

    vec![]
  }
}

fn main() {
  let v: i32 = 2;

  let r = Solution::inorder_traversal(Some(Rc::new(
    RefCell::new(TreeNode::new(v)),
  )));
}
