mod device;
mod house;
mod provider;

use crate::device::{Device, SmartSocket, SmartThermometer, State};
use crate::house::room::Room;
use crate::provider::{BorrowingDeviceInfoProvider, OwningDeviceInfoProvider};
use std::collections::HashSet;

fn main() {
    let mut house = house::SmartHouse::new("Smart House".into());
    let socket = SmartSocket::new("socket".into(), State::On);
    println!("{}", socket.create_report());
    let socket2 = SmartSocket::new("socket2".into(), State::Off);
    println!("{}", socket2.create_report());
    let thermo = SmartThermometer::new("thermo".into(), "25.0".into());
    let mut room = Room::new("room".into(), HashSet::new());
    room.devices.insert(socket.name.clone());
    let mut room2 = Room::new("room2".into(), HashSet::new());
    room2.devices.insert(socket2.name.clone());
    room.devices.insert(thermo.name.clone());
    house.add_room(room);
    house.add_room(room2);

    println!("{:?}", &house.devices("room"));
    println!("{:?}", &house.devices("room2"));
    println!("{}", thermo.create_report());
    let info_provider1 = OwningDeviceInfoProvider { socket };
    let info_provider2 = BorrowingDeviceInfoProvider {
        socket: &socket2,
        thermo: &thermo,
    };
    let report1 = house.create_report(&info_provider1);
    let report2 = house.create_report(&info_provider2);

    // Выводим отчёты на экран:
    println!("Report #1: {report1}");
    println!("Report #2: {report2}");
}
