use ggez::event::KeyCode;
use ggez::{event, graphics, Context, GameResult};
use crate::movement::{GridPosition, Direction};

pub struct Creature {
    pos: GridPosition,
    dir: Direction,
}

impl Creature {
    pub fn new(pos: GridPosition) -> Self {
        Creature {
            pos,
            dir: Direction::Right,
        }
    }

    fn randomize_direction(&mut self) {
        self.dir = Direction::rand();
    }

    pub fn update(&mut self) {
        self.randomize_direction();
        let new_pos = GridPosition::new_from_move(self.pos, self.dir);
        self.pos = new_pos;
    }

    pub fn draw(&self, ctx: &mut Context) -> GameResult {
        let mesh = graphics::MeshBuilder::new()
            .rectangle(
                graphics::DrawMode::fill(),
                self.pos.into(),
                graphics::Color::new(0.0, 0.5, 1.0, 1.0),

            )?
            .build(ctx)?;
        graphics::draw(ctx, &mesh, graphics::DrawParam::default())?;
        Ok(())
    }
}
