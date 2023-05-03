pub mod room;

use crate::house::room::Room;
use crate::provider::DeviceInfoProvider;
use std::collections::HashMap;

pub struct SmartHouse {
    #[allow(dead_code)]
    name: String,
    rooms: Vec<Room>,
}

impl SmartHouse {
    pub fn new(name: String) -> Self {
        SmartHouse {
            name,
            rooms: Vec::new(),
        }
    }

    pub fn add_room(&mut self, room: Room) -> Option<bool> {
        let root_names = self
            .rooms
            .iter()
            .map(|r| r.get_name())
            .collect::<Vec<&str>>();

        if !root_names.contains(&room.get_name()) {
            self.rooms.push(room);
            return Some(true);
        }

        None
    }

    pub fn get_rooms(&self) -> [&str; 2] {
        let mut rooms: [&str; 2] = ["", ""];
        for (i, room) in self.rooms.iter().enumerate() {
            rooms[i] = room.name.as_str();
        }
        rooms
    }

    pub fn devices(&self, room: &str) -> [&str; 2] {
        let mut devices: [&str; 2] = ["", ""];
        for r in &self.rooms {
            if r.name.as_str() == room {
                for (i, device) in r.devices.iter().enumerate() {
                    devices[i] = device.as_str();
                }
            }
        }
        devices
    }

    pub fn create_report(&self, provider: &dyn DeviceInfoProvider) -> String {
        let mut room_devices_map = HashMap::new();
        let mut devices_vec = Vec::new();
        for room in &self.rooms {
            for device in &room.devices {
                if provider.contains(device) {
                    let provider_device = provider.get_info(room.name.as_str(), device);
                    devices_vec.push(provider_device)
                }
                room_devices_map.insert(room.name.clone(), devices_vec.clone());
            }
        }
        devices_vec
            .iter()
            .map(|d| d.clone() + "\n")
            .collect::<String>()
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

    #[test]
    fn can_add_room() {
        let mut house = SmartHouse::new(String::from("smart house"));
        let room = Room::new(String::from("room"), Vec::new());

        house.add_room(room);

        assert!(house.get_rooms().contains(&"room"));
    }

    #[test]
    fn cannot_add_room_with_same_name() {
        let mut house = SmartHouse::new(String::from("smart house"));
        let room1 = Room::new(String::from("room"), Vec::new());
        let room2 = Room::new(String::from("room"), Vec::new());

        house.add_room(room1);
        house.add_room(room2);

        assert_eq!(house.get_rooms().first(), Some(&"room"));
        assert_eq!(house.get_rooms().last(), Some(&""));
    }

    #[test]
    fn create_report() {
        let socket = SmartSocket::new(String::from("socket"), State::On);
        let socket2 = SmartSocket::new(String::from("socket2"), State::Off);
        let thermo = SmartThermometer::new(String::from("thermo"), "25".to_string());
        let room1 = Room::new("room".to_string(), vec!["socket".to_string()]);
        let room2 = Room::new(
            "room2".to_string(),
            vec!["thermo".to_string(), "socket2".to_string()],
        );
        let mut house = SmartHouse::new(String::from("Smart House:"));
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
