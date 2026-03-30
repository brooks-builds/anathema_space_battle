use crate::{
    api::GetGameByIdResponse,
    app::{App, AppMessage},
    game::log,
};
use anathema::{
    component::Component,
    state::{FromColor, List, State, Value},
};
use bb_anathema_components::BBAppComponent;

pub struct GameOver;

impl GameOver {
    pub fn name() -> &'static str {
        "game_over_page"
    }
}

#[derive(Debug, State, Default)]
pub struct GameOverState {
    game_id: Value<String>,
    player_names: Value<List<String>>,
    player_hitpoints: Value<List<i32>>,
    player_status: Value<List<String>>,
}

impl Component for GameOver {
    type State = GameOverState;

    type Message = AppMessage;

    fn on_mount(
        &mut self,
        _state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        log("game over page loaded".to_owned(), &mut context);
        let message = AppMessage::GetGame;

        context.components.by_name(App::ident()).send(message);
    }

    fn on_message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if let AppMessage::GetGameResponse(GetGameByIdResponse {
            id,
            players,
            status,
        }) = message
        {
            log("Got game info".to_owned(), &mut context);
            log(
                format!("id: {id}, players: {players:#?}, status: {status:?}"),
                &mut context,
            );

            let mut names = vec![];
            let mut hitpoints = vec![];
            let mut status = vec![];

            for player in players {
                let player_status = if player.hitpoints > 0 {
                    "alive"
                } else {
                    "dead"
                };
                names.push(player.name);
                hitpoints.push(player.hitpoints);
                status.push(player_status.to_owned());
            }

            state.game_id.set(id);
            state.player_names.set(List::from_iter(names.into_iter()));
            state
                .player_hitpoints
                .set(List::from_iter(hitpoints.into_iter()));
            state.player_status.set(List::from_iter(status.into_iter()));
        }
    }
}

impl BBAppComponent for GameOver {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            Self::name(),
            "templates/pages/game_over.aml",
            GameOver,
            GameOverState::default(),
        )?;

        Ok(())
    }
}
