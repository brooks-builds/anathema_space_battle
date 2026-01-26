use anathema::{
    component::Component,
    state::{State, Value},
};
use bb_anathema_components::BBAppComponent;

pub struct App;

#[derive(Debug, State, Default)]
pub struct AppState {
    width: Value<u16>,
    height: Value<u16>,
    started: Value<bool>,
}

impl Component for App {
    type State = AppState;

    type Message = ();

    fn on_mount(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        let viewport = context.viewport.size();

        state.width.set(viewport.width);
        state.height.set(viewport.height);
    }

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if event.name() == "start_game" {
            let started = *state.started.to_ref();

            state.started.set(!started);
        }
    }
}

impl BBAppComponent for App {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component("app", "templates/app.aml", Self, AppState::default())?;

        Ok(())
    }
}
