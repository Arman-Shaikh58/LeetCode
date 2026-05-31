struct Solution;

// Definition for singly-linked list.
#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { next: None, val }
    }
}

impl Solution {
    pub fn merge_two_lists(
        mut list1: Option<Box<ListNode>>,
        mut list2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        while list1.is_some() && list2.is_some() {
            let is_l1 = list1.as_ref().unwrap().val <= list2.as_ref().unwrap().val;

            let mut node = if is_l1 {
                let mut n = list1.take().unwrap();
                list1 = n.next.take();
                n
            } else {
                let mut n = list2.take().unwrap();
                list2 = n.next.take();
                n
            };
            node.next = None;
            tail.next = Some(node);
            tail = tail.next.as_mut().unwrap();
        }

        tail.next = if list1.is_some() { list1 } else { list2 };
        dummy.next
    }
}

// Helper function to create linked list
fn create_list(nums: Vec<i32>) -> Option<Box<ListNode>> {
    let mut head = None;

    for &num in nums.iter().rev() {
        let mut node = Box::new(ListNode::new(num));

        node.next = head;

        head = Some(node);
    }

    head
}

// Helper function to print linked list
fn print_list(list: &Option<Box<ListNode>>) {
    let mut current = list;

    while let Some(node) = current {
        print!("{}", node.val);

        if node.next.is_some() {
            print!(" -> ");
        }

        current = &node.next;
    }

    println!();
}

fn main() {
    // Represents 342
    let l1 = create_list(vec![1, 2, 4]);

    // Represents 465
    let l2 = create_list(vec![1, 3, 4]);

    let result = Solution::merge_two_lists(l1, l2);

    // Expected: 7 -> 0 -> 8
    // Represents 807
    print!("Result: ");
    print_list(&result);
}
