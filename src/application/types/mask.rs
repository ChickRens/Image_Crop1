use ndarray::Array2;
use ndarray::OwnedRepr;
use ndarray::prelude::ArrayBase;
use ndarray::prelude::Dim;

pub struct Mask {
    value: Array2<f32>
}

impl Mask {
    pub fn new(value:ArrayBase<OwnedRepr<f32>, Dim<[usize; 2]>, f32> ) -> Self {
        Self { value }
    }
}