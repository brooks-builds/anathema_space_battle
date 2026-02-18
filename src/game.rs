mod world;

use crate::{
    app::{App, AppMessage},
    game::world::World,
};
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
    command_speed_change: Value<i8>,
    can_take_turn: Value<bool>,
    setting_destination: Value<bool>,
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
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match message {
            AppMessage::GameUpdate(game_stream) => {
                let world = &mut self.0;

                if world.turn < game_stream.game.turn_number {
                    world.turn = game_stream.game.turn_number;
                    state.can_take_turn.set(true);
                    context
                        .components
                        .by_name(App::ident())
                        .send(AppMessage::Log(format!("{world:#?}")));
                }

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

                    context
                        .components
                        .by_name(App::ident())
                        .send(AppMessage::Log(format!("world initiated: {world:#?}")));
                } else {
                    world.update_players_ready(game_stream.players);

                    state
                        .players_ready
                        .set(List::from_iter(world.players_ready.iter().copied()));
                }
            }
            AppMessage::GotPlayerFromServer(db_player) => {
                state.player_speed.set(db_player.speed);
                self.0.player_updated(db_player);

                context
                    .components
                    .by_name(App::ident())
                    .send(AppMessage::Log(format!(
                        "Calculated possible destinations for the player: \n{:#?}",
                        self.0.possible_destinations
                    )));
            }
            AppMessage::GameTurnSubmitted => {
                context
                    .components
                    .by_name(App::ident())
                    .send(AppMessage::GetPlayerFromServer);
            }
            _ => unreachable!(),
        }
    }

    fn on_mount(
        &mut self,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        context
            .components
            .by_name(App::ident())
            .send(AppMessage::GetPlayerFromServer);
        state.can_take_turn.set(true);
    }

    fn on_event(
        &mut self,
        event: &mut anathema::component::UserEvent<'_>,
        state: &mut Self::State,
        mut _children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        match event.name() {
            "decrease_speed" => {
                let mut current_speed_change = *state.command_speed_change.to_ref();
                let current_speed = *state.player_speed.to_ref();
                let min_speed_change = if current_speed == 0 { 0 } else { -1 };

                current_speed_change -= 1;

                state
                    .command_speed_change
                    .set(current_speed_change.clamp(min_speed_change, 1));
            }
            "reset_speed_change" => state.command_speed_change.set(0),
            "increase_speed" => {
                dbg!("increasing speed");
                let mut current_speed_change = *state.command_speed_change.to_ref();

                current_speed_change += 1;
                current_speed_change = current_speed_change.clamp(-1, 1);

                state.command_speed_change.set(current_speed_change);
                log(
                    format!("Increasing speed to {current_speed_change}"),
                    &mut context,
                );
            }
            "submit_command" => {
                let speed_change = *state.command_speed_change.to_ref();
                let message = AppMessage::SubmitTurnCommand(TurnCommand { speed_change });

                context.components.by_name(App::ident()).send(message);
                state.can_take_turn.set(false);
                state.command_speed_change.set(0);
            }
            "toggle_set_destination" => {
                let setting_destination = *state.setting_destination.to_ref();

                state.setting_destination.set(!setting_destination);
            }
            _ => unreachable!(),
        }
    }

    fn on_mouse(
        &mut self,
        mouse: anathema::component::MouseEvent,
        _state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        if mouse.left_up() {
            let position = mouse.pos();

            children.elements().at_position(position).each(|el, _attr| {
                if let Some(canvas) = el.try_to::<Canvas>() {
                    let local_position = canvas.translate(position);
                    let log_message = format!("Clicked on coordinates: {local_position:#?}");

                    context
                        .components
                        .by_name(App::ident())
                        .send(AppMessage::Log(log_message));
                };
            });
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

#[derive(Debug)]
pub struct TurnCommand {
    pub speed_change: i8,
}

pub fn log<T: State>(message: String, context: &mut anathema::component::Context<'_, '_, T>) {
    context
        .components
        .by_name(App::ident())
        .send(AppMessage::Log(message));
}
