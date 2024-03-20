use std::{collections::HashMap, collections::HashSet};

fn main() {
    // vectors
    let mut v: Vec<i32> = Vec::new();
    v.push(1);
    v.push(2);
    println!("v: {:?}", v);

    print!("v[0]: {}", v.get(0).unwrap());
    println!("v[1]: {}", v[1]);

    // hashset, there is no duplicates
    let mut hs = HashSet::new();
    hs.insert(1);
    hs.insert(2);
    println!("hs: {:?}", hs);

    // hashmap
    let mut hm: HashMap<&str, i32> = HashMap::new();
    hm.insert("a", 1);
    hm.insert("b", 2);
    println!("hm: {:?}", hm);
}
