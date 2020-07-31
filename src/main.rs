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
}

fn main() {
    println!("Hello, world!");
}
