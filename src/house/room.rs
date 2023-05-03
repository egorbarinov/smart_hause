pub struct Room {
    pub(crate) name: String,
    pub(crate) devices: Vec<String>,
}

impl Room {
    pub fn new(name: String, devices: Vec<String>) -> Self {
        Room { devices, name }
    }
}
