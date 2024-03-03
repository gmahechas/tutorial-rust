struct Counter {
    count: i32,
    max: i32,
}

impl Counter {
    fn new(max: Option<i32>) -> Counter {
        Counter { count: 0, max: max.unwrap_or(7) }
    }
}

impl Iterator for Counter {
    type Item = i32;
    fn next(&mut self) -> Option<Self::Item> {
        if self.count < self.max {
            self.count += 1;
            Some(self.count)
        } else {
            None
        }
    }
}

fn main() {
    // arrays
    let array1: [i32; 3] = [1, 2, 3]; // this is an array
    for i in array1.iter() {
        println!("{}", i);
    }

    // slices
    let slice1: &[i32] = &[4, 5, 6];
    for i in slice1.iter() {
        println!("{}", i);
    }

    // vetors, ideal for list that change in execution time
    let vector1: Vec<i32> = vec![7, 8, 9];
    for i in vector1.iter() {
        println!("{}", i);
    }

    // hashset, ideal for unorder sets and remove duplicates
    let hashset1: std::collections::HashSet<i32> = [10, 11, 12].iter().cloned().collect();
    for i in hashset1.iter() {
        println!("{}", i);
    }

    // hashmap, ideal for key-value pairs
    let hashmap1: std::collections::HashMap<i32, i32> = [(13, 14), (15, 16)].iter().cloned().collect();
    for i in hashmap1.iter() {
        println!("{:?}", i);
    }

    // range
    let range1: std::ops::Range<i32> = 17..20;
    for i in range1 {
        println!("{}", i);
    }

    // iterators
    let counter: Counter = Counter::new(Some(8));
    for i in counter {
        println!("{}", i);
    }
}
