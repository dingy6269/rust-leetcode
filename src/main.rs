use std::cell::RefCell;
use std::rc::Rc;
use std::cmp;

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
  pub fn max_depth(
    root: Option<Rc<RefCell<TreeNode>>>,
    depth: i32
  ) -> i32 {
    match root {
      Some(node_rc) => {
        let node = node_rc.borrow();

        let l_count = Self::max_depth(node.left.clone(), depth + 1);
        let r_count = Self::max_depth(node.right.clone(), depth + 1);

        return l_count.max(r_count);
      }
      None => return depth
    };
  }
}

fn main() {
  let v: i32 = 2;

  let r = Solution::max_depth(Some(Rc::new(
    RefCell::new(TreeNode::new(v)),
  )));
}
