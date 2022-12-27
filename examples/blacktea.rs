use blacktea_runner::BlackTeaRunner;
use boba_core::BobaApp;

fn main() {
    let app = BobaApp::default();
    BlackTeaRunner::run(app).unwrap();
}
