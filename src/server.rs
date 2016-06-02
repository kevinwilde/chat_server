use message::Message;
use roommap::RoomMap;
use usermap::UserMap;

pub struct Server {
    users: UserMap,
    rooms: RoomMap,
}

impl Server {
    fn add_user(&self) {
        unimplemented!();
    }

    fn send_message(&self, msg) {
        unimplemented!();
    }

    fn join_room(&self, user) {
        unimplemented!();
    }

    fn leave_room(&self, user) {
        unimplemented!();
    }
}