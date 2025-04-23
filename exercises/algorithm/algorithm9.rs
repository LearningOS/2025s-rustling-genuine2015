/*
	heap
	This question requires you to implement a binary heap function
*/

use std::cmp::{Ord, Ordering};
use std::default::Default;
use std::fmt::{write, Display, Formatter};

pub struct Heap<T>
where
    T: Default,
{
    count: usize,
    items: Vec<T>,
    comparator: fn(&T, &T) -> bool,
}

impl<T> Heap<T>
where
    T: Default,
{
    pub fn new(comparator: fn(&T, &T) -> bool) -> Self {
        Self {
            count: 0,
            items: vec![T::default()],
            comparator,
        }
    }

    pub fn len(&self) -> usize {
        self.count
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    pub fn add(&mut self, value: T) {
        //TODO
        self.items.push(value);
        self.count += 1;

        let mut bubble = self.count;
        while bubble != 1 {
            let parent = bubble >> 1;
            if (self.comparator)(&self.items[bubble], &self.items[parent]) {
                self.items.swap(bubble, parent);
                bubble = parent;
            } else {
                break;
            }
        }
    }

    fn parent_idx(&self, idx: usize) -> usize { idx / 2 }

    fn present(&self, idx: usize) -> bool { idx <= self.count }

    fn children_present(&self, idx: usize) -> bool { self.left_child_idx(idx) <= self.count }

    fn left_child_idx(&self, idx: usize) -> usize { idx << 1 }

    fn right_child_idx(&self, idx: usize) -> usize { idx << 1 | 1 }

    fn smallest_child_idx(&self, idx: usize) -> usize {
        match self.count.cmp(&(idx << 1)) {
            Ordering::Less => panic!("No children present"), // to prevent misuse of self.items[0]
            Ordering::Equal => idx << 1,
            Ordering::Greater => if (self.comparator)(&self.items[idx << 1], &self.items[idx << 1 | 1]){
                idx << 1
            } else {
                idx << 1 | 1
            }
        }
    }
}

impl<T> Heap<T>
where
    T: Default + Ord,
{
    /// Create a new MinHeap
    pub fn new_min() -> Self {
        Self::new(|a, b| a < b)
    }

    /// Create a new MaxHeap
    pub fn new_max() -> Self {
        Self::new(|a, b| a > b)
    }
}

impl<T> Iterator for Heap<T>
where
    T: Default,
{
    type Item = T;

    fn next(&mut self) -> Option<T> {
        //TODO
        if self.is_empty() {
            return None;
        }

        let ret = self.items.swap_remove(1);
        self.count -= 1;

        let mut stone = 1;
        while self.children_present(stone) {
            let smallest_child_idx = self.smallest_child_idx(stone);
            if (self.comparator)(&self.items[smallest_child_idx], &self.items[stone]) {
                self.items.swap(stone, smallest_child_idx);
                stone = smallest_child_idx;
            } else {
                break;
            }
        }

        Some(ret)
    }
}

pub struct MinHeap;

impl MinHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a < b)
    }
}

pub struct MaxHeap;

impl MaxHeap {
    #[allow(clippy::new_ret_no_self)]
    pub fn new<T>() -> Heap<T>
    where
        T: Default + Ord,
    {
        Heap::new(|a, b| a > b)
    }
}

impl<T> Display for Heap<T> where T: Default + std::fmt::Debug {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}", &self.items)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_empty_heap() {
        let mut heap = MaxHeap::new::<i32>();
        assert_eq!(heap.next(), None);
    }

    #[test]
    fn test_min_heap() {
        let mut heap = MinHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(2));
        assert_eq!(heap.next(), Some(4));
        assert_eq!(heap.next(), Some(9));
        heap.add(1);
        assert_eq!(heap.next(), Some(1));
    }

    #[test]
    fn test_max_heap() {
        let mut heap = MaxHeap::new();
        heap.add(4);
        heap.add(2);
        heap.add(9);
        heap.add(11);
        assert_eq!(heap.len(), 4);
        assert_eq!(heap.next(), Some(11));
        assert_eq!(heap.next(), Some(9));
        assert_eq!(heap.next(), Some(4));
        heap.add(1);
        assert_eq!(heap.next(), Some(2));
    }
}