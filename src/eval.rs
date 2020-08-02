use crate::ast::*;
use crate::state::State;

fn eval_expr(expr: &Expr, state: &State) -> i32 {
    match expr {
        Expr::Lit(num) => *num,
        Expr::Ref(var) => state.get_value(var),
        Expr::Add(lhs, rhs) => eval_expr(lhs, state) + eval_expr(rhs, state),
        Expr::Reg(var) => eval_expr(var, state),
    }
}

pub fn eval_prog(prog: &Prog, state: &State) -> State {
    let mut state_in = state.clone();
    let mut state_out = State::default();
    for stmt in prog.body.iter() {
        let val = eval_expr(&stmt.expr, &state_in);
        if state_in.is_output(&stmt.id) && state_in.is_reg(&stmt.id) {
            state_out.add_output(&stmt.id, state_in.get_value(&stmt.id));
            state_out.add_reg(&stmt.id, val);
        } else if state_in.is_output(&stmt.id) {
            state_out.add_output(&stmt.id, val);
        } else if state_in.is_reg(&stmt.id) {
            state_out.add_reg(&stmt.id, val);
        } else {
            state_in.add_temp(&stmt.id, val);
        }
    }
    state_out
}
