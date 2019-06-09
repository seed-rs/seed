use actix::prelude::*;

// ---- Actor ----

pub struct CountActor(pub u32);

impl Actor for CountActor {
    type Context = Context<Self>;
}

// ---- Messages ----

pub struct MsgIncrement;

impl Message for MsgIncrement {
    type Result = u32;
}

// ---- Handlers ----

impl Handler<MsgIncrement> for CountActor {
    type Result = u32;

    fn handle(&mut self, _: MsgIncrement, _: &mut Context<Self>) -> Self::Result {
        self.0 += 1;
        self.0
    }
}
