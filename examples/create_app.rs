use boba::prelude::*;

fn main() {
    let mut app = BobaApp::new();
    WinitRunner::default().run(&mut app);
}
