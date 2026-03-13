use std::fmt::Debug;

pub fn debug_render<T>(value: &T) -> String
where
    T: Debug,
{
    format!("{value:#?}")
}
