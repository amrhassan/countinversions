
pub fn count<T: Ord+Clone>(elements: Vec<T>) -> u64 {
    let (_, count) = sort_and_count_inversions(elements);
    count
}

fn sort_and_count_inversions<T: Ord+Clone>(elements: Vec<T>) -> (Vec<T>, u64) {

    if elements.len() < 2 {
        (elements, 0)
    } else if elements.len() == 2 {
        if elements[1] < elements[0] {
            (vec![elements[1].clone(), elements[0].clone()], 1)
        } else {
            (elements, 0)
        }
    } else {
        let left_size = elements.len() / 2;
        let left: Vec<T> = elements.clone().into_iter().take(left_size).collect();
        let right: Vec<T> = elements.clone().into_iter().skip(left_size).collect();

        let (sorted_left, left_inversions) = sort_and_count_inversions(left);
        let (sorted_right, right_inversions) = sort_and_count_inversions(right);
        let (sorted, split_inversions) = merge_and_count_inversions(sorted_left, sorted_right);

        (sorted, left_inversions + right_inversions + split_inversions)
    }
}

fn merge_and_count_inversions<T: Ord+Clone>(left: Vec<T>, right: Vec<T>) -> (Vec<T>, u64) {
    let mut left_i: usize = 0;
    let mut right_i: usize = 0;
    let mut sorted: Vec<T> = Vec::new();
    let mut split_inversions: u64 = 0;

    while left_i < left.len() || right_i < right.len() {
        if left_i == left.len() {
            sorted.push(right[right_i].clone());
            right_i += 1;
        } else if right_i == right.len() {
            sorted.push(left[left_i].clone());
            left_i += 1
        } else if left[left_i] < right[right_i] {
            sorted.push(left[left_i].clone());
            left_i += 1;
        } else {
            sorted.push(right[right_i].clone());
            right_i += 1;
            split_inversions += (left.len() - left_i) as u64;
        }
    }

    (sorted, split_inversions)
}

#[test]
fn test_counting_1() {
    assert!(count(vec![1, 3, 5, 2, 4, 6]) == 3)
}

#[test]
fn test_counting_2() {
    assert!(count(vec![29, 10, 14, 37, 13]) == 5)
}

#[test]
fn test_counting_3() {
    assert!(count(vec![9, 8, 7, 6, 5, 4, 3, 2, 1, 0]) == 45)
}
