use crate::{
    pearls::{Link, Pearl, PearlManager},
    BobaResources,
};

use super::DestroyPearl;

pub trait EventCommand: 'static {
    fn execute(&mut self, pearls: &mut PearlManager, resources: &mut BobaResources);
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
    pub fn execute(self, pearls: &mut PearlManager, resources: &mut BobaResources) {
        for mut command in self.commands.into_iter() {
            command.execute(pearls, resources);
        }
    }
}

#[derive(Default)]
pub struct CommandCollection {
    commands: Vec<EventCommands>,
}

impl CommandCollection {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn create(&mut self) -> &mut EventCommands {
        let index = self.commands.len();
        self.commands.push(EventCommands::new());
        &mut self.commands[index]
    }

    pub fn execute(self, pearls: &mut PearlManager, resources: &mut BobaResources) {
        for commands in self.commands.into_iter() {
            commands.execute(pearls, resources);
        }
    }
}
