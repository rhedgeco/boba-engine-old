pub trait BobaStage<T: 'static> {
    fn start(&mut self);
    fn data_mut(&mut self) -> &mut T;
    fn finish(&mut self);
}
