pub struct Room {
    pub(crate) name: String,
    pub(crate) devices: Vec<String>,
}

impl Room {
    pub fn new(name: String, devices: Vec<String>) -> Self {
        Room { devices, name }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_ref()
    }

    #[allow(dead_code)]
    pub fn devices(&self) -> &Vec<String> {
        &self.devices
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_return_room_name() {
        let room = Room::new(String::from("room"), Vec::new());

        assert_eq!(room.get_name(), "room");
    }

    #[test]
    fn can_return_devices() {
        let mut room = Room::new(String::from("room"), Vec::new());
        let device = String::from("socket");
        let device2 = String::from("thermo");
        room.devices.push(device);
        room.devices.push(device2);

        assert!(room.devices().contains(&String::from("socket")));
        assert!(room.devices().contains(&String::from("thermo")));
        assert_eq!(room.devices().len(), 2);
    }
}
