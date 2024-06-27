fn main() {
    println!("{:?}", selection_sort(&mut vec![100, 93, 20, 53, 143, 25]));
}

fn find_smallest<T: std::cmp::PartialOrd + std::cmp::PartialEq>(arr: &[T]) -> usize {
    let mut smallest = &arr[0];
    let mut smallest_index = 0;
    for i in 1..arr.len() {
        if smallest > &arr[i] {
            smallest = &arr[i];
            smallest_index = i;
        }
    }
    smallest_index
}

fn selection_sort<T: std::cmp::PartialOrd + std::cmp::PartialEq + Copy>(
    arr: &mut Vec<T>,
) -> Vec<T> {
    let mut new_arr = Vec::new();
    for _ in 0..arr.len() {
        let smallest_index = find_smallest(arr);
        new_arr.push(arr[smallest_index]);
        arr.remove(smallest_index);
    }
    new_arr
}
