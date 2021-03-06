mod nexus;
mod config;
mod tower;
mod enemy;
mod movement_helpers;
mod assets;
mod score_board;

use ggez::{event, graphics, Context, GameResult};
use std::time::{Duration, Instant};
use crate::config::{MILLIS_PER_UPDATE, SCREEN_SIZE, GRID_SIZE, GRID_CELL_SIZE};
use crate::nexus::Nexus;
use crate::enemy::Enemy;
use crate::assets::Assets;
use std::collections::VecDeque;
use std::{env, fs};
use std::path;
use ggez::graphics::Color;
use serde_json;
use crate::movement_helpers::{GridPosition, RectangleBorder};
use ggez::input::mouse::MouseButton;
use crate::tower::Tower;
use std::cmp::{min, max};
use crate::score_board::ScoreBoard;
use rand::Rng;
use serde_json::Value;

struct GameState {
    assets: Assets,
    map_json: serde_json::Value,
    nexus: Nexus,
    enemies: VecDeque<Enemy>,
    towers: Vec<Tower>,
    score: i32,
    honey: i32,
    lives: i32,
    ticks: i32,
    gameover: bool,
    last_update: Instant,
    score_board: ScoreBoard,
    hardness: i32,
}

impl GameState {
    pub fn new(ctx: &mut Context) -> GameResult<Self> {
        let assets = Assets::new(ctx)?;
        let map_json_file = fs::File::open(get_resource_path("map_layout.json"))
            .expect("file should open read only");
        let map_json: serde_json::Value = serde_json::from_reader(map_json_file)
            .expect("file should be proper JSON");

        let score_board = ScoreBoard::new();
        let nexus: Nexus = Nexus::new();
        let towers: Vec<Tower> = vec![
            Tower::new((16, 8).into()),
            Tower::new((32, 8).into()),
            Tower::new((48, 8).into()),
            Tower::new((52, 16).into()),
            Tower::new((32, 15).into()),
            Tower::new((32, 24).into()),
            Tower::new((48, 24).into()),
        ];

        Ok(GameState {
            assets,
            map_json,
            nexus,
            enemies: VecDeque::new(),
            score_board,
            towers,
            score: 0,
            honey: 100,
            lives: 3,
            ticks: 0,
            hardness: 1,
            gameover: false,
            last_update: Instant::now(),
        })
    }
}

fn draw_map(assets: &mut Assets, ctx: &mut Context, map_json: &mut serde_json::Value) {
    for x in 0..GRID_SIZE.0 {
        for y in 0..GRID_SIZE.1 {
            let dest: ggez::mint::Point2<f32> = GridPosition::new(x as f32, y as f32).into();
            let draw_params = graphics::DrawParam::new()
                .dest(dest);

            let key = &format!("{x}_{y}", x = x, y = y);

            {
                let tile_image = assets.get_tile_image(map_json[key]["sprite"].to_string());
                graphics::draw(ctx, tile_image, draw_params);
            }

            {
                let decor: &Value = &map_json[key]["decor"];
                if !decor.is_null() {
                    let decor_image = assets.get_decor_image(decor.to_string());
                    graphics::draw(ctx, decor_image, draw_params);
                }
            }
        }
    }
}

fn get_resources_dir() -> path::PathBuf {
    let resources_dir = if let Ok(resources_dir) = env::var("RESOURCES_DIR") {
        println!("manifest_dir: {}", resources_dir);
        let mut path = path::PathBuf::from(resources_dir);
        path
    } else {
        path::PathBuf::from("./resources")
    };

    resources_dir
}

fn get_resource_path(resource_name: &str) -> path::PathBuf {
    let resources_dir = get_resources_dir();
    let mut resources_dir_cloned = resources_dir.clone();
    resources_dir_cloned.push(resource_name);
    resources_dir_cloned
}

impl event::EventHandler<ggez::GameError> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        if Instant::now() - self.last_update >= Duration::from_millis(MILLIS_PER_UPDATE) {
            if !self.gameover {
                // update enemies
                for enemy in self.enemies.iter_mut() {
                    enemy.update();
                }

                let mut total_damage: i32 = 0;
                for tower in self.towers.iter() {
                    total_damage += tower.get_damage();
                }

                // for now all enemies will be with one speed
                // when this logic is changed make sure to update this code
                // DAMAGE ALL FRONT ENEMIES
                while total_damage > 0 && !self.enemies.is_empty() {
                    let front_enemy = self.enemies.front_mut().unwrap();
                    let front_enemy_health: i32 = front_enemy.get_health();
                    let health_to_reduce: i32 = min(front_enemy_health, total_damage);
                    front_enemy.reduce_health(health_to_reduce);

                    total_damage -= health_to_reduce;

                    if !front_enemy.is_alive() {
                        let honey_rewarded: i32 = front_enemy.get_honey_reward();
                        self.score += honey_rewarded * 3;
                        self.honey += honey_rewarded;
                        self.enemies.pop_front();
                    }
                }

                // increase the hardness on every 10 enemies
                if self.ticks % 70 == 0 {
                    self.hardness += 1;
                }

                // spawn the next enemy if its time to do so
                if self.ticks % 7 == 0 {
                    let mut rng = rand::thread_rng();
                    let health_multiply_noise = rng.gen_range(max(1, self.hardness - 2)..self.hardness + 1);
                    let health_m_noise = rng.gen_range(90..110);
                    let health_add_noise = rng.gen_range(0..10);
                    let enemy = Enemy::new(self.hardness, health_multiply_noise * health_m_noise + health_add_noise);
                    self.enemies.push_back(enemy);
                }

                // Check if any new enemies have hit the nexus - if so reduce its health
                // if the health is leq than 0 stop the game
                let first_enemy = self.enemies.front_mut();
                if first_enemy.is_some() {
                    if self.nexus.is_enemy_in(first_enemy.unwrap().get_position()) {
                        self.lives -= 1;
                        self.enemies.pop_front();
                    }
                }

                if self.lives <= 0 {
                    self.gameover = true;
                }
            }
            self.last_update = Instant::now();
            self.ticks = self.ticks + 1;
        }
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, Color::BLACK);
        {
            let assets = &mut self.assets;
            let map_json = &mut self.map_json;
            draw_map(assets, ctx, map_json);

            for enemy in self.enemies.iter() {
                enemy.draw(ctx, assets)?;
            }

            for tower in self.towers.iter() {
                tower.draw(ctx, assets)?;
            }

            self.nexus.draw(ctx, assets)?;
            self.score_board.draw(ctx, assets, self.score, self.lives, self.honey)?;
        }

        graphics::present(ctx)?;
        ggez::timer::yield_now();
        Ok(())
    }

    fn mouse_button_down_event(&mut self, ctx: &mut Context, _button: MouseButton, _x: f32, _y: f32) {
        println!("_button: {:?}, _x: {}, _y: {}", _button, _x, _y);
        let click_pos: GridPosition = (_x / GRID_CELL_SIZE.0 as f32, _y / GRID_CELL_SIZE.0 as f32).into();
        for tower in self.towers.iter_mut() {
            if tower.is_clicking_on(click_pos) {
                let honey_to_upgrade: i32 = tower.honey_to_upgrade();

                if honey_to_upgrade > self.honey {
                    // if the user doesn't have enough money nothing happens
                    return;
                }

                self.honey -= honey_to_upgrade;
                tower.upgrade();
            }
        }
    }
}

fn main() -> GameResult {
    let resources_dir = get_resources_dir();

    let (mut ctx, event_loop) = ggez::ContextBuilder::new("game", "Hristiyan Genchev")
        .window_setup(ggez::conf::WindowSetup::default().title("Game!"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(SCREEN_SIZE.0 as f32, SCREEN_SIZE.1 as f32),
        )
        .add_resource_path(resources_dir)
        .build()
        .expect("Failed to build ggez context");

    let state = GameState::new(&mut ctx)?;
    event::run(ctx, event_loop, state)
}
