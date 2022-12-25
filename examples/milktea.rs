use boba_core::BobaApp;
use milktea_runner::MilkTeaRunner;

fn main() {
    let app = BobaApp::default();
    MilkTeaRunner::run(app).unwrap();
}
