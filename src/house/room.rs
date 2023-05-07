use std::collections::HashSet;

pub struct Room {
    pub(crate) name: String,
    pub(crate) devices: HashSet<String>,
}

impl Room {
    pub fn new(name: String, devices: HashSet<String>) -> Self {
        Room { name, devices }
    }

    pub fn get_name(&self) -> &str {
        self.name.as_ref()
    }

    #[allow(dead_code)]
    pub fn devices(&self) -> &HashSet<String> {
        &self.devices
    }

    #[allow(dead_code)]
    pub fn add_device(&mut self, device: String) -> Option<bool> {
        if !self.devices.contains(&device) {
            self.devices.insert(device);

            return Some(true);
        }

        None
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn can_return_room_name() {
        let room = Room::new(String::from("room"), HashSet::new());

        assert_eq!(room.get_name(), "room");
    }

    #[test]
    fn can_return_devices() {
        let mut room = Room::new(String::from("room"), HashSet::new());
        let device = String::from("socket");
        let device2 = String::from("thermo");
        room.devices.insert(device);
        room.devices.insert(device2);

        assert!(room.devices().contains(&String::from("socket")));
        assert!(room.devices().contains(&String::from("thermo")));
        assert_eq!(room.devices().len(), 2);
    }
}
