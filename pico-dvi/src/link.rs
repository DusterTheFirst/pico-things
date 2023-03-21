#[macro_export]
macro_rules! scratch {
    (x, $fn_name:ident) => {
        concat!(".scratch_x.", file!(), ".", line!(), ".", stringify!($fn_name))
    };
    (y, $fn_name:ident) => {
        concat!(".scratch_y.", file!(), ".", line!(), ".", stringify!($fn_name))
    };
}