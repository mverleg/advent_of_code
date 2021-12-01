


macro_rules! regex2struct {
    ($ResType: ident, $regex: expr) => {
        #[derive(Debug)]
        pub struct $ResType {
        }
    }
}


regex2struct!(Res, r"^[0-9]$");

#[test]
fn test() {
    unimplemented!()
}

