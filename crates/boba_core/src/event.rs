use crate::{
    pearl::{map::PearlData, Pearl},
    BobaEventData,
};

pub trait Event: 'static {
    type Data<'a>;
}

pub trait EventListener<E: Event>: Pearl {
    fn callback(pearl: &mut PearlData<Self>, data: BobaEventData<E>);
}

pub trait EventRegistrar<P: Pearl> {
    fn listen_for<E: Event>(&mut self)
    where
        P: EventListener<E>;
}
