use std::collections::HashMap;

type StateMap = HashMap<String, i32>;

#[derive(Clone, Debug)]
pub struct State {
    inputs: StateMap,
    regs: StateMap,
    temps: StateMap,
}

impl Default for State {
    fn default() -> State {
        State {
            inputs: StateMap::new(),
            regs: StateMap::new(),
            temps: StateMap::new(),
        }
    }
}

impl State {
    pub fn add_input(&mut self, id: &str, value: i32) {
        self.inputs.insert(id.to_string(), value);
    }

    pub fn add_reg(&mut self, id: &str, value: i32) {
        self.regs.insert(id.to_string(), value);
    }

    pub fn add_temp(&mut self, id: &str, value: i32) {
        self.temps.insert(id.to_string(), value);
    }

    pub fn is_reg(&self, id: &str) -> bool {
        self.regs.contains_key(id)
    }

    pub fn inputs(&self) -> &StateMap {
        &self.inputs
    }

    pub fn regs(&self) -> &StateMap {
        &self.regs
    }

    pub fn set_inputs(&mut self, inputs: &StateMap) {
        self.inputs = inputs.clone();
    }

    pub fn set_regs(&mut self, regs: &StateMap) {
        self.regs = regs.clone();
    }

    pub fn get_value(&self, id: &str) -> i32 {
        if let Some(input) = self.inputs.get(id) {
            *input
        } else if let Some(reg) = self.regs.get(id) {
            *reg
        } else if let Some(temp) = self.temps.get(id) {
            *temp
        } else {
            panic!("Error: {} id not found", id)
        }
    }

    pub fn contains_id(&self, id: &str) -> bool {
        self.inputs.get(id).is_some() || self.regs.get(id).is_some() || self.temps.get(id).is_some()
    }
}
