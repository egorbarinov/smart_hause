use crate::device::{Device, SmartSocket, SmartThermometer};

pub trait DeviceInfoProvider {
    fn contains(&self, device_name: &str) -> bool;
    fn get_devices(&self) -> Vec<&dyn Device>;
    fn get_info(&self, room: &str, device: &str) -> String;
}

pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket,
    pub thermo: &'b SmartThermometer,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn contains(&self, device_name: &str) -> bool {
        self.get_devices()
            .iter()
            .any(|device| device.get_name() == device_name)
    }

    fn get_devices(&self) -> Vec<&dyn Device> {
        vec![&self.socket as &dyn Device]
    }

    fn get_info(&self, room: &str, device: &str) -> String {
        if self.socket.get_name() == device {
            format!("Room: {}, Device {}", room, self.socket.create_report())
        } else {
            "".to_string()
        }
    }
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn contains(&self, device_name: &str) -> bool {
        if self.socket.name == *device_name {
            true
        } else {
            self.thermo.name == *device_name
        }
    }

    fn get_devices(&self) -> Vec<&dyn Device> {
        vec![self.socket, self.thermo]
    }

    fn get_info(&self, room: &str, device: &str) -> String {
        if self.socket.get_name() == device {
            format!("Room: {}, Device {}", room, self.socket.create_report())
        } else {
            format!("Room: {}, Device {}", room, self.thermo.create_report())
        }
    }
}
