pub mod world;

use crate::{
    app::{App, AppMessage},
    game::world::{World, vector::Vector},
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
    command_destination_x: Value<i32>,
    command_destination_y: Value<i32>,
    command_destination_set: Value<bool>,
    player_torpedoes_remaining: Value<i32>,
    setting_torpedo_target: Value<bool>,
    torpedo_target_x: Value<i32>,
    torpedo_target_y: Value<i32>,
    torpedo_target_set: Value<bool>,
    animating_completed_turn: Value<bool>,
}

impl Component for Game {
    type State = GameState;

    type Message = AppMessage;

    fn on_tick(
        &mut self,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        _context: anathema::component::Context<'_, '_, Self::State>,
        _dt: std::time::Duration,
    ) {
        let world = &mut self.0;

        children.elements().by_tag("canvas").first(|el, _| {
            let canvas = el.to::<Canvas>();
            let animating_turn = *state.animating_completed_turn.to_ref();

            canvas.clear();

            if animating_turn {
                let done = world.animate_turn(canvas);

                if done {
                    state.player_position_xs.set(List::from_iter(
                        world.player_positions.iter().map(|position| position.x),
                    ));
                    state.player_position_ys.set(List::from_iter(
                        world.player_positions.iter().map(|position| position.y),
                    ));
                    state.animating_completed_turn.set(false);
                    state.can_take_turn.set(true);
                    world.finish_animating();
                    world.calculate_legal_destinations();
                }
            } else {
                world.draw(canvas);
            }
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

                if world.next_turn_number < game_stream.game.turn_number
                    && world.turn != 0
                    && !*state.animating_completed_turn.to_ref()
                {
                    state.animating_completed_turn.set(true);
                    world.next_turn_number = game_stream.game.turn_number;
                    world.turns = game_stream.turns;
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

                    world.turn = game_stream.game.turn_number;
                    world.next_turn_number = world.turn;
                } else {
                    world.update_players_ready(game_stream.players);

                    state
                        .players_ready
                        .set(List::from_iter(world.players_ready.iter().copied()));
                }
            }
            AppMessage::GotPlayerFromServer(db_player) => {
                state.player_speed.set(db_player.speed);
                state
                    .player_torpedoes_remaining
                    .set(db_player.torpedo_count);
                self.0.player_updated(db_player);
                self.0.calculate_legal_destinations();

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
                dbg!("decreasing speed");
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
                let destination = if *state.command_destination_set.to_ref() {
                    let x = state.command_destination_x.to_ref();
                    let y = state.command_destination_y.to_ref();
                    Some(Vector::new(*x, *y))
                } else {
                    None
                };
                let torpedo_target = self.0.torpedo_target;
                let message = AppMessage::SubmitTurnCommand(TurnCommand {
                    speed_change,
                    destination,
                    torpedo_target,
                });

                context.components.by_name(App::ident()).send(message);
                state.can_take_turn.set(false);
                state.command_speed_change.set(0);
                state.setting_destination.set(false);
                self.0.display_possible_destinations = false;
                state.command_destination_set.set(false);
            }
            "toggle_set_destination" => {
                let setting_destination = !(*state.setting_destination.to_ref());

                state.setting_destination.set(setting_destination);
                self.0.display_possible_destinations = setting_destination;

                if !setting_destination {
                    state.command_destination_set.set(false);
                }
            }
            "toggle_set_torpedo_target" => {
                let setting_torpedo_target = !(*state.setting_torpedo_target.to_ref());

                state.setting_torpedo_target.set(setting_torpedo_target);
                self.0.setting_torpedo_target = setting_torpedo_target;

                if !setting_torpedo_target {
                    state.torpedo_target_set.set(false);
                    self.0.torpedo_target = None;
                }
            }
            _ => unreachable!(),
        }
    }

    fn on_mouse(
        &mut self,
        mouse: anathema::component::MouseEvent,
        state: &mut Self::State,
        mut children: anathema::component::Children<'_, '_>,
        mut context: anathema::component::Context<'_, '_, Self::State>,
    ) {
        let world = &mut self.0;

        if world.display_possible_destinations
            && mouse.left_up()
            && !(*state.command_destination_set.to_ref())
        {
            children
                .elements()
                .at_position(mouse.pos())
                .by_tag("canvas")
                .each(|el, _attr| {
                    let Some(canvas) = el.try_to::<Canvas>() else {
                        return;
                    };

                    let coords = canvas.translate(mouse.pos());

                    state.command_destination_x.set(coords.x as i32);
                    state.command_destination_y.set(coords.y as i32);
                    state.command_destination_set.set(true);

                    log(
                        format!("Mouse clicked in canvas on cell: {coords:#?}"),
                        &mut context,
                    );
                });
        } else if world.setting_torpedo_target
            && mouse.left_up()
            && !(*state.torpedo_target_set.to_ref())
        {
            children
                .elements()
                .at_position(mouse.pos())
                .by_tag("canvas")
                .first(|el, _attrs| {
                    let canvas = el.to::<Canvas>();
                    let coords = canvas.translate(mouse.pos());
                    let x = coords.x as i32;
                    let y = coords.y as i32;

                    state.torpedo_target_x.set(x);
                    state.torpedo_target_y.set(y);
                    world.torpedo_target = Some(Vector::new(x, y));
                    state.torpedo_target_set.set(true);
                    log(
                        format!("Setting torpedo target to be: ({x},{y})"),
                        &mut context,
                    );
                });
        } else if world.setting_torpedo_target {
            let mouse_position = Vector::new(mouse.pos().x, mouse.pos().y);

            world.mouse_position = mouse_position;
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
    pub destination: Option<Vector>,
    pub torpedo_target: Option<Vector>,
}

pub fn log<T: State>(message: String, context: &mut anathema::component::Context<'_, '_, T>) {
    context
        .components
        .by_name(App::ident())
        .send(AppMessage::Log(message));
}
