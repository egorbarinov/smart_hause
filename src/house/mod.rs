pub mod room;

use crate::house::room::Room;
use crate::provider::DeviceInfoProvider;
use std::collections::hash_map::Entry::Vacant;
use std::collections::{HashMap, HashSet};

pub struct SmartHouse {
    #[allow(dead_code)]
    name: String,
    rooms: HashMap<String, Room>,
}

impl SmartHouse {
    pub fn new(name: String) -> Self {
        SmartHouse {
            name,
            rooms: HashMap::new(),
        }
    }

    pub fn add_room(&mut self, room: Room) -> Option<bool> {
        if let Vacant(e) = self.rooms.entry(room.get_name().to_string()) {
            e.insert(room);
            return Some(true);
        }
        None
    }

    pub fn get_rooms(&self) -> &HashMap<String, Room> {
        &self.rooms
    }

    pub fn devices(&self, room: &str) -> &HashSet<String> {
        self.rooms.get(room).unwrap().devices()
    }

    pub fn create_report(&self, provider: &dyn DeviceInfoProvider) -> String {
        let mut report: String = "".into();
        let _ = &self.get_rooms().iter().for_each(|(room_name, room)| {
            for device in &room.devices {
                if provider.contains(device) {
                    let device_info = provider.get_info(room_name, device);
                    report.push_str(&device_info);
                    report.push('\n');
                }
            }
        });
        report
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::device::SmartThermometer;
    use crate::provider::BorrowingDeviceInfoProvider;
    use crate::{
        device::{SmartSocket, State},
        provider::OwningDeviceInfoProvider,
    };
    use std::collections::HashSet;

    #[test]
    fn can_add_room() {
        let mut house = SmartHouse::new(String::from("smart house"));
        let room = Room::new(String::from("room"), HashSet::new());

        house.add_room(room);

        assert!(house.get_rooms().contains_key("room"));
    }

    #[test]
    fn cannot_add_room_with_same_name() {
        let mut house = SmartHouse::new("smart house".into());
        let room1 = Room::new("room".into(), HashSet::new());
        let room2 = Room::new("room".into(), HashSet::new());

        house.add_room(room1);
        house.add_room(room2);

        assert_eq!(house.get_rooms().len(), 1);
    }

    #[test]
    fn create_report() {
        let socket = SmartSocket::new("socket".into(), State::On);
        let socket2 = SmartSocket::new("socket2".into(), State::Off);
        let thermo = SmartThermometer::new("thermo".into(), "25".into());
        let mut room1 = Room::new("room".into(), HashSet::new());
        room1.devices.insert(socket.name.to_string().clone());
        let mut room2 = Room::new("room2".into(), HashSet::new());
        room2.devices.insert(socket2.name.to_string().clone());
        room2.devices.insert(thermo.name.to_string().clone());
        let mut house = SmartHouse::new("Smart House".into());
        house.add_room(room1);
        house.add_room(room2);
        let info_provider = OwningDeviceInfoProvider { socket };
        let info_provider2 = BorrowingDeviceInfoProvider {
            socket: &socket2,
            thermo: &thermo,
        };

        let report1 = house.create_report(&info_provider);
        let report2 = house.create_report(&info_provider2);

        assert_eq!(
            report1,
            "Room: room, Device SmartSocket: socket, state is On\n".to_string()
        );
        assert!(report2.contains("Room: room2, Device SmartSocket: socket2, state is Off\n"));
        assert!(
            report2.contains("Room: room2, Device Thermometer: thermo, temperature is \"25\"\n")
        );
    }
}
