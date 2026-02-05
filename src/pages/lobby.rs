use anathema::{
    component::Component,
    state::{List, State, Value},
};
use bb_anathema_components::BBAppComponent;

use crate::app::{App, AppMessage};

#[derive(Default)]
pub struct LobbyPage;

#[derive(Debug, State, Default)]
pub struct LobbyPageState {
    player_names: Value<List<String>>,
    player_colors: Value<List<String>>,
    player_ships: Value<List<char>>,
    player_ready: Value<List<bool>>,
}

impl Component for LobbyPage {
    type State = LobbyPageState;

    type Message = AppMessage;

    fn on_mount(
        &mut self,
        _state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        let Some(_player_name) = context
            .attribute("player_name")
            .and_then(|value| value.as_str())
        else {
            dbg!("got to the lobby without a player name");
            return;
        };
        if let Some(_game_code) = context
            .attribute("game_code")
            .and_then(|value| value.as_str())
        {
            // join game
        } else {
            // create game
        }
    }

    fn on_message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if let AppMessage::LobbyUpdate(lobby_stream) = message {
            let player_names_iter = lobby_stream
                .players
                .iter()
                .map(|player| player.name.to_owned());
            let player_colors_iter = lobby_stream
                .players
                .iter()
                .map(|player| player.ship_color.to_owned());
            let player_ships_iter = lobby_stream
                .players
                .iter()
                .map(|player| player.ship_character);
            let player_ready = lobby_stream.players.iter().map(|player| player.ready);

            state.player_names.set(List::from_iter(player_names_iter));
            state.player_colors.set(List::from_iter(player_colors_iter));
            state.player_ships.set(List::from_iter(player_ships_iter));
            state.player_ready.set(List::from_iter(player_ready));
        }
    }

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        _state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match event.name() {
            "set_ship_color" => {
                event.stop_propagation();

                let Some(ship_color) = event.data_checked::<String>().cloned() else {
                    return;
                };

                let message = AppMessage::ChangingShipColor(ship_color);

                context.components.by_name(App::ident()).send(message);
            }
            "set_ship" => {
                event.stop_propagation();

                let Some(ship_name) = event.data_checked::<String>() else {
                    eprintln!("Ship name not set on button value");
                    return;
                };
                let message = AppMessage::ChangeShip(ship_name.clone());

                context.components.by_name(App::ident()).send(message);
            }
            _ => unreachable!(),
        }
    }
}

impl BBAppComponent for LobbyPage {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            Self::ident(),
            "templates/pages/lobby.aml",
            LobbyPage,
            LobbyPageState::default(),
        )?;

        Ok(())
    }
}

impl LobbyPage {
    pub fn ident() -> &'static str {
        "lobby_page"
    }
}
