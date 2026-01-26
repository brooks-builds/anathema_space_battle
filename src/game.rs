mod world;

use crate::game::world::World;
use anathema::{component::Component, default_widgets::Canvas, state::State};
use bb_anathema_components::BBAppComponent;

pub struct Game(World);

#[derive(Debug, State, Default)]
pub struct GameState {}

impl Component for Game {
    type State = GameState;

    type Message = ();

    fn on_tick(
        &mut self,
        _state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        context: anathema::component::Context<'_, '_, Self::State>,
        _dt: std::time::Duration,
    ) {
        let Some(width) = context
            .attribute("width")
            .and_then(|value| value.to_int().map(|width| width as i32))
        else {
            return;
        };
        let Some(height) = context
            .attribute("height")
            .and_then(|value| value.to_int().map(|height| height as i32))
        else {
            return;
        };

        if width == 0 || height == 0 {
            return;
        }

        let world = &mut self.0;

        world.set_size(width, height);

        children.elements().by_tag("canvas").first(|el, _| {
            let canvas = el.to::<Canvas>();

            canvas.clear();
            world.print_grid(canvas);
        });
    }
}

impl BBAppComponent for Game {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            "game",
            "templates/game.aml",
            Self(World::default()),
            GameState::default(),
        )?;

        Ok(())
    }
}
