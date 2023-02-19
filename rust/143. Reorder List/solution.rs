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
    pub fn reorder_list(head: &mut Option<Box<ListNode>>) {
        let mut mid = {
            let mut second = &head.clone();
            let mut first = &mut *head;
            while second.is_some() && second.as_ref().unwrap().next.is_some() {
                second = &second.as_ref().unwrap().next;
                second = &second.as_ref().unwrap().next;
                first = &mut (first.as_mut().unwrap().next);
            }
            first.as_mut().unwrap().next.take()
        };

        let mut prev = None;
        while mid.is_some() {
            let mut cur = mid.unwrap();
            mid = cur.next;
            cur.next = prev;
            prev = Some(cur);
        }

        let mut first = &mut *head;
        let mut second = prev;
        while second.is_some() {
            let tmp1 = first.as_mut().unwrap().next.take();
            let tmp2 = second.as_mut().unwrap().next.take();
            first.as_mut().unwrap().next = second;
            first.as_mut().unwrap().next.as_mut().unwrap().next = tmp1;
            first = &mut (first.as_mut().unwrap().next.as_mut().unwrap().next);
            second = tmp2;
        }
    }
}
