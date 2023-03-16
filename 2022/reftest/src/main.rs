use std::{cell::RefCell, vec};

#[derive(Debug)]
struct Num_holder {
    num: i32,
}

impl Num_holder {
    fn add(&mut self, num: i32) {
        self.num += num;
    }
}

fn main() {

    let mut map = vec![];
    map.push(RefCell::new(Num_holder { num: 0 }));
    map.push(RefCell::new(Num_holder { num: 1 }));
    map.push(RefCell::new(Num_holder { num: 2 }));

    let holder_1 = map.get(0).unwrap();
    let holder_2 = map.get(0).unwrap();

    holder_1.borrow_mut().add(1);
    holder_2.borrow_mut().add(3);
    holder_1.borrow_mut().add(5);

    // convert number to binary
    let binary = format!("{:b}", holder_1.borrow().num);
    let binary = i32::
    println!("binary: {}", binary);
    // convert binary to number
    let num = i32::from_str_radix(&binary, 2).unwrap();
    println!("num: {}", num);


}
