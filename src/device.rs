pub trait Device {
    fn get_name(&self) -> &str;
    fn create_report(&self) -> String;
}

#[derive(Debug)]
pub enum State {
    Off,
    On,
}

pub struct SmartSocket {
    pub name: String,
    state: State,
}

impl SmartSocket {
    pub fn new(name: String, state: State) -> Self {
        SmartSocket { name, state }
    }
}

pub struct SmartThermometer {
    pub name: String,
    temperature: String,
}

impl SmartThermometer {
    pub fn new(name: String, temperature: String) -> Self {
        SmartThermometer { name, temperature }
    }
}

impl Device for SmartSocket {
    fn get_name(&self) -> &str {
        self.name.as_ref()
    }

    fn create_report(&self) -> String {
        format!("SmartSocket: {}, state is {:?}", &self.name, &self.state)
    }
}

impl Device for SmartThermometer {
    fn get_name(&self) -> &str {
        self.name.as_ref()
    }

    fn create_report(&self) -> String {
        format!(
            "Thermometer: {}, temperature is {:?}",
            self.name, self.temperature
        )
    }
}
