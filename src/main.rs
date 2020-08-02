use hw_interp::ast::*;
use hw_interp::interp;
use hw_interp::trace::Trace;
use std::rc::Rc;

fn prog_add() -> Prog {
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

fn prog_counter() -> Prog {
    let t0 = Expr::Ref("t0".to_string());
    let t1 = Expr::Ref("t1".to_string());
    let t2 = Expr::Ref("t2".to_string());
    let s0 = Stmt {
        id: "t0".to_string(),
        expr: Expr::Lit(1),
    };
    let s1 = Stmt {
        id: "t1".to_string(),
        expr: Expr::Add(Rc::new(t2.clone()), Rc::new(t0)),
    };
    let s2 = Stmt {
        id: "t2".to_string(),
        expr: Expr::Reg(Rc::new(t1)),
    };
    let s3 = Stmt {
        id: "y".to_string(),
        expr: t2,
    };
    Prog {
        inputs: vec![],
        outputs: vec!["y".to_string()],
        body: vec![s0, s1, s2, s3],
    }
}

fn trace_add() -> Trace {
    let mut trace = Trace::default();
    trace.enq("a", 3);
    trace.enq("a", 1);
    trace.enq("a", 0);
    trace.enq("b", 4);
    trace.enq("b", 3);
    trace.enq("b", 0);
    trace.enq("y", 0);
    trace.enq("y", 7);
    trace.enq("y", 4);
    trace
}

fn trace_counter() -> Trace {
    let mut trace = Trace::default();
    trace.enq("y", 0);
    trace.enq("y", 1);
    trace.enq("y", 2);
    trace
}

fn test_add() {
    let prog = prog_add();
    let trace = trace_add();
    println!("\nTesting add\n");
    interp(&prog, &trace);
}

fn test_counter() {
    let prog = prog_counter();
    let trace = trace_counter();
    println!("\nTesting counter\n");
    interp(&prog, &trace);
}

fn main() {
    test_add();
    test_counter();
}
