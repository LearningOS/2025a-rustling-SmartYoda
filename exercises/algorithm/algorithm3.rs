/*
	sort
	This problem requires you to implement a sorting algorithm
	you can use bubble sorting, insertion sorting, heap sorting, etc.
*/

fn sort<T>(array: &mut [T])
where
    T: PartialOrd + Clone,
{
	let n = array.len();
    if n < 2 {
        return;
    }
    sort(&mut array[..n/2]);
    sort(&mut array[n/2..]);
    let mut p1 = 0;
    let mut p2 = n/2;
    let mut v = vec![];
    while p1 < n/2 && p2 < n {
        if array[p1] <= array[p2]{
            v.push(array[p1].clone());
            p1 += 1;
        } else if array[p1] > array[p2] {
            v.push(array[p2].clone());
            p2 += 1;
        }
    }
    while p1 < n/2 {
        v.push(array[p1].clone());
        p1 += 1;
    }
    while p2 < n {
        v.push(array[p2].clone());
        p2 += 1;
    }
    array.clone_from_slice(&v);
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