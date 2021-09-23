use crate::protocol::message::ServerSideMessage;

pub fn process_server_side_message(message: ServerSideMessage) {
    match message {
        ServerSideMessage::Left => todo!(),
        ServerSideMessage::MoveRequest => todo!(),
        ServerSideMessage::Memento { game_state } => todo!(),
        ServerSideMessage::Result { result } => todo!(),
        ServerSideMessage::WelcomeMessage { room_id } => todo!(),
    }
}
