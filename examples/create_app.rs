use boba::prelude::*;

fn main() {
    let app = MilkTea::new();
    let runner = BobaWindow::<BobaRenderer>::new().unwrap();
    runner.run(app);
}
