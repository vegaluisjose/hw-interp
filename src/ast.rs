use std::rc::Rc;

type Id = String;

#[derive(Clone, Debug)]
pub enum Expr {
    Lit(i32),
    Ref(Id),
    Add(Rc<Expr>, Rc<Expr>),
}

#[derive(Clone, Debug)]
pub struct Stmt {
    pub id: Id,
    pub expr: Expr,
}

#[derive(Clone, Debug)]
pub struct Prog {
    pub inputs: Vec<Id>,
    pub outputs: Vec<Id>,
    pub body: Vec<Stmt>,
}
