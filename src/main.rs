mod creature;
mod player;
mod movement;
mod config;

use ggez::event::KeyCode;
use ggez::{event, graphics, Context, GameResult};
use std::time::{Duration, Instant};
use crate::creature::Creature;
use crate::player::Player;
use crate::config::{MILLIS_PER_UPDATE, SCREEN_SIZE, GRID_SIZE};
use crate::movement::Direction;

struct GameState {
    asset: graphics::Image,
    player: Player,
    creature: Creature,
    gameover: bool,
    last_update: Instant,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let asset = graphics::Image::new(ctx, "/tiles/generic-rpg-Slice.png")?;
        let player_pos = (GRID_SIZE.0 / 4, GRID_SIZE.1 / 2).into();
        let creature_pos = (GRID_SIZE.0 / 4, GRID_SIZE.1 / 2).into();

        Ok(GameState {
            asset,
            player: Player::new(player_pos),
            creature: Creature::new(creature_pos),
            gameover: false,
            last_update: Instant::now(),
        })
    }
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE) {
            if !self.gameover {
                self.player.update();
                self.creature.update();
            }
            self.last_update = Instant::now();
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, [0.0, 1.0, 0.0, 1.0].into());
        self.player.draw(ctx)?;
        // self.creature.draw(ctx, self.asset)?;
        graphics::draw(ctx, &self.asset, graphics::DrawParam::default())?;
        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: ggez::input::keyboard::KeyMods,
        _repeat: bool,
    ) {
        if let Some(dir) = Direction::from_keycode(keycode) {
            self.player.set_dir(dir);
        }
    }
}

fn main() -> GameResult {
    let (mut ctx, event_loop) = ggez::ContextBuilder::new("game", "Hristiyan Genchev")
        .window_setup(ggez::conf::WindowSetup::default().title("Game!"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0 as f32, SCREEN_SIZE.1 as f32),
        )
        .build()
        .expect("Failed to build ggez context");

    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
