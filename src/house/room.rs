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

    pub fn devices(&self) -> Option<&HashSet<String>> {
        if self.devices.is_empty() {
            None
        } else {
            Some(&self.devices)
        }
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
        let room = Room::new("room".into(), HashSet::new());

        assert_eq!(room.get_name(), "room");
    }

    #[test]
    fn can_return_devices() {
        let mut room = Room::new("room".into(), HashSet::new());
        let device = "socket".into();
        let device2 = "thermo".into();
        room.devices.insert(device);
        room.devices.insert(device2);

        assert_eq!(room.devices().is_some(), true);
        assert!(room.devices().unwrap().contains(&"socket".to_string()));
        assert!(room.devices().unwrap().contains(&"thermo".to_string()));
        assert_eq!(room.devices().unwrap().len(), 2);
    }
}
