use crate::ast::{Expr, Prog};
use crate::eval::eval_prog;
use crate::state::State;
use crate::trace::Trace;

fn get_reg_ids(prog: &Prog) -> Vec<String> {
    let mut ids: Vec<String> = Vec::new();
    for stmt in prog.body.iter() {
        if let Expr::Reg(_) = stmt.expr {
            ids.push(stmt.id.to_string())
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
    for i in 0..cycles {
        for id in prog.inputs.iter() {
            state.add_input(&id, trace.deq(&id));
        }
        println!("[in] cycle:{} {:?}", i, state.inputs());
        let next = eval_prog(&prog, &state);
        for id in prog.outputs.iter() {
            if state.is_reg(id) {
                println!("[out] cycle:{} {}:{}", i, &id, state.get_value(&id))
            } else {
                println!("[out] cycle:{} {}:{}", i, &id, next.get_value(&id))
            }
        }
        state.set_regs(next.regs());
    }
}
