//https://leetcode.com/problems/reorder-list/solutions/2794174/rust-get-middle-reverse-and-merge-modular-solution/?languageTags=rust


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
use std::cmp::Ordering;


impl Solution {
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        #[inline(always)]
        fn get_list_middle(head: &mut Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let (mut fast, mut slow) = (&head.clone(), head);
            while fast.is_some() {
                fast = &(fast.as_ref().unwrap().next);
                if fast.is_some() {
                    fast = &fast.as_ref().unwrap().next;   
                    slow = &mut(slow.as_mut().unwrap().next); 
                }
            }
            slow.as_mut().unwrap().next.take()
        }

        #[inline(always)]
        fn reverse_list(mut head: Option<Box<ListNode>>) -> Option<Box<ListNode>> {
            let mut prev = None;
            while let Some(mut curr) = head {
                head = curr.next;
                curr.next = prev;
                prev = Some(curr);
            }
            prev
        }

        #[inline(always)]
        fn merge_lists(mut head1: &mut Option<Box<ListNode>>, mut head2: Option<Box<ListNode>>) {
            let mut h1 = head1;
            let mut h2 = head2;
            while h1.is_some() && h2.is_some() {
                let mut h1next = h1.as_mut().unwrap().next.take();
                let mut h2next = h2.as_mut().unwrap().next.take();
                h1.as_mut().unwrap().next = h2;
                h1.as_mut().unwrap().next.as_mut().unwrap().next = h1next;
                h1 = &mut(h1.as_mut().unwrap().next.as_mut().unwrap().next);
                h2 = h2next;
            }
            if h2.is_some() {
                h1 = &mut(h2);
            }
        }

        let mut head2 = get_list_middle(head);
        head2 = reverse_list(head2);
        merge_lists(head, head2);
    }
}
