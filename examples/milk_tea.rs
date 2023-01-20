use boba::prelude::*;
use taro_milk_tea::TaroGraphicsAdapter;

fn main() {
    MilkTeaApp::default().run::<TaroGraphicsAdapter>().unwrap();
}
