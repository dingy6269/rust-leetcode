#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

struct Solution {}

impl Solution {
    pub fn merge_two_lists(
        list1: Option<Box<ListNode>>,
        list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        match (list1, list2) {
            (None, None) => None,
            (Some(n), None) | (None, Some(n)) => Some(n),
            (Some(mut n1), Some(mut n2)) => {
                if n1.val < n2.val {
                    let next = n1.next.take();
                    n1.next = Self::merge_two_lists(next, Some(n2));
                
                    Some(n1)
                } else {
                    let next = n2.next.take();
                    n2.next = Self::merge_two_lists(Some(n1), next);

                    Some(n2)
                }
            }
        }
    }
}
