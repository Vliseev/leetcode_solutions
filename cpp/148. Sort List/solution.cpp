/**
 * Definition for singly-linked list.
 * struct ListNode {
 *     int val;
 *     ListNode *next;
 *     ListNode() : val(0), next(nullptr) {}
 *     ListNode(int x) : val(x), next(nullptr) {}
 *     ListNode(int x, ListNode *next) : val(x), next(next) {}
 * };
 */


ListNode* mergeTwoList(ListNode* first, ListNode* second){
    ListNode dummy{};
    auto cur = &dummy;
    while (first && second) {
        if (first->val < second->val) {
            cur->next = first;
            first = first->next;
        } else {
            cur->next = second;
            second = second->next;
        }
        cur = cur->next;
    }
    if (first) {
        cur->next = first;
    }
    if (second) {
        cur->next = second;
    }
    return dummy.next;
}


ListNode* getMid(ListNode* head) {
    ListNode *prev = nullptr;
    ListNode *fast = head;
    ListNode *slow = head;
    while (fast && fast->next) {
        prev = slow;
        slow = slow->next;
        fast = fast->next;
        fast = fast->next;
    }
    prev->next = nullptr;
    return slow;
}


ListNode* sortListImpl(ListNode* head) {
    if (!head || !head->next)
        return head;
    ListNode* mid = getMid(head);
    ListNode* left = sortListImpl(head);
    ListNode* right = sortListImpl(mid);
    return mergeTwoList(left, right);
}


class Solution {
public:
    ListNode* sortList(ListNode* head) {
        return sortListImpl(head);
    }
};

/*
 ListNode head(4, new ListNode(2, new ListNode(1, new ListNode(3, new ListNode(5)))));
*/
