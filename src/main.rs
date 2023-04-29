mod house;

use std::collections::HashSet;
use crate::house::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider, SmartSocket, SmartThermometer, State};


fn main() {
    let house = house::SmartHouse::new(String::from("Smart House"));
    println!("{:?}", &house.get_rooms());
    println!("{:?}", &house.devices("Room"));

    // let socket = SmartSocket::new(String::from("socket"), State::On);
    let socket2 = SmartSocket::new(String::from("socket"), State::Off);
    let thermo = SmartThermometer::new(String::from("thermo"), String::from("25.0"));
    // let info_provider1 = OwningDeviceInfoProvider { socket };
    let info_provider2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    // let report1 = house.create_report(&info_provider1);
    let report2 = house.create_report(&info_provider2);

    // Выводим отчёты на экран:
    // println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}