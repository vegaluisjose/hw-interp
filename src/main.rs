use std::rc::Rc;
use std::collections::HashMap;

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

fn eval_expr(expr: &Expr, state: &State) -> i32 {
    match expr {
        Expr::Lit(num) => *num,
        Expr::Ref(var) => state.get_value(var),
        Expr::Add(lhs, rhs) => eval_expr(lhs, state) + eval_expr(rhs, state),
    }
}

fn eval_prog(prog: &Prog, state: &State) -> State {
    let mut state_in = state.clone();
    let mut state_out = State::default();
    for stmt in prog.body.iter() {
        let val = eval_expr(&stmt.expr, &state_in);
        if state_in.is_output(&stmt.id) {
            state_out.add_output(&stmt.id, val);
        } else if state_in.is_reg(&stmt.id) {
            state_out.add_reg(&stmt.id, val);
        } else {
            state_in.add_temp(&stmt.id, val);
        }
    }
    state_out
}

fn add_two_inputs() -> Prog {
    let a = Expr::Ref("a".to_string());
    let b = Expr::Ref("b".to_string());
    let add = Expr::Add(Rc::new(a), Rc::new(b));
    let stmt = Stmt { id: "y".to_string(), expr: add };
    Prog { body: vec![stmt] }
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

fn interp(prog: &Prog, cycles: u32) {
    let mut state = State::default();
    for i in 0..cycles {
        state.add_input("a", 3);
        state.add_input("b", 4);
        state.add_output("y", 0);
        let next = eval_prog(&prog, &state);
        state.set_outputs(next.outputs());
        state.set_regs(next.regs());
        println!("cycle:{} i:{}", i, state.inputs().to_string());
        println!("cycle:{} o:{}", i, state.outputs().to_string());
    }
}

fn main() {
    let prog = add_two_inputs();
    interp(&prog, 10);
}
