use hw_interp::ast::*;
use hw_interp::state::{State, ToString};
use std::rc::Rc;

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

fn prog_add() -> Prog {
    let a = Expr::Ref("a".to_string());
    let b = Expr::Ref("b".to_string());
    let add = Expr::Add(Rc::new(a), Rc::new(b));
    let stmt = Stmt {
        id: "y".to_string(),
        expr: add,
    };
    Prog { body: vec![stmt] }
}

fn interp(prog: &Prog, cycles: u32) {
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

fn main() {
    let prog = prog_add();
    interp(&prog, 10);
}
