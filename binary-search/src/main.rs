use common;

fn binary_search(data: &Vec<i32>, target: i32) -> (i32, i32) {
    let mut low = 0 as usize;
    let mut high = data.len() - 1;
    let mut worker_count: i32 = 0;
    // Then if we are

    while low <= high {
        worker_count += 1;

        // Calculate our current search point
        let middle = low + (high - low) / 2;

        if data[middle] == target {
            return (middle as i32, worker_count);
        }

        // If x is greater, then ignore the left half
        if data[middle] < target {
            low = middle + 1;
        } else {
            // If x is smaller, then ignore the left half
            high = middle - 1;
        }
    }

    return (-1, worker_count);
}

fn main() {
    let num_items = common::get_i32("Items: ");
    let max = common::get_i32("Max: ");

    // Print the generated array now
    let mut data = common::make_random_vec(num_items, max);
    common::print_vec(&data, num_items);

    // Sorting the vector myself.
    data.sort();

    loop {
        let target = common::get_i32("Target (-1 to quit)");
        if target == -1 {
            break;
        }

        let (index, num_tests) = binary_search(&data, target);

        if index >= 0 {
            let from_vec = data[index as usize];
            std::println!("numbers[{index}] = {from_vec}, {num_tests} tests");
            continue;
        }

        std::println!("Target {index} not found, {num_tests} tests");
    }
}
