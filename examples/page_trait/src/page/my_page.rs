use seed::{prelude::*, *};
use super::PageTrait;

#[allow(dead_code)]
pub struct MyPage {
    text: &'static str,
    received_text: Option<&'static str>,
    receiver_handle: SubHandle,
}

pub enum Msg {
    Clicked,
    TextReceived(&'static str),
}

impl PageTrait for MyPage {
    type Message = Msg;

    fn init(orders: &mut impl Orders<Self::Message>) -> Self {
        Self {
            text: "",
            received_text: None,
            receiver_handle: orders.subscribe_with_handle(Msg::TextReceived),
        }
    }

    fn update(&mut self, msg: Self::Message, _orders: &mut impl Orders<Self::Message>) {
        match msg {
            Msg::Clicked => self.text = "MyPage button clicked!",
            Msg::TextReceived(text) => self.received_text = Some(text),
        }
    }

    fn view(&self) -> Vec<Node<Self::Message>> {
        vec![
            button![
                "MyPage button",
                ev(Ev::Click, |_| Msg::Clicked)
            ],
            plain![self.text],
            div![
                &self.received_text,
            ]
        ]
    }
}
