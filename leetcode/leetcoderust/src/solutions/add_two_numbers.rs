pub struct Solution {}

use crate::utils::linked_list::ListNode;

impl Solution {
    #[allow(dead_code)]
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut l1 = l1;
        let mut l2 = l2;

        let mut carry = 0;
        let mut l3 = Some(Box::new(ListNode { val: 0, next: None }));
        let mut head = l3.as_mut();

        while l1.is_some() || l2.is_some() || carry > 0 {
            if carry > 0 && !(l1.is_some() || l2.is_some()) {
                head.as_mut().unwrap().next = Some(Box::new(ListNode {
                    val: carry,
                    next: None,
                }));
                head = head.unwrap().next.as_mut();
                carry = 0;
                continue;
            }

            let mut sum = 0;
            if let Some(node) = l1 {
                sum += node.val;
                l1 = node.next;
            }

            if let Some(node) = l2 {
                sum += node.val;
                l2 = node.next
            }

            sum += carry;
            carry = 0;
            carry += sum / 10;
            sum %= 10;

            head.as_mut().unwrap().next = Some(Box::new(ListNode {
                val: sum,
                next: None,
            }));
            head = head.unwrap().next.as_mut();
        }

        l3.unwrap().next
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::linked_list::LinkedList;

    #[test]
    fn test_add_two_numbers() {
        let l1 = LinkedList::from_vec(vec![2, 4, 3]);
        let l2 = LinkedList::from_vec(vec![5, 6, 4]);

        let mut result = Solution::add_two_numbers(l1.head, l2.head);
        while let Some(node) = result {
            println!("{}, ", node.val);
            result = node.next;
        }
    }

    #[test]
    fn test_add_two_numbers_2() {
        let l1 = LinkedList::from_vec(vec![9, 9, 9, 9, 9, 9, 9]);
        let l2 = LinkedList::from_vec(vec![9, 9, 9, 9]);

        let mut result = Solution::add_two_numbers(l1.head, l2.head);
        while let Some(node) = result {
            println!("{}, ", node.val);
            result = node.next;
        }
    }
}
