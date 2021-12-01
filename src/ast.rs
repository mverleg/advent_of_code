
#[derive(Debug)]
pub enum Ops {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug)]
pub enum Expr<T> {
    Bin {
        op: String,
        left: Expr<T>,
        right: Expr<T>,
    },
    Uni {
        op: String,
        target: Expr<T>,
    },
    Val(T)
}

