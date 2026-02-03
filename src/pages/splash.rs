use crate::app::{App, AppMessage};
use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_components::BBAppComponent;

pub struct SplashPage;

impl SplashPage {
    fn can_begin(&self, state: &mut SplashPageState) {
        state.can_begin.set(!state.player_name.to_ref().is_empty());
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
                self.can_begin(state);
            }
            "begin" => {
                let name = state.player_name.to_ref();

                if name.is_empty() {
                    return;
                }

                let message = AppMessage::NameSet(name.clone());
                context.components.by_name(App::ident()).send(message);
            }
            _ => unreachable!(),
        }
    }

    fn on_key(
        &mut self,
        key: anathema::component::KeyEvent,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if matches!(key.code, anathema::component::KeyCode::Enter) {
            let name = state.player_name.to_ref();
            let message = AppMessage::NameSet(name.clone());
            context.components.by_name(App::ident()).send(message);
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
    player_name: Value<String>,
    can_begin: Value<bool>,
}
