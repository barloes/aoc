use std::{
    fs::File,
    io::{BufRead, BufReader},
};

fn main() {
    // read file
    let filename = "./input";
    let file = File::open(filename).unwrap();
    let reader = BufReader::new(file);
    let mut ans = 0;
    for line in reader.lines() {
        let query = line
            .unwrap()
            .split(",")
            .map(|x| {
                x.split("-")
                    .map(|x| x.parse::<i32>().unwrap())
                    .collect::<Vec<i32>>()
            })
            .collect::<Vec<Vec<i32>>>();

            if is_overlapping(query[0].clone(), query[1].clone()) {
                println!("{} {} {} {}", query[0][0], query[0][1], query[1][0], query[1][1]);
                ans += 1;
            } 
    }
    println!("{}", ans);
}

fn is_bounded(left: Vec<i32>, right: Vec<i32>) -> bool {
    let a = left[0];
    let b = left[1];

    let c = right[0];
    let d = right[1];
    if (a <= c && b >= d) | (c <= a && d >= b) {
        return true;
    }
    false
}

fn is_overlapping(left: Vec<i32>, right: Vec<i32>) -> bool {
    let a = left[0];
    let b = left[1];

    let c = right[0];
    let d = right[1];
    if (b >= c && b <= d) | (a >= c && a <= d) 
        | (d >= a && d <= b) | (c >= a && c <= b)    {
        return true;
    }
    false
}