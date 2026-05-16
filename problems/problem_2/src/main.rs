// Definition for singly-linked list.

#[derive(PartialEq, Eq, Clone, Debug)]
pub struct ListNode {
    pub val: i32,
    pub next: Option<Box<ListNode>>,
}

impl ListNode {
    #[inline]
    fn new(val: i32) -> Self {
        ListNode { val, next: None }
    }
}

struct Solution;

impl Solution {
    pub fn add_two_numbers(
        l1: Option<Box<ListNode>>,
        l2: Option<Box<ListNode>>,
    ) -> Option<Box<ListNode>> {
        let mut p1 = l1;
        let mut p2 = l2;

        let mut carry = 0;

        // Dummy node
        let mut new_list = Box::new(ListNode::new(0));

        let mut tail = &mut new_list;

        while p1.is_some() || p2.is_some() || carry != 0 {
            let n1 = p1.take();
            let n2 = p2.take();

            let mut v1 = 0;
            let mut v2 = 0;

            if let Some(node) = n1 {
                v1 = node.val;
                p1 = node.next;
            }

            if let Some(node) = n2 {
                v2 = node.val;
                p2 = node.next;
            }

            let sum = v1 + v2 + carry;

            carry = sum / 10;
            let digit = sum % 10;

            tail.next = Some(Box::new(ListNode::new(digit)));

            tail = tail.next.as_mut().unwrap();
        }

        new_list.next
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
    let l1 = create_list(vec![2, 4, 3]);

    // Represents 465
    let l2 = create_list(vec![5, 6, 4]);

    let result = Solution::add_two_numbers(l1, l2);

    // Expected: 7 -> 0 -> 8
    // Represents 807
    print!("Result: ");
    print_list(&result);
}
