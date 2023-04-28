use std::collections::HashSet;

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

    // pub fn create_report(
    //     &self,
    //     /* todo: принять обобщённый тип предоставляющий информацию об устройствах */
    // ) -> String {
    //     todo!("перебор комнат и устройств в них для составления отчёта")
    // }
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
