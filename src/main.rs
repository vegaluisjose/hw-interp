use hw_interp::ast::*;
use hw_interp::interp;
use std::rc::Rc;

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

fn main() {
    let prog = prog_add();
    interp(&prog, 10);
}
