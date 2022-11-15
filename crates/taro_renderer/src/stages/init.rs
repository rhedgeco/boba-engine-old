use boba_core::{storage::ControllerStorage, BobaResources, BobaStage};
use log::warn;
use milk_tea_runner::MilkTeaWindows;

use crate::{TaroCamera, TaroCameraSettings, TaroRenderer};

pub struct TaroRendererInitStage;

impl BobaStage for TaroRendererInitStage {
    type StageData = ();

    fn run(&mut self, _: &mut ControllerStorage<Self>, resources: &mut BobaResources)
    where
        Self: 'static,
    {
        let renderer = match resources.borrow::<TaroRenderer>() {
            Ok(item) => item,
            Err(e) => {
                warn!(
                    "Skipping TaroRenderer initialization. TaroRenderer Resource Error: {:?}",
                    e
                );
                return;
            }
        };

        let windows = match resources.borrow::<MilkTeaWindows>() {
            Ok(item) => item,
            Err(e) => {
                warn!(
                    "Skipping TaroRenderer initialization. MilkTeaWindows Resource Error: {:?}",
                    e
                );
                return;
            }
        };

        let camera = TaroCamera::new(
            TaroCameraSettings {
                eye: (0.0, 1.0, 2.0).into(),
                target: (0.0, 0.0, 0.0).into(),
                up: cgmath::Vector3::unit_y(),
                aspect: 1.,
                fovy: 45.0,
                znear: 0.1,
                zfar: 100.0,
            },
            renderer.resources(),
        );

        drop(windows);
        drop(renderer);
        resources.add(camera);
    }
}
