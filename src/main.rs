mod house;

use std::collections::HashSet;
use crate::house::{OwningDeviceInfoProvider, SmartSocket, State};


fn main() {
    let house = house::SmartHouse::new(String::from("Smart House"));
    println!("{:?}", &house.get_rooms());
    println!("{:?}", &house.devices("Room"));

    let socket = SmartSocket::new(String::from("socket"), State::On);
    let info_provider_1 = OwningDeviceInfoProvider { socket };
    let report1 = house.create_report(&info_provider_1);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    // // println!("Report #2: {report2}");
}