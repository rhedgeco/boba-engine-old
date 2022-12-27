use boba_core::BobaApp;
use milktea_runner::BlackTeaRunner;

fn main() {
    let app = BobaApp::default();
    BlackTeaRunner::run(app).unwrap();
}
