use std::time::{Instant};

use rand::{seq::SliceRandom, thread_rng};

use crate::game::{game_state::GameState, moves::Move};
use crate::protocol::{
    manager::ProtocolManager,
    message::{ClientSideMessage, ServerSideMessage},
};
use crate::xml::enums::PlayerTeam;

pub struct Logic {
    pub current_game_state: Option<GameState>,
    pub room_id: Option<String>,
    pub last_move: Option<Move>,

    pub own_team: Option<PlayerTeam>,
}

pub enum ClientState {
    Running,
    ShouldTerminate,
}

impl Logic {
    pub fn new() -> Self {
        Self {
            current_game_state: None,
            room_id: None,
            last_move: None,
            own_team: None,
        }
    }

    fn calculate_move(&mut self) -> Option<Move> {
        let game_state = self.current_game_state.as_mut()?;
        let team = self.own_team.as_ref()?;

        log::info!("Current turn: {}", game_state.turn);
        log::info!("Current player: {:?}", game_state.get_current_team());
        log::info!("Current ambers: {:?}", game_state.ambers);

        let start_time = Instant::now();
        let possible_moves = game_state.calculate_possible_moves(&team);
        let mut rng = thread_rng();
        let sent_move = possible_moves.choose(&mut rng);

        let cloned_sent_move = sent_move.cloned()?;

        let elapsed = start_time.elapsed();
        log::info!("Calculated move: {:?}", cloned_sent_move);
        log::info!("Needed {:?} to calculate move", elapsed);

        match game_state.perform_move(&cloned_sent_move) {
            Ok(_) => {},
            Err(error) => {
                log::error!("Error while trying to perform move on game state: {:?}", error);
            },
        }

        log::info!("New Game State: \n{}", game_state.board);
        log::info!("Result: {:?}", game_state.get_result());

        Some(cloned_sent_move)
    }

    fn process_move_request(&mut self, protocol_manager: &mut ProtocolManager) -> ClientState {
        let calculated_move = self.calculate_move();
        if let Some(sent_move) = calculated_move {
            let state_room_id = self.room_id.as_ref().unwrap();
            let room_id = String::from(state_room_id);

            let message = ClientSideMessage::Move { sent_move, room_id };

            if let Err(error) = protocol_manager.send_client_side_message(message) {
                log::error!("Error while trying to send move: {:?}", error);
                return ClientState::ShouldTerminate;
            }
        }

        ClientState::Running
    }

    pub fn process_server_side_message(
        &mut self,
        protocol_manager: &mut ProtocolManager,
        message: ServerSideMessage,
    ) -> ClientState {
        match message {
            ServerSideMessage::Left => {
                log::info!("Left");
                ClientState::ShouldTerminate
            },
            ServerSideMessage::MoveRequest => {
                self.process_move_request(protocol_manager);
                ClientState::Running
            }
            ServerSideMessage::Memento { game_state } => {
                self.current_game_state = Some(game_state);
                ClientState::Running
            }
            ServerSideMessage::Result { result } => {
                log::info!("Result: {:?}", result);

                if result.winner_team == self.own_team {
                    log::info!("#1 Victory Royale");
                } else {
                    log::info!("Lost the game :(");
                }

                ClientState::Running
            }
            ServerSideMessage::WelcomeMessage { room_id, own_team } => {
                self.room_id = Some(room_id);
                self.own_team = own_team;

                log::info!("Own team: {:?}", self.own_team);
                ClientState::Running
            }
            ServerSideMessage::Error => {
                log::error!("Received error message from server!");
                ClientState::ShouldTerminate
            }
        }
    }
}
