use std::slice::IterMut;
pub trait U64arraySerialize {
    fn to_u64_array(&self) -> Vec<u64>;
    fn from_u64_array(data: &mut IterMut<u64>) -> Self;
}
