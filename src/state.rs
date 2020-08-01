use std::collections::HashMap;

type Inputs = HashMap<String, i32>;
type Outputs = HashMap<String, i32>;
type Regs = HashMap<String, i32>;
type Temps = HashMap<String, i32>;

#[derive(Clone, Debug)]
pub struct State {
    inputs: Inputs,
    outputs: Outputs,
    regs: Regs,
    temps: Temps,
}

impl Default for State {
    fn default() -> State {
        State {
            inputs: Inputs::new(),
            outputs: Outputs::new(),
            regs: Regs::new(),
            temps: Temps::new(),
        }
    }
}

impl State {
    pub fn add_input(&mut self, id: &str, value: i32) {
        self.inputs.insert(id.to_string(), value);
    }

    pub fn add_output(&mut self, id: &str, value: i32) {
        self.outputs.insert(id.to_string(), value);
    }

    pub fn add_reg(&mut self, id: &str, value: i32) {
        self.regs.insert(id.to_string(), value);
    }

    pub fn add_temp(&mut self, id: &str, value: i32) {
        self.temps.insert(id.to_string(), value);
    }

    pub fn is_output(&self, id: &str) -> bool {
        self.outputs.contains_key(id)
    }

    pub fn is_reg(&self, id: &str) -> bool {
        self.regs.contains_key(id)
    }

    pub fn inputs(&self) -> &Inputs {
        &self.inputs
    }

    pub fn outputs(&self) -> &Outputs {
        &self.outputs
    }

    pub fn regs(&self) -> &Regs {
        &self.regs
    }

    pub fn set_inputs(&mut self, inputs: &Inputs) {
        self.inputs = inputs.clone();
    }

    pub fn set_outputs(&mut self, outputs: &Outputs) {
        self.outputs = outputs.clone();
    }

    pub fn set_regs(&mut self, regs: &Regs) {
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
}

pub trait ToString {
    fn to_string(&self) -> String;
}

impl ToString for HashMap<String, i32> {
    fn to_string(&self) -> String {
        let mut fmt = String::new();
        for (id, val) in self.iter() {
            fmt.push_str(&format!(" {}:{}", id, val));
        }
        fmt
    }
}
