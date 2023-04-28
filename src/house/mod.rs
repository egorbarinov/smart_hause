use std::collections::{HashMap, HashSet};

#[derive(Debug)]
pub struct SmartHouse {
    name: String,
    rooms: Vec<Room>,
}

#[derive(Debug)]
struct Room {
    name: String,
    devices: Vec<String>,
}

impl SmartHouse {
    pub fn new(name: String) -> Self {
        let room = Room::new(
            "Room".to_string(),
            Vec::from_iter(vec!["socket".to_string()]),
        );
        SmartHouse {
            name,
            rooms: vec![room],
        }
    }

    pub fn get_rooms(&self) -> [&str; 1] {
        let rooms = self.rooms
            .iter()
            .map(|r| r.name.as_str())
            .collect::<Vec<&str>>();
        let slice = rooms.as_slice();
        let array = match slice.try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Expected a Vec of length {} but it was {}", 1, &rooms.len()),
        };
        array
    }

    pub fn devices(&self, room: &str) -> [&str; 1] {
        let mut devices = Vec::new();
        for r in &self.rooms {
            if r.name.as_str() == room {
                devices = r.devices.iter().map(|d| d.as_str()).collect::<Vec<&str>>();
            }
        }
        let slice = devices.as_slice();
        let array = match slice.try_into() {
            Ok(arr) => arr,
            Err(_) => panic!("Expected a Vec of length {} but it was {}", 1, &devices.len()),
        };
        array
    }

    pub fn create_report(&self, provider: &dyn DeviceInfoProvider) -> String {
        let mut device_report: String = "".to_string();
        let mut room_devices_map = HashMap::new();

        for room in &self.rooms {
            let mut devices_vec = Vec::new();
            for device in &room.devices {
                if provider.provider_contains(device) {
                    let provider_device = provider.get_info(room.name.as_str(), device);
                    devices_vec.push(provider_device)
                }
                room_devices_map.insert(room.name.clone(), devices_vec.clone());
            }
        }

        for (room, devices) in room_devices_map.iter() {
            device_report = devices.iter()
                .map(|device| String::from(device.to_owned() + "\n"))
                .collect();
        }
        device_report
    }
}

impl Room {
    pub fn new(name: String, devices: Vec<String>) -> Self {
        Room { devices, name }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_ref()
    }

    pub fn get_devices(&self) -> &Vec<String> {
        &self.devices
    }
}

///////////////////////////////////////////////////////////////////////////////

pub trait Device {
    fn get_name(&self) -> &str;
    fn create_report(&self) -> String;
}

#[derive(Debug, Copy, Clone)]
pub enum State {
    Off,
    On,
}

pub struct SmartSocket {
    name: String,
    state: State,
}

impl SmartSocket {
    pub fn new(name: String, state: State) -> Self { SmartSocket { name, state } }
}

pub struct SmartThermometer {
    name: String,
    temperature: String,
}

impl SmartThermometer {
    pub fn new(name: String, temperature: String) -> Self { SmartThermometer { name, temperature } }
}

impl Device for SmartSocket {
    fn get_name(&self) -> &str {
        &self.name.as_ref()
    }

    fn create_report(&self) -> String {
        format!("SmartSocket: {}, state is {:?}", &self.name, &self.state)
    }
}

impl Device for SmartThermometer {
    fn get_name(&self) -> &str {
        &self.name.as_ref()
    }

    fn create_report(&self) -> String {
        format!("Thermometer: {}, temperature is {:?}", self.name, self.temperature)
    }
}

pub trait DeviceInfoProvider {
    fn provider_contains(&self, device_name: &String) -> bool;
    fn get_devices(&self) -> Vec<&dyn Device>;
    fn get_info(&self, room: &str, device: &str) -> String;
}

pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}
//
// struct BorrowingDeviceInfoProvider<'a, 'b> {
//     socket: &'a SmartSocket,
//     thermo: &'b SmartThermometer,
// }
//
// impl BorrowingDeviceInfoProvider {
// }

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn provider_contains(&self, device_name: &String) -> bool {
        if self.get_devices()
            .iter()
            .find(|device| device.get_name() == device_name).is_some() {
            true
        } else {
            false
        }
    }

    fn get_devices(&self) -> Vec<&dyn Device> {
        vec![&self.socket as &dyn Device]
    }

    fn get_info(&self, room: &str, device: &str) -> String {
        if self.socket.get_name() == device {
            (format!("Room: {}, Device {}", room, self.socket.create_report()))
        } else {
            "".to_string()
        }
    }
}

// impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
//     fn get_info(&self, room: &str, device: &str) -> Option<String> {
//         let devices: HashMap<&str, &dyn Device> = HashMap::from([
//             (self.socket.get_name(), self.socket as &dyn Device),
//             (self.thermo.get_name(), self.thermo as &dyn Device),
//         ]);
//
//         extract_info(room, device, devices)
//     }
//
//     fn required_devices(&self) -> Vec<&dyn Device> {
//         vec![self.socket, self.thermo]
//     }
// }

// impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
//     fn get_name(&self) -> String {
//         if self.socket
//         todo!()
//     }
//
//     fn get_info(&self, room: &str, device: &str) -> String {
//         todo!()
//     }
// }
