pub struct GetImageOutput{
    data: Vec<u8>
}

impl GetImageOutput{
    pub fn new(data: Vec<u8>) -> Self {
        Self { data }
    }

    pub fn into_image_data(self) -> Vec<u8> {
        self.data
    }
}