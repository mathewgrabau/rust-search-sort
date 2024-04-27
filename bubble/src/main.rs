use common;

fn bubble_sort(vec: &mut Vec<i32>) {
    // Floor the value.
    let mut end_index = vec.len().saturating_sub(1);
    while end_index > 0 {
        let mut new_end = 0;
        for i in 1..=end_index {
            // If we need to, move the update up there
            if vec[i - 1] > vec[i] {
                vec.swap(i - 1, i);
                new_end = i;
            }
        }

        end_index = new_end;
    }
}

fn main() {
    let result = common::get_i32("Hello, world - enter a number for the input: ");
    println!("you entered: {}", result);

    let mut data = common::make_random_vec(result, result.saturating_mul(100));
    bubble_sort(&mut data);

    common::print_vec(&data, data.len() as i32);
}

#[cfg(test)]
mod test {
    use super::bubble_sort;
    use common::check_sorted;

    #[test]
    fn it_handles_empty_array() {
        let mut test_data = Vec::new();
        bubble_sort(&mut test_data);
        let result = check_sorted(&test_data);
        assert_eq!(result, true);
    }

    #[test]
    fn it_handles_single_entry() {
        let mut test_data = vec![1; 1];
        bubble_sort(&mut test_data);
        let result = check_sorted(&test_data);
        assert_eq!(result, true);
    }

    #[test]
    fn it_sorts_sorted() {
        let mut test_data = vec![0; 3];
        bubble_sort(&mut test_data);
        let result = check_sorted(&test_data);
        assert_eq!(result, true);
    }

    #[test]
    fn it_sorts_array() {
        let mut test_data = vec![0; 3];
        for i in 0..3 {
            test_data[i] = 3 - i as i32;
        }

        bubble_sort(&mut test_data);
        let result = check_sorted(&test_data);
        assert_eq!(result, true);
    }
}
