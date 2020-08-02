use crate::ast::{Prog, Expr};
use crate::eval::eval_prog;
use crate::state::{State, ToString};
use crate::trace::Trace;

fn get_reg_ids(prog: &Prog) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    for stmt in prog.body.iter() {
        match stmt.expr {
            Expr::Reg(_) => ids.push(stmt.id.to_string()),
            _ => (),
        }
    }
    ids
}

pub fn interp(prog: &Prog, trace: &Trace) {
    let mut trace = trace.clone();
    assert!(trace.is_valid(), "Error: invalid trace");
    let cycles = trace.cycles();
    let mut state = State::default();
    for id in get_reg_ids(prog).iter() {
        state.add_reg(id, 0);
    }
    for id in prog.outputs.iter() {
        state.add_output(id, 0);
    }
    for i in 0..cycles {
        for id in prog.inputs.iter() {
            state.add_input(&id, trace.deq(&id));
        }
        let next = eval_prog(&prog, &state);
        state.set_outputs(next.outputs());
        state.set_regs(next.regs());
        println!("cycle:{} i:{}", i, state.inputs().to_string());
        println!("cycle:{} o:{}", i, state.outputs().to_string());
    }
}
