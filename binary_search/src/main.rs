fn main() {
    let nums: Vec<char> = (0..=100).map(|num| char::from_u32(num).unwrap()).collect();
    println!("{}", nums[binary_search(&65.into(), &nums).unwrap()]);
}

fn binary_search<T: std::cmp::PartialEq + std::cmp::PartialOrd>(
    item: &T,
    list: &Vec<T>,
) -> Option<usize> {
    let mut low = 0;
    let mut high = list.len() - 1;
    while high >= low {
        let mid = (high + low) / 2;
        let guess = &list[mid];
        if guess == item {
            return Some(mid);
        } else if guess < item {
            low = mid + 1;
        } else {
            high = mid - 1;
        }
    }
    None
}
