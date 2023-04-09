use crate::{
    pearls::{Link, Pearl, PearlCollection},
    BobaResources,
};

use super::DestroyPearl;

pub trait EventCommand: 'static {
    fn execute(&mut self, pearls: &mut PearlCollection, resources: &mut BobaResources);
}

#[derive(Default)]
pub struct EventCommands {
    commands: Vec<Box<dyn EventCommand>>,
}

impl EventCommands {
    #[inline]
    pub fn new() -> Self {
        Self::default()
    }

    /// Inserts a command
    pub fn insert(&mut self, command: impl EventCommand) {
        self.commands.push(Box::new(command))
    }

    /// Shortahand for inserting a `DestroyPearl` command
    pub fn destroy_pearl<P: Pearl>(&mut self, link: &Link<P>) {
        self.insert(DestroyPearl { link: *link })
    }

    /// Consumes and executes all commands
    pub fn execute(mut self, pearls: &mut PearlCollection, resources: &mut BobaResources) {
        for command in self.commands.iter_mut() {
            command.execute(pearls, resources);
        }
    }
}
