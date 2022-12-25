use crate::{BobaResources, PearlRegistry};

pub trait BobaStage: 'static {
    type Data;
    fn run(&mut self, registry: &mut PearlRegistry, resources: &mut BobaResources);
}
