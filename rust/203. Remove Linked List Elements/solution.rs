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
    pub fn remove_elements(mut head: Option<Box<ListNode>>, val: i32) -> Option<Box<ListNode>> {
        let mut p = &mut head;
        while p.is_some() {
            if p.as_ref().unwrap().val == val {
                *p = p.take().unwrap().next;
            } else {
                p = &mut p.as_mut().unwrap().next;
            }
        }
        head
    }
}


/*
 *
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
let mut res = Solution::remove_elements(list, 5);
println!("{res:?}");

list = linkedlist![1,2,6,3,4,5,6];
res = Solution::remove_elements(list, 6);
println!("{res:?}");

list = linkedlist![7, 7, 7];
res = Solution::remove_elements(list, 7);
println!("{res:?}");
 */
