mod read;
pub mod aysnc_read;
pub mod frame;
mod error;
pub mod metadata;
pub type Result<T> = std::result::Result<T, error::Error>;
pub use read::*;




#[macro_export]
macro_rules! const_array {

    (@start($($start:expr), *), @end($($end:expr), *), $arr:expr => $($i:expr), +) => {
        [$($start)*, $($arr[$i],)+ $($end),*]
    };
    (@start($($start:expr), +), $arr:expr => $($i:expr), +) => {
        [$($start)+, $($arr[$i]), +]
    };
    (@end($($end:expr), +), $arr:expr => $($i:expr), +) => {
        [$($arr[$i],)+ $($end),+ ]
    };
    ($arr:expr => $($i:expr), +) => {
        [$($arr[$i]), +]
    };
    ($arr:expr, $start:expr, $const_count:expr) => {
        {
            let mut __buf__ = [0; $const_count];
            __buf__.copy_from_slice(&$arr[$start..$start + $const_count]);
            __buf__
        }
    }
}

