use milk_tea::MilkTeaApp;
use taro_renderer::adapters::TaroMilkTea;

fn main() {
    MilkTeaApp::<TaroMilkTea>::default().run().unwrap();
}
