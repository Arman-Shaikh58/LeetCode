struct Solution;

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
    pub fn remove_nth_from_end(head: Option<Box<ListNode>>, n: i32) -> Option<Box<ListNode>> {
        // First pass: find length
        let mut len = 0;
        let mut current = &head;

        while let Some(node) = current {
            len += 1;
            current = &node.next;
        }

        // Dummy node
        let mut dummy = Box::new(ListNode { val: 0, next: head });

        // Move to node before target
        let target = len - n as usize;

        let mut current = &mut dummy;

        for _ in 0..target {
            current = current.next.as_mut().unwrap();
        }

        // Remove node
        let next = current.next.as_mut().unwrap().next.take();
        current.next = next;

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
    let head = create_list(vec![1, 2, 3, 4, 5]);
    let n = 2;

    let head = Solution::remove_nth_from_end(head, n);

    print_list(&head);
}
