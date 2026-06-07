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
    pub fn merge_k_lists(mut lists: Vec<Option<Box<ListNode>>>) -> Option<Box<ListNode>> {
        let lists_len = lists.len();
        let mut dummy = Box::new(ListNode::new(0));
        let mut tail = &mut dummy;

        loop {
            let mut smallest = i32::MAX;
            let mut sli: i32 = -1; // sli means smallest mean iudex 
            for i in 0..lists_len {
                if let Some(list) = lists[i].as_ref() {
                    if list.val < smallest {
                        smallest = list.val;
                        sli = i as i32;
                    }
                }
            }
            if sli == -1 {
                break;
            }
            let mut list = lists[sli as usize].take().unwrap();
            lists[sli as usize] = list.next.take();

            tail.next = Some(list);
            tail = tail.next.as_mut().unwrap();
        }
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

fn main() {}
