use indexmap::{set::Drain, IndexSet};

pub struct MilkTeaCommands {
    commands: IndexSet<MilkTeaCommand>,
}

impl MilkTeaCommands {
    pub(crate) fn new() -> Self {
        Self {
            commands: IndexSet::new(),
        }
    }

    pub(crate) fn drain(&mut self) -> Drain<MilkTeaCommand> {
        self.commands.drain(..)
    }

    pub fn exit_app(&mut self) {
        self.commands.insert(MilkTeaCommand::Exit);
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) enum MilkTeaCommand {
    Exit,
    _Nothing,
}
