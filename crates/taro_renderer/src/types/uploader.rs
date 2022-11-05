use crate::TaroRenderer;

pub trait TaroUploader {
    type UploadedData;

    fn upload(&mut self, renderer: &TaroRenderer);
    fn get_data(&self) -> &Option<Self::UploadedData>;

    fn get_uploaded(&mut self, renderer: &TaroRenderer) -> &Self::UploadedData {
        if self.get_data().is_some() {
            return self.get_data().as_ref().unwrap();
        }

        self.upload(renderer);
        self.get_data().as_ref().unwrap()
    }
}
