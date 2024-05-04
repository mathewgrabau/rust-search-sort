use std::fmt;

use common::check_sorted;
use common::make_random_vec_of;
use common::print_vec;

// The types and stuff

#[derive(Debug, Ord)]
struct Customer {
    id: String,
    num_purchases: i32,
}

impl fmt::Display for Customer {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({} - {})", self.id, self.num_purchases)
    }
}

impl PartialEq for Customer {
    fn eq(&self, other: &Self) -> bool {
        return self.id == other.id;
    }
}

impl Eq for Customer {}

impl PartialOrd for Customer {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let comparison = self.num_purchases - other.num_purchases;
        if comparison < 0 {
            return Some(std::cmp::Ordering::Less);
        }
        if comparison > 0 {
            return Some(std::cmp::Ordering::Greater);
        }

        return Some(std::cmp::Ordering::Equal);
    }
}

fn generate_customer(index: i32, current_value: i32) -> Customer {
    return Customer {
        id: format!("C{}", index),
        num_purchases: current_value,
    };
}

// The sorting
fn counting_sort(data: &Vec<Customer>) -> Vec<&Customer> {
    if data.is_empty() {
        return vec![];
    }
    let mut sorted: Vec<Option<&Customer>> = vec![None; data.len()];
    let max_count = data.iter().map(|c| c.num_purchases).max().unwrap();
    let mut counts = vec![0i32; (max_count + 1) as usize];

    // Count the number of occurrences of each purchases
    for i in 0..data.len() {
        let current = data[i].num_purchases as usize;
        counts[current] += 1;
    }

    // The counts array now has the cumulative number of items.
    for i in 1..counts.len() {
        counts[i] = counts[i] + counts[i - 1];
    }

    // Loop through the origianl data in reverse.
    for customer in data.iter() {
        let location = counts[customer.num_purchases as usize] - 1;
        counts[customer.num_purchases as usize] -= 1;
        sorted[location as usize] = Some(customer);
    }

    sorted
        .into_iter()
        .map(|c| {
            return c.unwrap();
        })
        .collect()
}

fn main() {
    let customers: Vec<Customer> = make_random_vec_of(10, 9999, generate_customer);
    println!("customers: {customers:?}");
    let sorted_items = counting_sort(&customers);
    print_vec(&sorted_items, 10);
    let is_sorted = check_sorted(&sorted_items);
    println!("is_sorted: {is_sorted}");
}

#[cfg(test)]
mod test {
    #[test]
    fn it_sorts_empty() {}
}
