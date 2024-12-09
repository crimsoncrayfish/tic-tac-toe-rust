#[macro_export]
macro_rules! assert_r {
    ($cond: expr, $err: expr) => {
        if !$cond {
            return Err($err);
        }
    };
}
