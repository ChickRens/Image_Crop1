use ndarray::Array4;
use ndarray::ArrayView4;
use ndarray::OwnedRepr;
use ndarray::prelude::ArrayBase;
use ndarray::prelude::Dim;

pub struct Mask {
    value: Array4<f32>,
}

impl Mask {
    pub fn new(value: ArrayBase<OwnedRepr<f32>, Dim<[usize; 4]>, f32>) -> Self {
        Self { value }
    }

    pub fn view(&self) -> ArrayView4<f32> {
        self.value.view()
    }

    pub fn into_mask(self) -> ArrayBase<OwnedRepr<f32>, Dim<[usize; 4]>, f32> {
        self.value
    }
    // for Zero-Copy
}
