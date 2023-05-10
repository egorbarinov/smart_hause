use crate::device::{Device, SmartSocket, SmartThermometer};

pub trait DeviceInfoProvider {
    fn get_devices(&self) -> Vec<&dyn Device>;
    fn get_info(&self, room: &str, device: &str) -> Option<String>;
}

pub struct OwningDeviceInfoProvider {
    pub socket: SmartSocket,
}

pub struct BorrowingDeviceInfoProvider<'a, 'b> {
    pub socket: &'a SmartSocket,
    pub thermo: &'b SmartThermometer,
}

impl DeviceInfoProvider for OwningDeviceInfoProvider {
    fn get_devices(&self) -> Vec<&dyn Device> {
        vec![&self.socket as &dyn Device]
    }

    fn get_info(&self, room: &str, device: &str) -> Option<String> {
        if self.socket.get_name() == device {
            Some(format!(
                "Room: {}, Device {}",
                room,
                self.socket.create_report()
            ))
        } else {
            None
        }
    }
}

impl<'a, 'b> DeviceInfoProvider for BorrowingDeviceInfoProvider<'a, 'b> {
    fn get_devices(&self) -> Vec<&dyn Device> {
        vec![self.socket, self.thermo]
    }

    fn get_info(&self, room: &str, device: &str) -> Option<String> {
        if self.socket.get_name() == device {
            Some(format!(
                "Room: {}, Device {}",
                room,
                self.socket.create_report()
            ))
        } else if self.thermo.get_name() == device {
            Some(format!(
                "Room: {}, Device {}",
                room,
                self.thermo.create_report()
            ))
        } else {
            None
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::device::State;

    #[test]
    fn create_report_for_owning_device() {
        let socket = SmartSocket::new("socket".into(), State::On);
        let info_provider = OwningDeviceInfoProvider { socket };

        let report = info_provider.get_info("room", "socket");
        assert_eq!(
            report,
            Some("Room: room, Device SmartSocket: socket, state is On".to_string())
        );
    }

    #[test]
    fn create_report_for_borrowing_device() {
        let socket = SmartSocket::new("socket".into(), State::On);
        let thermo = SmartThermometer::new("thermo".into(), "25".into());
        let info_provider = BorrowingDeviceInfoProvider {
            socket: &socket,
            thermo: &thermo,
        };

        let socket_info = info_provider.get_info("room", "socket");
        let thermo_info = info_provider.get_info("room", "thermo");

        assert_eq!(
            socket_info,
            Some("Room: room, Device SmartSocket: socket, state is On".to_string())
        );
        assert_eq!(
            thermo_info,
            Some("Room: room, Device Thermometer: thermo, temperature is \"25\"".to_string())
        );
    }
}
