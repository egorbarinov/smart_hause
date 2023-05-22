use std::collections::HashSet;

#[derive(Clone, PartialEq, Debug)]
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

    #[allow(dead_code)]
    pub fn delete_device(&mut self, device: &str) -> Option<bool> {
        Some(self.devices.remove(device))
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
    fn can_add_device() {
        let mut room = Room::new("room".into(), HashSet::new());
        let device = "socket".into();
        let device2 = "thermo".into();

        let result = room.add_device(device);
        let result2 = room.add_device(device2);
        assert_eq!(result, Some(true));
        assert_eq!(result2, Some(true));
    }

    #[test]
    fn can_delete_device() {
        let mut room = Room::new("room".into(), HashSet::new());
        let device = "socket".to_string();
        let device2 = "thermo".to_string();

        room.add_device(device.clone());
        room.add_device(device2.clone());

        assert_eq!(room.delete_device(&device), Some(true));
        assert_eq!(room.delete_device(&device), Some(false));
        assert_eq!(room.delete_device(&device2), Some(true));
        assert_eq!(room.delete_device(&device2), Some(false));
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
