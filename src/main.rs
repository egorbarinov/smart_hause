mod house;

use std::collections::HashSet;



fn main() {
    let house = house::SmartHouse::new(String::from("Smart House"));
    println!("{:?}", &house.get_rooms());
    println!("{:?}", &house.devices("Room"));
}