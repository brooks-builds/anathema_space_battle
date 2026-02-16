mod world;

use crate::{api, app::AppMessage, game::world::World};
use anathema::{
    component::Component,
    default_widgets::Canvas,
    state::{List, State, Value},
};
use bb_anathema_components::BBAppComponent;

pub struct Game(World);

impl Game {
    pub fn ident() -> &'static str {
        "game"
    }
}

#[derive(Debug, State, Default)]
pub struct GameState {
    id: Value<String>,
    width: Value<i32>,
    height: Value<i32>,
    player_ships: Value<List<char>>,
    player_ids: Value<List<String>>,
    player_names: Value<List<String>>,
    player_colors: Value<List<String>>,
    player_max_speeds: Value<List<i32>>,
    players_ready: Value<List<bool>>,
    player_position_xs: Value<List<i32>>,
    player_position_ys: Value<List<i32>>,
    player_speed: Value<i32>,
    player_ship_classnames: Value<List<String>>,
    host_id: Value<String>,
}

impl Component for Game {
    type State = GameState;

    type Message = AppMessage;

    fn on_tick(
        &mut self,
        _state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        _context: anathema::component::Context<'_, '_, Self::State>,
        _dt: std::time::Duration,
    ) {
        let world = &mut self.0;

        children.elements().by_tag("canvas").first(|el, _| {
            let canvas = el.to::<Canvas>();

            canvas.clear();
            world.draw(canvas);
        });
    }

    fn on_message(
        &mut self,
        message: Self::Message,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut _context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match message {
            AppMessage::GameUpdate(game_stream) => {
                let world = &mut self.0;

                if world.host_id != game_stream.game.host_id {
                    world.host_id = game_stream.game.host_id;

                    state.host_id.set(world.host_id.clone());
                }

                if world.no_players() {
                    let width = game_stream.game.width;
                    let height = game_stream.game.height;

                    world.set_size(width, height);
                    state.id.set(game_stream.game.id);
                    state.width.set(width);
                    state.height.set(height);

                    for player in game_stream.players {
                        world.add_player(player);
                    }

                    state
                        .player_ships
                        .set(List::from_iter(world.player_characters.iter().cloned()));
                    state
                        .player_ids
                        .set(List::from_iter(world.player_ids.iter().cloned()));
                    state
                        .player_names
                        .set(List::from_iter(world.player_names.iter().cloned()));
                    state.player_colors.set(List::from_iter(
                        world
                            .player_styles
                            .iter()
                            .map(|style| style.fg.unwrap_or_default().to_string()),
                    ));
                    state
                        .player_max_speeds
                        .set(List::from_iter(world.player_max_speeds.iter().copied()));
                    state
                        .players_ready
                        .set(List::from_iter(world.players_ready.iter().copied()));
                    state.player_position_xs.set(List::from_iter(
                        world.player_positions.iter().map(|position| position.x),
                    ));
                    state.player_position_ys.set(List::from_iter(
                        world.player_positions.iter().map(|position| position.y),
                    ));
                    state.player_ship_classnames.set(List::from_iter(
                        world.player_ship_classnames.iter().map(ToOwned::to_owned),
                    ));
                }
            }
            AppMessage::GotPlayerFromServer(db_player) => {
                state.player_speed.set(db_player.speed);
            }
            _ => unreachable!(),
        }
    }

    fn on_mount(
        &mut self,
        _state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if let Some(token) = context
            .attribute("player_token")
            .and_then(|value| value.as_str())
        {
            api::get_player(context.widget_id, context.emitter.clone(), token.to_owned());
        }
    }
}

impl BBAppComponent for Game {
    fn register_to(
        builder: &mut anathema::runtime::Builder<()>,
    ) -> Result<(), anathema::runtime::Error> {
        builder.component(
            Self::ident(),
            "templates/game.aml",
            Self(World::default()),
            GameState::default(),
        )?;

        Ok(())
    }
}
