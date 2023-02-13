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
        let mut p = head.as_mut();
        while let Some(n) = p {
            if let Some(next) = n.next.as_mut() {
                if n.val == next.val {
                    n.next = next.next.take();
                    p = Some(n);
                } else {
                    p = n.next.as_mut();
                }
            }
            else {
                p = n.next.as_mut();
            }
        }
        head
    }
}

/*
 #[macro_export]
macro_rules! linkedlist {
    () => {
        None
    };
    ($($e:expr), *) => {
        {
            let mut head = Box::new($crate::ListNode::new(0));
            let mut ref_head = &mut head;

            $(
            ref_head.next = Some(Box::new($crate::ListNode::new($e)));
            ref_head = ref_head.next.as_mut().unwrap();
            )*

            let _ = ref_head;
            head.next
        }
    };
}
    let mut list = linkedlist![1, 2];
    let mut res = Solution::delete_duplicates(list);
    println!("{res:?}");
 */
