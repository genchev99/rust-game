pub const GRID_SIZE: (i16, i16) = (64, 64);
pub const GRID_CELL_SIZE: (i16, i16) = (16, 16);

pub const SCREEN_SIZE: (u32, u32) = (
    GRID_SIZE.0 as u32 * GRID_CELL_SIZE.0 as u32,
    GRID_SIZE.1 as u32 * GRID_CELL_SIZE.1 as u32,
);

pub const UPDATES_PER_SECOND: f32 = 8.0;
pub const MILLIS_PER_UPDATE: u64 = (1.0 / UPDATES_PER_SECOND * 1000.0) as u64;
