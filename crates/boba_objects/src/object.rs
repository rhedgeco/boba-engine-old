use uuid::Uuid;

use crate::{BobaComponent, BobaWorld, ComponentLink, RegisteredUpdater, UntypedComponentLink};

pub struct BobaObject {
    pub(crate) uuid: Uuid,
    pub(crate) components: Vec<UntypedComponentLink>,
}

impl BobaObject {
    pub fn connect_component<T: 'static + RegisteredUpdater>(
        &mut self,
        world: &mut BobaWorld,
        updater: T,
    ) -> ComponentLink<T> {
        let world_index = world.components.len();
        let component = BobaComponent::new(updater);
        let typed_link: ComponentLink<T> = ComponentLink::new(component.uuid, world_index);
        let untyped_link = UntypedComponentLink {
            uuid: component.uuid,
            world_index,
        };

        world.components.push(Some(component));
        self.components.push(untyped_link);

        typed_link
    }
}
