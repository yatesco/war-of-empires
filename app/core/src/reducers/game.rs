use cgmath::Vector2;
use std::collections::HashMap;

use crate::maps::{templates, Tile, World};

use super::state::{default, next, State};
use super::utils::{calculate_translation_for_view_position, tiles_for_world_view};
use super::{Actions, DEFAULT_WORLD_SCALE};

pub static STATIC_WORLD_VIEW_ITEMS: &'static str = "static_world_items";

#[derive(Clone, Debug)]
pub struct Game {
    pub elapsed_time: f64,
    pub world: World,
    pub cursor: Vector2<i32>,
    pub view_position: Vector2<i32>,
    pub views: HashMap<&'static str, Vec<Option<Tile>>>,
    pub translation: Vector2<i32>,
}

impl Default for Game {
    fn default() -> Self {
        Game {
            elapsed_time: 0.,
            world: World::new(templates::TEMPLE_MAP, DEFAULT_WORLD_SCALE),
            cursor: Vector2 { x: 0, y: 0 },
            view_position: Vector2::new(0, 0),
            views: HashMap::new(),
            translation: Vector2 { x: 0, y: 0 },
        }
    }
}

fn set_world(state: &State, world: &World) -> State {
    let mut next_state = next(state);
    next_state.next.game.world = world.clone();
    next_state = set_view_for_position(&next_state, &Vector2 { x: 0, y: 0 });
    next_state
}

fn set_cursor(state: &State, cursor: &Vector2<i32>) -> State {
    let mut next_state = next(state);
    next_state.next.game.cursor = *cursor;
    next_state
}

fn handle_key_up(state: &State, key: &String) -> State {
    match key.as_ref() {
        "w" => set_view_for_position(
            state,
            &(state.next.game.view_position - Vector2 { x: 0, y: 1 }),
        ),
        "d" => set_view_for_position(
            state,
            &(state.next.game.view_position + Vector2 { x: 1, y: 0 }),
        ),
        "s" => set_view_for_position(
            state,
            &(state.next.game.view_position + Vector2 { x: 0, y: 1 }),
        ),
        "a" => set_view_for_position(
            state,
            &(state.next.game.view_position - Vector2 { x: 1, y: 0 }),
        ),
        _ => default(state),
    }
}

fn handle_key_down(state: &State, _key: &str) -> State {
    default(&state)
}

fn set_elapsed_time(state: &State, tick: &f64) -> State {
    let mut next_state = next(state);
    next_state.next.game.elapsed_time = next_state.next.game.elapsed_time + (*tick);
    next_state
}

fn set_view_for_position(state: &State, view_position: &Vector2<i32>) -> State {
    let mut next_state = next(state);

    let mut views: HashMap<&'static str, Vec<Option<Tile>>> = HashMap::new();
    let world_view = tiles_for_world_view(state, view_position);
    
    views.insert(STATIC_WORLD_VIEW_ITEMS, world_view);

    next_state.next.game.views = views;
    next_state.next.game.view_position = *view_position;
    next_state.next.game.translation =
        calculate_translation_for_view_position(&next_state, view_position);
    next_state
}

pub fn game_reducer(state: &State, action: &Actions) -> State {
    match action {
        Actions::GameSetElapsedTime(dt) => set_elapsed_time(state, dt),
        Actions::GameSetWorld(w) => set_world(state, w),
        Actions::GameSetGameCursor(c) => set_cursor(state, c),
        Actions::GameHandleKeyUp(k) => handle_key_up(state, k),
        Actions::GameHandleKeyDown(k) => handle_key_down(state, k),
        _ => default(state),
    }
}
