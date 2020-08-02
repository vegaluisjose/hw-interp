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

fn is_ready(expr: &Expr, state: &State) -> bool {
    match expr {
        Expr::Lit(_) => true,
        Expr::Ref(var) => state.contains_value(var),
        Expr::Add(lhs, rhs) => is_ready(lhs, state) && is_ready(rhs, state),
        Expr::Reg(var) => is_ready(var, state),
    }
}

pub fn eval_prog(prog: &Prog, state: &State) -> State {
    let mut state_in = state.clone();
    let mut state_out = State::default();
    let mut unresolved: Vec<Stmt> = Vec::new();
    // initial eval, push unresolved instr for later
    for stmt in prog.body.iter() {
        if is_ready(&stmt.expr, &state_in) {
            let val = eval_expr(&stmt.expr, &state_in);
            if state_in.is_reg(&stmt.id) {
                state_out.add_reg(&stmt.id, val);
            } else {
                state_in.add_temp(&stmt.id, val);
                state_out.add_temp(&stmt.id, val);
            }
        } else {
            unresolved.push(stmt.clone());
        }
    }
    // eval unresolved instr, it should succeed
    for stmt in unresolved.iter() {
        if is_ready(&stmt.expr, &state_in) {
            let val = eval_expr(&stmt.expr, &state_in);
            if state_in.is_reg(&stmt.id) {
                state_out.add_reg(&stmt.id, val);
            } else {
                state_in.add_temp(&stmt.id, val);
                state_out.add_temp(&stmt.id, val);
            }
        } else {
            panic!("Error: program contains cycles, malformed");
        }
    }
    state_out
}
