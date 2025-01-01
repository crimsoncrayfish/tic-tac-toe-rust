#[macro_export]
macro_rules! assert_r {
    ($cond: expr, $err: expr) => {
        if !$cond {
            return Err($err);
        }
    };
}
#[macro_export]
macro_rules! vec_vec_u8_to_string {
    ($vec:expr) => {
        $vec.iter()
            .map(|inner| inner.iter().map(|&byte| byte as char).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    };
}
#[macro_export]
macro_rules! vec_vec_enum_to_string {
    ($vec:expr) => {
        $vec.iter()
            .map(|inner| inner.iter().map(|v| v.to_string()).collect::<String>())
            .collect::<Vec<String>>()
            .join("\n")
    };
}
#[macro_export]
macro_rules! enum_to_string {
    ($enum_name:ident, { $($variant:ident),* }) => {
        impl ToString for $enum_name {
            fn to_string(&self) -> String {
                match self {
                    $(
                        $enum_name::$variant => stringify!($variant).to_string(),
                    )*
                }
            }
        }
    };
}
