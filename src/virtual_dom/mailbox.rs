use std::rc::Rc;

pub struct Mailbox<Message: 'static> {
    func: Rc<dyn Fn(Option<Message>)>,
}

impl<Ms> Mailbox<Ms> {
    pub fn new(func: impl Fn(Option<Ms>) + 'static) -> Self {
        Mailbox {
            func: Rc::new(func),
        }
    }

    pub fn send(&self, message: Option<Ms>) {
        (self.func)(message)
    }
}

impl<Ms> Clone for Mailbox<Ms> {
    fn clone(&self) -> Self {
        Mailbox {
            func: self.func.clone(),
        }
    }
}
