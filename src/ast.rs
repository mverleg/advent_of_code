
#[derive(Debug)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Expr<T> {
    Bin {
        op: Op,
        left: Box<Expr<T>>,
        right: Box<Expr<T>>,
    },
    Uni {
        op: Op,
        target: Box<Expr<T>>,
    },
    Val(T)
}

