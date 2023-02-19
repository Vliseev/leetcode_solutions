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
    pub fn middle_node(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
        let mut slow = &head;
        let mut fast = &head;
        while let Some(val) = fast {
            if val.next.is_none() {
                return slow.clone();
            }
            slow = &slow.as_ref().unwrap().next;

            fast = &fast.as_ref().unwrap().next;
            fast = &fast.as_ref().unwrap().next;
        }

        slow.clone()
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

    let mut list = linkedlist![1, 2, 3, 4, 5];
    let mut res = Solution::middle_node(list);
 */
