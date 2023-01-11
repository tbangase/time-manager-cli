pub use stdext::function_name;

#[macro_export]
macro_rules! error_trace{
    () =>{
        format!("{}:{} {}", file!(), line!(), $crate::function_name!())
    };
    ($($x:expr), *) => {
        format!("{}:{} {}: \n {}", file!(), line!(), $crate::function_name!(), format!($($x),*))
    };
}
