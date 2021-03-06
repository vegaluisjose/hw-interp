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
        // add inputs for this cycle
        for id in prog.inputs.iter() {
            state.add_input(&id, trace.deq(&id));
        }
        // print inputs
        println!("[in] cycle:{} {:?}", i, state.inputs());
        // eval
        let next = eval_prog(&prog, &state);
        // print outputs
        for id in prog.outputs.iter() {
            if state.is_reg(id) {
                assert!(trace.deq(&id) == state.get_value(&id));
                println!("[out] cycle:{} {}:{}", i, &id, state.get_value(&id))
            } else {
                assert!(trace.deq(&id) == next.get_value(&id));
                println!("[out] cycle:{} {}:{}", i, &id, next.get_value(&id))
            }
        }
        // commit new register values
        state.set_regs(next.regs());
    }
}
