use std::collections::HashSet;

struct Foo {}

fn main() {
    let foo = Foo {};
    let mut set: HashSet<i32> = HashSet::new();
    let mut map: std::collections::HashMap<i32, i32> = std::collections::HashMap::new();

    // prelude
    let option: Option<i32> = None; // Option<T> is imported automatically from "use std::prelude::v1::*;"
}
