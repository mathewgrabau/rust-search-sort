use std::iter::once;

use common;

fn linear_search(data: &Vec<i32>, target: i32) -> (i32, i32) {
    for (index, val) in data.into_iter().enumerate() {
        if *val == target {
            return (index as i32, (index + 1) as i32);
        }
    }

    return (-1, data.len() as i32);
}

fn main() {
    let num_items = common::get_i32("Items: ");
    let max = common::get_i32("Max: ");

    // Print the generated array now
    let data = common::make_random_vec(num_items, max);
    common::print_vec(&data, num_items);

    loop {
        let target = common::get_i32("Target (-1 to quit)");
        if target == -1 {
            break;
        }

        let (index, num_tests) = linear_search(&data, target);

        if index >= 0 {
            let from_vec = data[index as usize];
            std::println!("numbers[{index}] = {from_vec}, {num_tests} tests");
            continue;
        }

        std::println!("Target {index} not found, {num_tests} tests");
    }
}
