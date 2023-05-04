use indexmap::{set::Drain, IndexSet};

pub struct Commands {
    commands: IndexSet<Command>,
}

impl Commands {
    pub(crate) fn new() -> Self {
        Self {
            commands: IndexSet::new(),
        }
    }

    pub(crate) fn drain(&mut self) -> Drain<Command> {
        self.commands.drain(..)
    }

    pub fn exit_app(&mut self) {
        self.commands.insert(Command::Exit);
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) enum Command {
    Exit,
    _Nothing,
}
