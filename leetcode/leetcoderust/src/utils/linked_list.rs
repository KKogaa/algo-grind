#[derive(PartialEq, Eq, Clone, Debug)]
pub struct LinkedList {
    pub head: Option<Box<ListNode>>,
}

impl LinkedList {
    pub fn from_vec(v: Vec<i32>) -> Self {
        let mut head = None;
        let mut current = &mut head;
        for i in v {
            *current = Some(Box::new(ListNode::new(i)));
            current = &mut current.as_mut().unwrap().next;
        }
        LinkedList { head }
    }
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    pub fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_linked_list() {
        let mut linked_list = LinkedList::from_vec(vec![1, 2, 3]);
        while let Some(node) = linked_list.head {
            linked_list.head = node.next;
        }
    }
}
