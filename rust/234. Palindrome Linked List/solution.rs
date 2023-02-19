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


pub fn reverse_list(head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
  let (mut prev, mut curr) = (None, head);
  while let Some(mut node) = curr  {
      let tmp = node.next;
      node.next = prev;
      prev = Some(node);
      curr = tmp;
  }
  prev
}

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

impl Solution {
    pub fn is_palindrome(head: Option<Box<ListNode>>) -> bool {
        let middle = middle_node(head.clone());
        let reversed_head = reverse_list(middle);

        let mut fp = &head;
        let mut sp = &reversed_head;
        while fp.is_some() && sp.is_some() {
            if fp.as_ref().unwrap().val != sp.as_ref().unwrap().val {
                return false;
            }
            fp = &fp.as_ref().unwrap().next;
            sp = &sp.as_ref().unwrap().next;
        }

        true
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
let mut res = Solution::is_palindrome(list);
*/
