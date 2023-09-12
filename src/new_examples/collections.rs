use std::collections::{HashMap, HashSet};

fn main() {
    hash();
    set();
}

fn hash() {
    let mut map = HashMap::new();
    map.insert("foo", 100);
    map.insert("bar", 200);

    println!("{:?}", map);

    for (&k, &v) in map.iter() {
        println!("{}: {}", k, v);
    }
}

fn set() {
    let mut a: HashSet<i32> = vec![10, 20, 30, 40].into_iter().collect();
    let b: HashSet<i32> = vec![10, 30, 50].into_iter().collect();
    a.insert(100);

    println!("a: {:?}", a);
    println!("b: {:?}", b);
    println!("a - b: {:?}", a.difference(&b).collect::<Vec<&i32>>());
}
