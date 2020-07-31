use std::rc::Rc;
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

    pub fn get(&self, id: &str) -> i32 {
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

type Id = String;

#[derive(Clone, Debug)]
pub enum Expr {
    Lit(i32),
    Ref(Id),
    Add(Rc<Expr>, Rc<Expr>),
}

#[derive(Clone, Debug)]
pub struct Stmt {
    pub id: Id,
    pub expr: Expr,
}

#[derive(Clone, Debug)]
pub struct Prog {
    pub body: Vec<Stmt>,
}

fn add_two_inputs() -> Prog {
    let a = Expr::Ref("a".to_string());
    let b = Expr::Ref("b".to_string());
    let add = Expr::Add(Rc::new(a), Rc::new(b));
    let stmt = Stmt { id: "y".to_string(), expr: add };
    Prog { body: vec![stmt] }
}

fn main() {
    let prog = add_two_inputs();
    for stmt in prog.body.iter() {
        println!("{:?}", stmt);
    }
}
