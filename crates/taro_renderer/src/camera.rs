use milk_tea::boba_core::{
    pearls::{
        map::{EventData, PearlData},
        Pearl,
    },
    EventListener, EventRegistrar,
};

use crate::events::TaroRender;

#[derive(Default)]
pub struct TaroCamera {
    target: Option<String>,
}

impl Pearl for TaroCamera {
    fn register(registrar: &mut impl EventRegistrar<Self>) {
        registrar.listen_for::<TaroRender>();
    }
}

impl TaroCamera {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_target(target: &str) -> Self {
        Self {
            target: Some(target.into()),
        }
    }

    pub fn target(&self) -> Option<&str> {
        let target = self.target.as_ref()?;
        Some(target.as_str())
    }

    pub fn has_target(&self, target: &str) -> bool {
        let Some(this_target) = self.target() else { return false };
        this_target == target
    }

    pub fn set_target(&mut self, target: &str) {
        self.target = Some(target.into());
    }
}

impl EventListener<TaroRender> for TaroCamera {
    fn callback(pearl: &mut PearlData<Self>, mut event: EventData<TaroRender>) {
        if Some(event.target()) != pearl.target() {
            return;
        }

        // TODO: Render objects using encoder
        event.set_immediate_redraw();
    }
}
