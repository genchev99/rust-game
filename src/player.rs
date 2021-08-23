use ggez::event::KeyCode;
use ggez::{event, graphics, Context, GameResult};
use crate::movement::{GridPosition, Direction};

pub struct Player {
    pos: GridPosition,
    dir: Direction,
    last_update_dir: Direction,
}

impl Player {
    pub fn new(pos: GridPosition) -> Self {
        Player {
            pos,
            dir: Direction::Right,
            last_update_dir: Direction::Right,
        }
    }

    pub fn set_dir(&mut self, dir: Direction) {
        self.dir = dir;
    }

    pub fn update(&mut self) {
        let new_pos = GridPosition::new_from_move(self.pos, self.dir);
        self.pos = new_pos;
        self.last_update_dir = self.dir;
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let mesh = graphics::MeshBuilder::new()
            .rectangle(
                graphics::DrawMode::fill(),
                self.pos.into(),
                graphics::Color::new(1.0, 0.5, 0.0, 1.0),
            )?
            .build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        Ok(())
    }
}
