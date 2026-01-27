use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_components::BBAppComponent;

use crate::{
    app::{App, AppMessage},
    router::Route,
};

pub struct SplashPage;

impl SplashPage {
    fn set_can_host_game(&self, state: &mut SplashPageState) {
        state
            .can_host_game
            .set(!state.player_name.to_ref().is_empty());
    }

    fn set_can_join_game(&self, state: &mut SplashPageState) {
        state
            .can_join_game
            .set(*state.can_host_game.to_ref() && !state.game_code.to_ref().is_empty());
    }
}

impl Component for SplashPage {
    type State = SplashPageState;

    type Message = ();

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        event.stop_propagation();

        match event.name() {
            "name_changed" => {
                let new_name = event.data_checked::<String>().cloned().unwrap_or_default();

                state.player_name.set(new_name);
                self.set_can_host_game(state);
                self.set_can_join_game(state);
            }
            "game_code_changed" => {
                let new_game_code = event.data_checked::<String>().cloned().unwrap_or_default();

                state.game_code.set(new_game_code);
                self.set_can_host_game(state);
                self.set_can_join_game(state);
            }
            "host_game" => {
                let player_name = state.player_name.to_ref().clone();

                context
                    .components
                    .by_name(App::ident())
                    .send(AppMessage::HostGame { player_name });
            }
            "join_game" => todo!(),
            _ => unreachable!(),
        }
    }
}

impl BBAppComponent for SplashPage {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            "splash_page",
            "templates/pages/splash.aml",
            Self,
            SplashPageState::default(),
        )?;

        Ok(())
    }
}

#[derive(Debug, State, Default)]
pub struct SplashPageState {
    can_host_game: Value<bool>,
    can_join_game: Value<bool>,
    player_name: Value<String>,
    game_code: Value<String>,
}
