use crate::ast::Prog;
use crate::eval::eval_prog;
use crate::state::{State, ToString};

pub fn interp(prog: &Prog, cycles: u32) {
    let mut state = State::default();
    // init regs and outputs with zero
    state.add_output("y", 0);
    for i in 0..cycles {
        state.add_input("a", 3);
        state.add_input("b", 4);
        let next = eval_prog(&prog, &state);
        state.set_outputs(next.outputs());
        state.set_regs(next.regs());
        println!("cycle:{} i:{}", i, state.inputs().to_string());
        println!("cycle:{} o:{}", i, state.outputs().to_string());
    }
}
