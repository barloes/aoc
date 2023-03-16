use std::{fs::File, io::BufReader, io::BufRead, cmp::Ordering};

use serde_json::Value;

fn main() {
    let mut packets = load_packet();
    let mut packets_iter = packets.iter();

    let mut counter = 0;
    let mut solution_1_ans = 0;
    while let Some(left_value) = packets_iter.next() {
        counter += 1;
        let right_value = packets_iter.next().unwrap();
        let result = compare_packets(left_value, right_value);
        if result == Ordering::Less {
            solution_1_ans += counter;
        }
    }    
    println!("solution 1: {}", solution_1_ans);

     // create new serde value
     // [[2]]
     // [[6]]
     let new_dividers: Vec<Value> = vec!(serde_json::from_str("[[2]]").unwrap(), serde_json::from_str("[[6]]").unwrap());
    packets.extend(new_dividers.clone());
    packets.sort_by(compare_packets);
    let index_1 = packets.iter().position(|x| x == &new_dividers[0]).unwrap() + 1;
    let index_2 = packets.iter().position(|x| x == &new_dividers[1]).unwrap() + 1;
    let solution_2_ans = index_2 * index_1;
    println!("solution 2: {}", solution_2_ans);
}

fn compare_packets(left: &Value, right: &Value) -> Ordering {
    // if left and right are both array
    // if left or right is array
    // if left and right are number
    // else panic

    if left.is_array() && right.is_array() {
        let l = left.as_array().unwrap();
        let r = right.as_array().unwrap();

        for (l_elem, r_elem) in l.iter().zip(r.iter()) {
            let result = compare_packets(l_elem, r_elem);
            if result != Ordering::Equal {
                return result;
            }
        }
        return l.len().cmp(&r.len());
    } else if left.is_array() || right.is_array() {
        let l = convert_to_value(left);
        let r = convert_to_value(right);
        return compare_packets(&l, &r);
    } else if left.is_number() && right.is_number() {
        let l = left.as_i64().unwrap();
        let r = right.as_i64().unwrap();
        return l.cmp(&r);
    } else {
        panic!();
    }
}

// convert int into Value of array
fn convert_to_value(value: &Value) -> Value {
    if value.is_array(){
        value.clone()
    } else{
        Value::Array(vec![value.clone()])
    }
}

fn load_packet() -> Vec<Value>{
    let file = File::open("input").unwrap();
    let lines = BufReader::new(file).lines();
    let mut packets = Vec::new();
    for line in lines {
        let line = line.unwrap();
        if line.is_empty() {
            continue;
        }
        let packet = serde_json::from_str(&line).unwrap();
        packets.push(packet)
    }
    packets
}