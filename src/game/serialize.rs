use std::slice::IterMut;
pub trait U64arraySerialize {
    fn to_u64_array(&self, data: &mut Vec<u64>);
    fn from_u64_array(data: &mut IterMut<u64>) -> Self;
    fn modify_from_u64_array(&mut self, data: &mut IterMut<u64>); // better trace if called from outside
}
