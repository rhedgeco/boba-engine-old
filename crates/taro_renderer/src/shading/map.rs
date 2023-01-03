use once_map::{GetOrInitData, OnceMap};

use crate::{HardwareId, TaroHardware};

pub trait TaroData<T> {
    fn write_new(&self, new_data: &T, hardware: &TaroHardware);
}

pub trait TaroDataUploader
where
    Self: Sized,
{
    type UploadData: TaroData<Self>;
    fn new_upload(&self, hardware: &TaroHardware) -> Self::UploadData;
}

pub struct TaroMap<T>
where
    T: TaroDataUploader,
{
    cache: OnceMap<HardwareId, T::UploadData>,
}

impl<T> Default for TaroMap<T>
where
    T: TaroDataUploader,
{
    fn default() -> Self {
        Self {
            cache: Default::default(),
        }
    }
}

impl<T> TaroMap<T>
where
    T: TaroDataUploader,
{
    pub fn new() -> Self {
        Default::default()
    }

    pub fn get_or_upload<F>(&self, f: F, hardware: &TaroHardware) -> &T::UploadData
    where
        F: FnOnce() -> T::UploadData,
    {
        self.cache.get_or_init(hardware.id(), f).into_data()
    }

    pub fn upload_new(&self, data: &T, hardware: &TaroHardware) -> &T::UploadData {
        return match self
            .cache
            .get_or_init(hardware.id(), || data.new_upload(hardware))
        {
            GetOrInitData::Init(d) => d,
            GetOrInitData::Get(d) => {
                d.write_new(data, hardware);
                d
            }
        };
    }
}
