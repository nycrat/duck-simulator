use actix::prelude::*;

use crate::actors::game_server::GameServer;

#[derive(Message)]
#[rtype(result = "()")]
pub struct Update {
    pub id: u32,
    pub duck: crate::duck::Duck,
}

impl Handler<Update> for GameServer {
    type Result = MessageResult<Update>;

    fn handle(&mut self, msg: Update, _: &mut Self::Context) -> Self::Result {
        match self.ducks.get_mut(&msg.id) {
            Some(state) => {
                state.x = msg.duck.x;
                state.y = msg.duck.y;
                state.z = msg.duck.z;
                state.rotation_radians = msg.duck.rotation_radians;
            }
            None => {}
        }
        MessageResult(())
    }
}
