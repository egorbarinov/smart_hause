pub mod room;

use crate::house::room::Room;
use crate::provider::DeviceInfoProvider;
use std::collections::hash_map::Entry::Vacant;
use std::collections::{HashMap, HashSet};
use std::error::Error;
use std::fmt;
use std::fmt::{Display, Formatter};

pub struct SmartHouse {
    #[allow(dead_code)]
    name: String,
    rooms: HashMap<String, Room>,
}

#[derive(PartialEq, Debug)]
pub enum ReportError {
    NoInfoProvided,
    RoomsNotFound,
}

impl Error for ReportError {}

impl Display for ReportError {
    fn fmt(&self, f: &mut Formatter) -> fmt::Result {
        match self {
            Self::NoInfoProvided => write!(f, "Devices not found"),
            Self::RoomsNotFound => write!(f, "Rooms not found"),
        }
    }
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

    pub fn get_rooms(&self) -> Option<&HashMap<String, Room>> {
        if self.rooms.is_empty() {
            None
        } else {
            Some(&self.rooms)
        }
    }

    pub fn devices(&self, room: &str) -> Option<&HashSet<String>> {
        self.rooms.get(room).unwrap().devices()
    }

    pub fn create_report(&self, provider: &dyn DeviceInfoProvider) -> Result<String, ReportError> {
        match &self.get_rooms() {
            None => Err(ReportError::RoomsNotFound),
            Some(rooms) => {
                let mut report: String = "".to_string();
                rooms.iter().for_each(|(room_name, room)| {
                    if let Some(devices) = room.devices() {
                        devices.iter().for_each(|device| {
                            if let Some(device_info) = provider.get_info(room_name, device) {
                                report.push_str(&device_info);
                                report.push('\n');
                            }
                        })
                    }
                });
                if report.is_empty() {
                    return Err(ReportError::NoInfoProvided);
                }
                Ok(report)
            }
        }
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

        assert!(house.get_rooms().unwrap().contains_key("room"));
        assert!(house.get_rooms().is_some());
    }

    #[test]
    fn cannot_add_room_with_same_name() {
        let mut house = SmartHouse::new("smart house".into());
        let room1 = Room::new("room".into(), HashSet::new());
        let room2 = Room::new("room".into(), HashSet::new());

        house.add_room(room1);
        house.add_room(room2);

        assert_eq!(house.get_rooms().unwrap().len(), 1);
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
            Ok("Room: room, Device SmartSocket: socket, state is On\n".to_string())
        );
        assert_eq!(report2, Ok("Room: room2, Device SmartSocket: socket2, state is Off\nRoom: room2, Device Thermometer: thermo, temperature is \"25\"\n".to_string()));
    }

    #[test]
    fn create_report_return_rooms_not_found_error() {
        let house = SmartHouse::new("Smart House".into());
        let socket = SmartSocket::new("socket".into(), State::On);
        let info_provider = OwningDeviceInfoProvider { socket };

        let report1 = house.create_report(&info_provider);

        assert_eq!(report1, Err(ReportError::RoomsNotFound));
    }

    #[test]
    fn create_report_return_no_info_provided_error() {
        let mut house = SmartHouse::new("Smart House".into());
        let room1 = Room::new("room".into(), HashSet::new());
        let room2 = Room::new("room2".into(), HashSet::new());
        house.add_room(room1);
        house.add_room(room2);
        let socket = SmartSocket::new("socket".into(), State::On);
        let info_provider = OwningDeviceInfoProvider { socket };

        let report1 = house.create_report(&info_provider);

        assert_eq!(report1, Err(ReportError::NoInfoProvided))
    }
}
