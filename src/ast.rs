
#[derive(Debug, PartialEq, PartialOrd)]
pub enum Op {
    Add,
    Sub,
    Mul,
    Div,
}

#[derive(Debug, PartialEq, PartialOrd)]
pub enum Expr<T: PartialEq + PartialOrd> {
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

