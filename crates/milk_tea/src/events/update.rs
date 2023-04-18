use indexmap::IndexSet;

pub struct Update {
    delta_time: f64,
    pub(crate) commands: IndexSet<MilkTeaCommand>,
}

impl Update {
    pub(crate) fn new(delta_time: f64) -> Self {
        Self {
            delta_time,
            commands: IndexSet::new(),
        }
    }

    pub fn delta_time(&self) -> f64 {
        self.delta_time
    }

    pub fn resize_windoe(&mut self, width: u32, height: u32) {
        self.commands.insert(MilkTeaCommand::Resize(width, height));
    }

    pub fn exit_application(&mut self) {
        self.commands.insert(MilkTeaCommand::Exit);
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub(crate) enum MilkTeaCommand {
    Resize(u32, u32),
    Exit,
}
