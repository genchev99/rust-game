use ggez::{graphics, Context, GameResult};
use crate::movement_helpers::{GridPosition, Direction};
use crate::assets::Assets;
use ggez::graphics::Color;
use rand::Rng;

const STARTING_POINT: (i16, i16) = (0, 4);
const CHECKPOINTS: [(i16, i16); 7] = [
    (55, 4),
    (55, 27),
    (7, 27),
    (7, 10),
    (47, 10),
    (47, 18),
    (14, 18),
];

pub struct Enemy {
    checkpoint_index: usize,
    position: GridPosition,
    speed: f32,
    health: i32,
    hardness: i32,
}

impl Enemy {
    pub fn new(hardness: i32, health: i32) -> Self {
        Enemy {
            checkpoint_index: 0,
            position: STARTING_POINT.into(),
            health,
            speed: 1.0,
            hardness,
        }
    }

    fn get_direction(&self) -> Option<Direction> {
        let current_position: (f32, f32) = self.position.into();
        let checkpoint: (i16, i16) = CHECKPOINTS[self.checkpoint_index];
        let div: (f32, f32) = (current_position.0 - checkpoint.0 as f32, current_position.1 - checkpoint.1 as f32);

        return if div.0 < 0.0 { Some(Direction::Right) } else if div.0 > 0.0 { Some(Direction::Left) } else if div.1 < 0.0 { Some(Direction::Down) } else if div.1 > 0.0 { Some(Direction::Up) } else { None };
    }

    fn step(&mut self) {
        let new_direction: Option<Direction> = self.get_direction();

        if new_direction.is_none() {
            self.checkpoint_index += 1;
            return;
        }

        let new_position: GridPosition = GridPosition::new_from_move(self.position, new_direction.unwrap(), self.speed);
        self.position = new_position;
    }

    pub fn update(&mut self) {
        self.step()
    }

    pub fn reduce_health(&mut self, reduce_by: i32) {
        self.health -= reduce_by;
    }

    pub fn get_health(&self) -> i32 {
        self.health
    }

    pub fn get_honey_reward(&self) -> i32 {
        let mut rng = rand::thread_rng();
        let reward_noise: i32 = rng.gen_range(70..130);
        self.hardness * reward_noise
    }

    pub fn is_alive(&self) -> bool {
        return self.health > 0;
    }

    pub fn get_position(&self) -> GridPosition {
        self.position
    }

    pub fn draw(&self, ctx: &mut Context, assets: &mut Assets) -> GameResult {
        let current_position: (f32, f32) = self.position.into();
        let enemy_sprite = assets.get_enemy_image("default".to_string());
        let enemy_sprite_dest: ggez::mint::Point2<f32> = self.position.into();
        let enemy_draw_params = graphics::DrawParam::new().dest(enemy_sprite_dest);

        let health_position: GridPosition = (current_position.0 - 1 as f32, current_position.1 + 3 as f32).into();
        let health_str = format!("Health: {}", self.health);
        let health_display = graphics::Text::new((health_str));
        let health_dest: ggez::mint::Point2<f32> = health_position.into();

        graphics::draw(ctx, enemy_sprite, enemy_draw_params);
        graphics::draw(ctx, &health_display, (health_dest, 0.0, Color::WHITE))?;
        Ok(())
    }
}
