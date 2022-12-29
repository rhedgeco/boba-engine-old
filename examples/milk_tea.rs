use milk_tea::Bobarista;
use taro_renderer::adapters::TaroMilkTea;

fn main() {
    Bobarista::<TaroMilkTea>::default().run().unwrap();
}
