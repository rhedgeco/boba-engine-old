use crate::{
    pearls::{Link, Pearl, PearlManager},
    BobaResources,
};

use super::EventCommand;

pub struct DestroyPearl<P: Pearl> {
    pub link: Link<P>,
}

impl<P: Pearl> EventCommand for DestroyPearl<P> {
    fn execute(&mut self, pearls: &mut PearlManager, _: &mut BobaResources) {
        pearls.remove(&self.link);
    }
}
