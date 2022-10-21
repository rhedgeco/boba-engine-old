use downcast_rs::Downcast;

#[derive(PartialEq, Eq)]
pub enum ComponentType {
    Normal,
    Unique,
}

#[allow(unused_variables)]
pub trait ComponentData<const TYPE: ComponentType>: Downcast {
    fn pre_update(&mut self, delta: &f64) {}
    fn update(&mut self, delta: &f64) {}
    fn post_update(&mut self, delta: &f64) {}
    fn fixed_update(&mut self, fixed_delta: &f64) {}
}

pub struct Component<const TYPE: ComponentType> {
    enabled: bool,
    data: Box<dyn ComponentData<TYPE>>,
}

impl<const TYPE: ComponentType> Component<TYPE> {
    pub fn enabled(&self) -> &bool {
        &self.enabled
    }

    pub fn set_enabled(&mut self, enabled: &bool) {
        self.enabled = *enabled
    }

    pub fn data_ref<'a>(&self) -> &(dyn ComponentData<TYPE> + 'a) {
        self.data.as_ref()
    }

    pub fn data_mut<'a>(&mut self) -> &mut (dyn ComponentData<TYPE> + 'a) {
        self.data.as_mut()
    }

    pub fn replace_data<C: ComponentData<TYPE> + 'static>(&mut self, component_data: C) {
        self.data = Box::new(component_data)
    }

    pub fn is_type<C: ComponentData<TYPE> + 'static>(&self) -> bool {
        // lol I couldnt figure out how to compare types correctly so I resorted to downcasting
        (*self.data).as_any().downcast_ref::<C>().is_some()
    }
}

pub(crate) mod crate_sealed {
    use super::ComponentType;
    use crate::{Component, ComponentData};

    pub trait WrapComponentData<const TYPE: ComponentType> {
        fn wrap<C: ComponentData<TYPE> + 'static>(component_data: C) -> Self;
    }

    impl<const TYPE: ComponentType> WrapComponentData<TYPE> for Component<TYPE> {
        fn wrap<C: ComponentData<TYPE> + 'static>(component_data: C) -> Self {
            Self {
                enabled: true,
                data: Box::new(component_data),
            }
        }
    }
}
