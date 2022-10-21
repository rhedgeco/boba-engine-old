use crate::{crate_sealed::WrapComponentData, Component, ComponentData, ComponentType};

pub struct BobaObject {
    active: bool,
    unique_components: Vec<Component<{ ComponentType::Unique }>>,
    components: Vec<Component<{ ComponentType::Normal }>>,
}

impl Default for BobaObject {
    fn default() -> Self {
        Self {
            active: true,
            unique_components: Vec::new(),
            components: Vec::new(),
        }
    }
}

impl BobaObject {
    pub fn with_active(active: bool) -> Self {
        Self {
            active,
            unique_components: Vec::new(),
            components: Vec::new(),
        }
    }

    pub fn active(&self) -> bool {
        self.active
    }

    pub fn component_count(&self) -> usize {
        self.components.len() + self.unique_components.len()
    }

    /// Add a component to the BobaObject.
    pub fn add_component<C: ComponentData<{ ComponentType::Normal }> + 'static>(
        &mut self,
        component_data: C,
    ) {
        let new_component = Component::wrap(component_data);
        self.components.push(new_component);
        println!("Added component");
    }

    /// Add a unique component to the BobaObject.
    /// If the component already exists on the object, it is replaced by the new one.
    pub fn add_unique_component<C: ComponentData<{ ComponentType::Unique }> + 'static>(
        &mut self,
        component_data: C,
    ) {
        for component in self.unique_components.iter_mut() {
            if component.is_type::<C>() {
                component.replace_data(component_data);
                return;
            }
        }

        let new_component = Component::wrap(component_data);
        self.unique_components.push(new_component);
        println!("Added component");
    }
}
