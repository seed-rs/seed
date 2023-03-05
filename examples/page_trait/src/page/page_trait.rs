use seed::prelude::*;
use std::any::Any;

pub trait PageTrait: Sized {
    type Message: 'static;

    fn init(orders: &mut impl Orders<Self::Message>) -> Self;

    fn new(orders: &mut impl Orders<Box<dyn Any>>) -> Self {
        Self::init(&mut orders.proxy(|msg| Box::new(Some(msg)) as Box<dyn Any>))
    }

    fn update(&mut self, msg: Self::Message, orders: &mut impl Orders<Self::Message>);

    fn invoke_update(&mut self, mut msg: Box<dyn Any>, orders: &mut impl Orders<Box<dyn Any>>) {

        msg.downcast_mut::<Option<Self::Message>>().and_then(Option::take)
            .map_or_else(||{
              gloo_console::error!("Msg not handled!");
            }, |msg|
            self.update(msg, &mut orders.proxy(|msg| Box::new(Some(msg)) as Box<dyn Any>)));
    }

    fn view(&self) -> Vec<Node<Self::Message>>;

    fn invoke_view(&self) -> Vec<Node<Box<dyn Any>>> {
        self.view()
            .map_msg(|msg| Box::new(Some(msg)) as Box<dyn Any>)
    }
}
