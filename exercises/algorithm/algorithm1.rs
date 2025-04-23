/*
	single linked list merge
	This problem requires you to merge two ordered singly linked lists into one ordered singly linked list
*/

use std::fmt::{self, Display, Formatter};
use std::ptr::NonNull;
use std::vec::*;
use std::cmp::Ordering;

#[derive(Debug)]
struct Node<T> {
    val: T,
    next: Option<NonNull<Node<T>>>,
}

impl<T> Node<T> {
    fn new(t: T) -> Node<T> {
        Node {
            val: t,
            next: None,
        }
    }
}
#[derive(Debug)]
struct LinkedList<T> {
    length: u32,
    start: Option<NonNull<Node<T>>>,
    end: Option<NonNull<Node<T>>>,
}

impl<T> Default for LinkedList<T> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T> LinkedList<T> {
    pub fn new() -> Self {
        Self {
            length: 0,
            start: None,
            end: None,
        }
    }

    pub fn add(&mut self, obj: T) {
        let mut node = Box::new(Node::new(obj));
        node.next = None;
        let node_ptr = Some(unsafe { NonNull::new_unchecked(Box::into_raw(node)) });
        match self.end {
            None => self.start = node_ptr,
            Some(end_ptr) => unsafe { (*end_ptr.as_ptr()).next = node_ptr },
        }
        self.end = node_ptr;
        self.length += 1;
    }

    pub fn get(&mut self, index: i32) -> Option<&T> {
        self.get_ith_node(self.start, index)
    }

    fn get_ith_node(&mut self, node: Option<NonNull<Node<T>>>, index: i32) -> Option<&T> {
        match node {
            None => None,
            Some(next_ptr) => match index {
                0 => Some(unsafe { &(*next_ptr.as_ptr()).val }),
                _ => self.get_ith_node(unsafe { (*next_ptr.as_ptr()).next }, index - 1),
            },
        }
    }

	pub fn merge(list_a:LinkedList<T>,list_b:LinkedList<T>) -> Self
    where T: Ord + Clone
	{
        fn into_vec<T: Clone>(list: LinkedList<T>) -> Vec<T> {
            let mut vec: Vec<T> = Vec::with_capacity(list.length as usize);

            let mut node_ptr = list.start;
            while let Some(this) = node_ptr {
                let this = unsafe { this.as_ref() };
                let obj = this.val.clone();
                vec.push(obj);
                node_ptr = this.next;
            }

            assert_eq!(vec.len(), list.length as usize);

            vec
        }

        fn merge_sorted_vecs<T: Ord>(mut vec1: Vec<T>, mut vec2: Vec<T>) -> Vec<T> {
            let mut result = Vec::with_capacity(vec1.len() + vec2.len());
            let mut iter1 = vec1.drain(..);
            let mut iter2 = vec2.drain(..);

            let mut next1 = iter1.next();
            let mut next2 = iter2.next();

            while next1.is_some() && next2.is_some() {
                match next1.as_ref().unwrap().cmp(next2.as_ref().unwrap()) {
                    std::cmp::Ordering::Less => {
                        result.push(next1.take().unwrap());
                        next1 = iter1.next();
                    }
                    std::cmp::Ordering::Greater => {
                        result.push(next2.take().unwrap());
                        next2 = iter2.next();
                    }
                    std::cmp::Ordering::Equal => {
                        result.push(next1.take().unwrap());
                        result.push(next2.take().unwrap());
                        next1 = iter1.next();
                        next2 = iter2.next();
                    }
                }
            }

            if let Some(remaining) = next1 {
                result.push(remaining);
            } else if let Some(remaining) = next2 {
                result.push(remaining);
            }

            result.extend(iter1);
            result.extend(iter2);

            result
        }

        let vec_a = into_vec(list_a);
        let vec_b = into_vec(list_b);

        let vec_merged = merge_sorted_vecs(vec_a, vec_b);

        let mut target = LinkedList::<T>::new();
        for obj in vec_merged {
            target.add(obj);
        }

        target
	}
}

impl<T> Display for LinkedList<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.start {
            Some(node) => write!(f, "{}", unsafe { node.as_ref() }),
            None => Ok(()),
        }
    }
}

impl<T> Display for Node<T>
where
    T: Display,
{
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self.next {
            Some(node) => write!(f, "{}, {}", self.val, unsafe { node.as_ref() }),
            None => write!(f, "{}", self.val),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::LinkedList;

    #[test]
    fn create_numeric_list() {
        let mut list = LinkedList::<i32>::new();
        list.add(1);
        list.add(2);
        list.add(3);
        println!("Linked List is {}", list);
        assert_eq!(3, list.length);
    }

    #[test]
    fn create_string_list() {
        let mut list_str = LinkedList::<String>::new();
        list_str.add("A".to_string());
        list_str.add("B".to_string());
        list_str.add("C".to_string());
        println!("Linked List is {}", list_str);
        assert_eq!(3, list_str.length);
    }

    #[test]
    fn test_merge_linked_list_1() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![1,3,5,7];
		let vec_b = vec![2,4,6,8];
		let target_vec = vec![1,2,3,4,5,6,7,8];
		
		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
	#[test]
	fn test_merge_linked_list_2() {
		let mut list_a = LinkedList::<i32>::new();
		let mut list_b = LinkedList::<i32>::new();
		let vec_a = vec![11,33,44,88,89,90,100];
		let vec_b = vec![1,22,30,45];
		let target_vec = vec![1,11,22,30,33,44,45,88,89,90,100];

		for i in 0..vec_a.len(){
			list_a.add(vec_a[i]);
		}
		for i in 0..vec_b.len(){
			list_b.add(vec_b[i]);
		}
		println!("list a {} list b {}", list_a,list_b);
		let mut list_c = LinkedList::<i32>::merge(list_a,list_b);
		println!("merged List is {}", list_c);
		for i in 0..target_vec.len(){
			assert_eq!(target_vec[i],*list_c.get(i as i32).unwrap());
		}
	}
}