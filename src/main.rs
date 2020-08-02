use hw_interp::ast::*;
use hw_interp::interp;
use hw_interp::trace::Trace;
use std::rc::Rc;

fn prog_add() -> Prog {
    let a = Expr::Ref("a".to_string());
    let b = Expr::Ref("b".to_string());
    let add = Expr::Add(Rc::new(a), Rc::new(b));
    let stmt = Stmt {
        id: "y".to_string(),
        expr: add,
    };
    Prog {
        inputs: vec!["a".to_string(), "b".to_string()],
        outputs: vec!["y".to_string()],
        body: vec![stmt],
    }
}

fn build_trace() -> Trace {
    let mut trace = Trace::default();
    trace.push_value("a", 3);
    trace.push_value("b", 4);
    trace.push_value("y", 7);
    trace.push_value("a", 1);
    trace.push_value("b", 3);
    trace.push_value("y", 4);
    trace
}

fn main() {
    let prog = prog_add();
    let trace = build_trace();
    interp(&prog, &trace);
}
