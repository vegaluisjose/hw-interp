use hw_interp::ast::*;
use hw_interp::interp;
use hw_interp::trace::Trace;
use std::rc::Rc;

fn prog() -> Prog {
    let a = Expr::Ref("a".to_string());
    let b = Expr::Ref("b".to_string());
    let x = Expr::Ref("x".to_string());
    let s0 = Stmt {
        id: "x".to_string(),
        expr: Expr::Add(Rc::new(a), Rc::new(b)),
    };
    let s1 = Stmt {
        id: "y".to_string(),
        expr: Expr::Reg(Rc::new(x)),
    };
    Prog {
        inputs: vec!["a".to_string(), "b".to_string()],
        outputs: vec!["y".to_string()],
        body: vec![s0, s1],
    }
}

fn build_trace() -> Trace {
    let mut trace = Trace::default();
    trace.enq("a", 3);
    trace.enq("b", 4);
    trace.enq("y", 0);
    trace.enq("a", 1);
    trace.enq("b", 3);
    trace.enq("y", 7);
    trace.enq("a", 0);
    trace.enq("b", 0);
    trace.enq("y", 4);
    trace
}

fn main() {
    let prog = prog();
    let trace = build_trace();
    interp(&prog, &trace);
}
