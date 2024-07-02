fn main() {
    let list = [2, 1, 5, 14];
    let sorted_list = [1, 2, 5, 14];
    assert_eq!(quicksort(Vec::from(list)), Vec::from(sorted_list));
}

fn quicksort<T: std::cmp::PartialOrd + std::clone::Clone>(list: Vec<T>) -> Vec<T> {
    if list.len() < 2 {
        list
    } else {
        let pivot = &list[0];
        let less: Vec<T> = list.clone().into_iter().filter(|x| x.lt(&pivot)).collect();
        let greater: Vec<T> = list.clone().into_iter().filter(|x| x.gt(&pivot)).collect();
        [
            quicksort(less).as_slice(),
            &[pivot.clone()],
            quicksort(greater).as_slice(),
        ]
        .concat()
    }
}
