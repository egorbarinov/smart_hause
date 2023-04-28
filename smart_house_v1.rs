use crate::Switch::{Off, On};

struct SmartSocket {
    description: String,
    switched: Switch,
    watt: i32,
}

enum Switch {
    Off,
    On,
}

struct Thermometer {
    temperature: i32,
}

impl SmartSocket {
    pub fn get_description(&self) -> String {
        let description = &self.description;
        println!("Description: {}", description);
        description.to_string()
    }

    pub fn switch(&mut self) {
        match &self.switched {
            Off => self.switched = On,
            On => self.switched = Off
        }
    }

    pub fn is_switched(&self) -> bool {
        match &self.switched {
            Off => false,
            On => true
        }
    }

    pub fn get_watt(&self) -> i32 {
        let watt = self.watt;
        println!("Watt: {}", watt);
        watt
    }
}

impl Thermometer {
    pub fn get_temperature(&self) -> i32 {
        let temperature = self.temperature;
        println!("Temperature: {}", temperature);
        temperature
    }
}

fn main() {
    let mut smart_socket = SmartSocket {
        description: "I am a smart plug".to_string(),
        switched: Off,
        watt: 220,
    };
    smart_socket.switch();
    println!("Switch: {}", smart_socket.is_switched()); //true
    smart_socket.switch();
    println!("Switch: {}", smart_socket.is_switched()); //false
    smart_socket.switch();
    println!("Switch: {}", smart_socket.is_switched()); //true
    let _watt = smart_socket.get_watt(); //220
    let _description = smart_socket.get_description(); // "I am a smart plug"

    let thermometer = Thermometer {
        temperature: 25
    };
    let _temperature = thermometer.get_temperature(); // 25
}
