use crate::provider::DeviceInfoProvider;
use std::collections::HashMap;
use crate::house::room::Room;

pub mod room;

pub struct SmartHouse {
    #[allow(dead_code)]
    name: String,
    rooms: Vec<Room>,
}

impl SmartHouse {
    pub fn new(name: String) -> Self {
        let room = Room::new(
            "Room".to_string(),
            vec!["socket".to_string(), "thermo".to_string()],
        );
        SmartHouse {
            name,
            rooms: vec![room],
        }
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
