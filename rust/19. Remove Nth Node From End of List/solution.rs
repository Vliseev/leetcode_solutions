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
    pub fn remove_nth_from_end(mut head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        let mut len = 0;
        let mut cur = &head;
        while let Some(val) = &cur {
            len += 1;
            cur = &val.next;
        }
        let mut dummy = Some(Box::new(ListNode {
            val: -1,
            next: head,
        }));
        let mut cur = dummy.as_mut();
        for i in 0..len {
            let cur_node = cur.unwrap();
            if i == len - n {
                let next = cur_node.next.as_mut().unwrap();
                cur_node.next = next.next.take();
                cur = Some(cur_node);
            } else {
                cur = cur_node.next.as_mut();
            }
        }
        dummy.unwrap().next
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

let list = linkedlist![1, 2, 3, 4, 5];
let res = Solution::remove_nth_from_end(list, 2);
println!("{res:?}");
*/
