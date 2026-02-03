use anathema::{component::Component, state::State};
use bb_anathema_components::BBAppComponent;

pub struct LobbyPage;

#[derive(Debug, State, Default)]
pub struct LobbyPageState {}

impl Component for LobbyPage {
    type State = LobbyPageState;

    type Message = ();

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
}

impl BBAppComponent for LobbyPage {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            "lobby_page",
            "templates/pages/lobby.aml",
            Self,
            LobbyPageState::default(),
        )?;

        Ok(())
    }
}
