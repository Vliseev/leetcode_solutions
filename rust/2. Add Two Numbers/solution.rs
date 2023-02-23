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
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut p1 = &l1;
        let mut p2 = &l2;

        let mut dummy = Box::new(ListNode::new(-1));
        let mut cur = &mut dummy.next;
        let mut carry = 0;
        while p1.is_some() || p2.is_some() || carry > 0 {
            let v1 = match p1 {
                Some(val) => {
                  p1 = &p1.as_ref().unwrap().next;
                  val.val
                },
                None => 0,
            };
            let v2 = match p2 {
                Some(val) => {
                  p2 = &p2.as_ref().unwrap().next;
                  val.val
                },
                None => 0,
            };

            let cur_val = v1 + v2 + carry;
            *cur = Some(Box::new(ListNode::new(cur_val % 10)));
            cur = &mut (cur.as_mut().unwrap().next);
            carry = cur_val / 10;
        }

        dummy.next
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

fn main() {
    let l1 = linkedlist![2, 4, 3];
    let l2 = linkedlist![5, 6, 4];
    let res = Solution::add_two_numbers(l1, l2);
    println!("{res:?}");
}
 */ 
