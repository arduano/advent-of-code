mod coord2;
mod grid2;
mod igrid2;
mod iter_helpers;
mod nums;
mod parse;
mod string_helpers;

pub use coord2::*;
pub use grid2::*;
pub use igrid2::*;
pub use iter_helpers::*;
pub use nums::*;
pub use parse::*;
pub use string_helpers::*;

#[macro_export]
macro_rules! day_input {
    () => {
        include_str!(concat!(env!("CARGO_MANIFEST_DIR"), "/input.txt"))
    };
}
