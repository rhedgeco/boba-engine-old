use std::{cell::RefCell, rc::Rc};

use uuid::Uuid;

use crate::{BobaResources, BobaStage};

pub struct Pearl<T> {
    uuid: Uuid,
    data: Rc<RefCell<T>>,
}

impl<T> Clone for Pearl<T> {
    fn clone(&self) -> Self {
        Self {
            uuid: self.uuid,
            data: self.data.clone(),
        }
    }
}

impl<T> Pearl<T> {
    pub fn build(data: T) -> Self {
        Self {
            uuid: Uuid::new_v4(),
            data: Rc::new(RefCell::new(data)),
        }
    }

    pub fn uuid(&self) -> &Uuid {
        &self.uuid
    }

    pub fn data(&self) -> &RefCell<T> {
        self.data.as_ref()
    }
}

pub trait BobaUpdate<Stage>
where
    Stage: 'static + BobaStage,
{
    fn update(&mut self, data: &Stage::StageData, resources: &mut BobaResources);
}
