use std::cmp::max;
use std::fs::File;
use std::io::{ self, BufRead, BufReader };

fn main() {

    let mut mx = 0;
    let mut cur_elf_food = 0;
    for line in read_lines("./input".to_string()) {
        let temp = line.unwrap();
        match temp.parse::<i32>()  {
            Ok(food_amt) =>{
                cur_elf_food += food_amt;
            }
            Err(_) => {
                mx = max(mx, cur_elf_food);
                cur_elf_food = 0;
            }
        }
    }

    println!("{}", mx);

}

fn read_lines(filename: String) -> io::Lines<BufReader<File>> {
    // Open the file in read-only mode.
    let file = File::open(filename).unwrap();
    // Read the file line by line, and return an iterator of the lines of the file.
    return io::BufReader::new(file).lines();
}