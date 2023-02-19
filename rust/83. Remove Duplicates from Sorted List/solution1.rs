// Definition for singly-linked list.
// #[derive(PartialEq, Eq, Clone, Debug)]
// pub struct ListNode {
//   pub val: i32,
//   pub next: Option<Box<ListNode>>
// }
// 
// impl ListNode {
//   #[inline]
//   fn new(val: i32) -> Self {
//     ListNode {
//       next: None,
//       val
//     }
//   }
// }
impl Solution {
    pub fn delete_duplicates(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut cur = &mut head;
        while let Some(node) = &cur {
            match &node.next {
                Some(next) => {
                if node.val == next.as_ref().val {
                    *cur = cur.take().unwrap().next;
                } else {
                    cur = &mut cur.as_mut().unwrap().next;
                }
                }
                None => {
                    cur = &mut cur.as_mut().unwrap().next;
                }
            }
        }
        head
    }
}
