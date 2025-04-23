/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

use std::mem::swap;

fn sort<T: Ord>(array: &mut [T]){
    array.sort();
}

fn bubble_sort<T: Ord>(array: &mut [T]){
    for i in (1..array.len()).rev() {
        for j in 0..i {
            if array[j] > array[j+1] {
                array.swap(j, j+1)
            }
        }
    }
}

fn selection_sort<T: Ord>(array: &mut [T]){
    for i in 0..array.len() {
        // array[0..i] has already been sorted as the smallest items.
        // The following code makes array[i] be the smallest one.
        for j in i..array.len() {
            if array[i] > array[j] {
                array.swap(i, j);
            }
        }
        // Now array[i] has been the minimum of array[i..], and a[0..=i]has been sorted.
    }
}

fn insertion_sort<T: Ord>(array: &mut [T]){
    for i in 0..array.len() {
        // Insert array[i] into sorted array[0..i], to make array[0..=i] sorted.
        for j in (0..i).rev() {
            if array[j] < array[j+1] {
                break;
            } else {
                array.swap(j, j+1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sort_1() {
        let mut vec = vec![37, 73, 57, 75, 91, 19, 46, 64];
        sort(&mut vec);
        assert_eq!(vec, vec![19, 37, 46, 57, 64, 73, 75, 91]);
    }
	#[test]
    fn test_sort_2() {
        let mut vec = vec![1];
        sort(&mut vec);
        assert_eq!(vec, vec![1]);
    }
	#[test]
    fn test_sort_3() {
        let mut vec = vec![99, 88, 77, 66, 55, 44, 33, 22, 11];
        sort(&mut vec);
        assert_eq!(vec, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]);
    }
}