fn main() {
    let list = [10, 102, 4, 52, 2, 16];
    let vec_list = vec![list];

    dbg!(sum(&list));
    dbg!(num_of_items_in_list(&list, None));
    dbg!(max_num_in_list(&list, None));
    dbg!(recursive_binary_search(&vec_list));
}

// exercise 4.1: write out the code for the earlier SUM function
// Grokking algorithms: page 118
fn sum(num_list: &[usize]) -> usize {
    if num_list.len() == 0 {
        0
    } else {
        num_list[0] + sum(&num_list[1..])
    }
}

// exercise 4.2: write a recursive function to count the number of items in a list.
// Grokking algorithms: page 123
fn num_of_items_in_list<T>(list: &[T], sum: Option<usize>) -> usize {
    match list.get(0) {
        None => sum.unwrap(),
        Some(_) => num_of_items_in_list(&list[1..], Some(sum.unwrap_or(0) + 1)),
    }
}

// exercise 4.3: find the maximum number in a list
// Grokking algorithms: page 123
fn max_num_in_list(list: &[usize], num: Option<usize>) -> usize {
    match list.get(0) {
        None => num.unwrap(),
        Some(new_num) => {
            if new_num > &num.unwrap_or(0) {
                max_num_in_list(&list[1..], Some(*new_num))
            } else {
                max_num_in_list(&list[1..], Some(num.unwrap()))
            }
        }
    }
}

// exercise 4.4: Can you come up with a basecase and recursive case for binary search?
// Grokking algorithms: page 123
fn recursive_binary_search<T>(_list: &Vec<T>) -> Vec<T> {
    todo!("recursive_binary_search");
}
