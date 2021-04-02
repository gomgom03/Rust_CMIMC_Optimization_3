use itertools::Itertools;

#[derive(Debug)]
struct Person {
    name: String,
    age: u32,
}

pub fn run() {
    let t_vec = vec![
        (0,1),(0,0),(0,1),(0,0),(0,-1)
    ];

    let v: Vec<_> = t_vec.into_iter().unique().collect();
    println!("{:?}", v);
}