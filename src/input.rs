use device_query::{DeviceQuery, DeviceState, Keycode};

pub struct Keyboard {
    device_state: DeviceState,
    prev_keys: Vec<Keycode>,
}

impl Keyboard {
    pub fn new() -> Keyboard {
        Keyboard {
            device_state: DeviceState::new(),
            prev_keys: vec![],
        }
    }

    pub fn get_active_keys(&mut self) -> &Vec<Keycode> {
        let keys = self.device_state.get_keys();
        if keys != self.prev_keys {
            println!("{:?}", keys);
        }
        self.prev_keys = keys;
        &self.prev_keys
    }
}
