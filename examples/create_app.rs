use boba_app::*;
use boba_winit::*;

fn main() {
    let app = BobaApp::new();
    WinitRunner::default().run(app);
}
