
pub trait Planar<T> {
    fn get(&self, index: usize) -> T;
    fn is_empty(&self) -> bool;
    fn len(&self) -> usize;
    fn set(&mut self, index: usize, value: T);
    fn to_interleaved(&self) -> Vec<T>;

    fn from_interleaved(packed: Vec<T>) -> Self where Self: Sized;
}
