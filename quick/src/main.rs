use common;

fn quicksort(data: &mut [i32]) {
    if data.len() == 0 {
        return;
    }
    let p = partition(data);
    if p > 0 {
        quicksort(&mut data[0..p]);
    }
    if p + 1 < data.len() {
        quicksort(&mut data[p + 1..]);
    }
}

fn partition(part: &mut [i32]) -> usize {
    // Choose the last element as the pivot.
    let pivot: i32 = *(part.last().unwrap());

    // Temporary pivot index
    let mut current_swap = 0;

    let stop = part.len().saturating_sub(1);
    for index in 0..stop {
        if part[index] <= pivot {
            // Swap the current element with the element at the temporary pivot index
            part.swap(current_swap, index);
            // Move the temporary pivot index forward.
            current_swap += 1;
        }
    }
    // Swap the last element with the pivot index
    if current_swap < part.len() {
        part.swap(current_swap, part.len() - 1);
    }
    return current_swap;
}

fn main() {
    let num_elements = common::get_i32("Enter the number of elements to generate: ");
    let mut test_data = common::make_random_vec(num_elements, 12);
    common::print_vec(&test_data, test_data.len() as i32);
    quicksort(&mut test_data.as_mut_slice());
    common::print_vec(&test_data, test_data.len() as i32);
}

#[cfg(test)]
mod test {
    use super::quicksort;
    use common;

    #[test]
    fn it_sorts_empty() {
        let mut test_data = Vec::new();
        quicksort(&mut test_data);
        let result = common::check_sorted(&mut test_data);
        assert_eq!(result, true);
    }

    #[test]
    fn it_sorts_single() {
        let mut test_data = Vec::new();
        test_data.push(1);
        quicksort(&mut test_data);
        let result = common::check_sorted(&mut test_data);
        assert_eq!(result, true);
    }

    #[test]
    fn it_sorts() {
        let mut test_data = Vec::new();
        test_data.push(3);
        test_data.push(2);
        test_data.push(1);
        test_data.push(4);
        quicksort(&mut test_data);
        let result = common::check_sorted(&mut test_data);
        assert_eq!(result, true);
    }
}
