use std::{
    collections::{BTreeSet, HashMap},
    f32::consts::PI,
    sync::{Arc, LazyLock},
};

use nalgebra::{SimdComplexField, Vector2, vector};
use pixels::{Pixels, SurfaceTexture};
use rand::random_range;
use winit::{
    application::ApplicationHandler,
    dpi::PhysicalSize,
    event::{DeviceEvent, DeviceId, StartCause, WindowEvent},
    event_loop::ActiveEventLoop,
    keyboard::KeyCode,
    window::{Window, WindowId},
};
use winit_input_helper::WinitInputHelper;

use crate::{
    direction::Direction,
    idx,
    input::InputAction,
    sprite::Sprite,
    tween::Tween,
    tween_manager::TweenManager,
    wave_function_collapse::{PROPAGATE_OFFSETS, WaveFunctionCollapse},
};

pub const SCREEN_WIDTH: u32 = 400;
pub const SCREEN_HEIGHT: u32 = 400;

pub const MAP_WIDTH: usize = 4;
pub const MAP_HEIGHT: usize = 4;

pub const SPRITE_WIDTH: u32 = (SCREEN_WIDTH as f32 / MAP_WIDTH as f32) as u32;
pub const SPRITE_HEIGHT: u32 = (SCREEN_WIDTH as f32 / MAP_WIDTH as f32) as u32;

pub const DEFAULT_SPRITE_WIDTH: u32 = 3;
pub const DEFAULT_SPRITE_HEIGHT: u32 = 3;
pub const DEFAULT_SPRITE_COLOR: [u8; 4] = [255, 0, 0, 255];

pub const SELECTED_SPRITE_COLOR: [u8; 4] = [0, 0, 255, 255];

pub const TILE_ROTATION_DURATION: f32 = 0.5;

pub const GAME_RUNNING_BG_COLOR: [u8; 4] = [0, 0, 0, 255];
pub const GAME_SOLVED_BG_COLOR: [u8; 4] = [0, 255, 0, 255];

pub const SOLVED_TIMER_DURATION: f32 = 2.0;

// MISC
pub static TBLR: LazyLock<BTreeSet<Direction>> = LazyLock::new(|| {
    let mut dirs = BTreeSet::new();
    dirs.insert(Direction::Up);
    dirs.insert(Direction::Down);
    dirs.insert(Direction::Left);
    dirs.insert(Direction::Right);
    dirs
});
pub static ALL_DIRS: LazyLock<Vec<BTreeSet<Direction>>> = LazyLock::new(|| {
    let mut all = Vec::new();
    all.push(TBLR.clone());

    all.push(TOP.clone());
    all.push(BOTTOM.clone());
    all.push(LEFT.clone());
    all.push(RIGHT.clone());

    all.push(TB.clone());
    all.push(TR.clone());
    all.push(TL.clone());
    all.push(LR.clone());
    all.push(BL.clone());
    all.push(BR.clone());

    all.push(TLR.clone());
    all.push(BLR.clone());
    all.push(TBL.clone());
    all.push(TBR.clone());

    all
});

pub type Tile = BTreeSet<Direction>;
pub trait Rotate {
    fn rotate(&mut self);
}
impl Rotate for Tile {
    fn rotate(&mut self) {
        let mut new_dirs = BTreeSet::new();

        for dir in self.iter() {
            let new_dir = match dir {
                Direction::Up => Direction::Right,
                Direction::Right => Direction::Down,
                Direction::Down => Direction::Left,
                Direction::Left => Direction::Up,
            };

            new_dirs.insert(new_dir);
        }

        *self = new_dirs;
    }
}

pub enum GameState {
    Running,
    Solved,
}
// ONE
pub static TOP: LazyLock<BTreeSet<Direction>> = LazyLock::new(|| {
    let mut dirs = BTreeSet::new();
    dirs.insert(Direction::Up);
    dirs
});
pub static BOTTOM: LazyLock<BTreeSet<Direction>> = LazyLock::new(|| {
    let mut dirs = BTreeSet::new();
    dirs.insert(Direction::Down);
    dirs
});
pub static LEFT: LazyLock<BTreeSet<Direction>> = LazyLock::new(|| {
    let mut dirs = BTreeSet::new();
    dirs.insert(Direction::Left);
    dirs
});
pub static RIGHT: LazyLock<BTreeSet<Direction>> = LazyLock::new(|| {
    let mut dirs = BTreeSet::new();
    dirs.insert(Direction::Right);
    dirs
});

// TWO
pub static TB: LazyLock<Tile> = LazyLock::new(|| {
    let mut dirs = BTreeSet::new();
    dirs.insert(Direction::Up);
    dirs.insert(Direction::Down);
    dirs
});
pub static TR: LazyLock<Tile> = LazyLock::new(|| {
    let mut dirs = BTreeSet::new();
    dirs.insert(Direction::Up);
    dirs.insert(Direction::Right);
    dirs
});
pub static TL: LazyLock<Tile> = LazyLock::new(|| {
    let mut dirs = BTreeSet::new();
    dirs.insert(Direction::Up);
    dirs.insert(Direction::Left);
    dirs
});
pub static LR: LazyLock<Tile> = LazyLock::new(|| {
    let mut dirs = BTreeSet::new();
    dirs.insert(Direction::Left);
    dirs.insert(Direction::Right);
    dirs
});
pub static BL: LazyLock<Tile> = LazyLock::new(|| {
    let mut dirs = BTreeSet::new();
    dirs.insert(Direction::Left);
    dirs.insert(Direction::Down);
    dirs
});
pub static BR: LazyLock<Tile> = LazyLock::new(|| {
    let mut dirs = BTreeSet::new();
    dirs.insert(Direction::Right);
    dirs.insert(Direction::Down);
    dirs
});

// THREE
pub static TLR: LazyLock<Tile> = LazyLock::new(|| {
    let mut dirs = BTreeSet::new();
    dirs.insert(Direction::Up);
    dirs.insert(Direction::Left);
    dirs.insert(Direction::Right);
    dirs
});
pub static BLR: LazyLock<Tile> = LazyLock::new(|| {
    let mut dirs = BTreeSet::new();
    dirs.insert(Direction::Down);
    dirs.insert(Direction::Left);
    dirs.insert(Direction::Right);
    dirs
});
pub static TBL: LazyLock<Tile> = LazyLock::new(|| {
    let mut dirs = BTreeSet::new();
    dirs.insert(Direction::Up);
    dirs.insert(Direction::Down);
    dirs.insert(Direction::Left);
    dirs
});
pub static TBR: LazyLock<Tile> = LazyLock::new(|| {
    let mut dirs = BTreeSet::new();
    dirs.insert(Direction::Up);
    dirs.insert(Direction::Down);
    dirs.insert(Direction::Right);
    dirs
});

// SPRITES
pub const CURRENTLY_SELECTED_SPRITE: LazyLock<Sprite> = LazyLock::new(|| {
    let mut pixels = vec![0; 4];

    for i in 0..4 {
        pixels[i] = SELECTED_SPRITE_COLOR[i];
    }

    let size = vector![1, 1];

    Sprite::new(pixels, size)
});
pub const TBLR_SPRITE: LazyLock<Sprite> = LazyLock::new(|| {
    let mut pixels = vec![0; (DEFAULT_SPRITE_WIDTH * DEFAULT_SPRITE_HEIGHT) as usize * 4];

    let positions = [
        vector![0, 1],
        vector![1, 1],
        vector![2, 1],
        vector![1, 0],
        vector![1, 2],
    ];

    for position in positions {
        let idx = idx(position.x, position.y, DEFAULT_SPRITE_WIDTH) as usize * 4;

        for i in 0..4 {
            pixels[idx + i] = DEFAULT_SPRITE_COLOR[i];
        }
    }

    let size = vector![DEFAULT_SPRITE_WIDTH, DEFAULT_SPRITE_HEIGHT];

    Sprite::new(pixels, size)
});
pub const ONE_SPRITE: LazyLock<Sprite> = LazyLock::new(|| {
    let mut pixels = vec![0; (DEFAULT_SPRITE_WIDTH * DEFAULT_SPRITE_HEIGHT) as usize * 4];

    let positions = [vector![1, 0], vector![1, 1]];

    for position in positions {
        let idx = idx(position.x, position.y, DEFAULT_SPRITE_WIDTH) as usize * 4;

        for i in 0..4 {
            pixels[idx + i] = DEFAULT_SPRITE_COLOR[i];
        }
    }

    let size = vector![DEFAULT_SPRITE_WIDTH, DEFAULT_SPRITE_HEIGHT];

    Sprite::new(pixels, size)
});
pub const CORNER_SPRITE: LazyLock<Sprite> = LazyLock::new(|| {
    let mut pixels = vec![0; (DEFAULT_SPRITE_WIDTH * DEFAULT_SPRITE_HEIGHT) as usize * 4];

    let positions = [vector![1, 0], vector![1, 1], vector![2, 1]];

    for position in positions {
        let idx = idx(position.x, position.y, DEFAULT_SPRITE_WIDTH) as usize * 4;

        for i in 0..4 {
            pixels[idx + i] = DEFAULT_SPRITE_COLOR[i];
        }
    }

    let size = vector![DEFAULT_SPRITE_WIDTH, DEFAULT_SPRITE_HEIGHT];

    Sprite::new(pixels, size)
});
pub const LONG_SPRITE: LazyLock<Sprite> = LazyLock::new(|| {
    let mut pixels = vec![0; (DEFAULT_SPRITE_WIDTH * DEFAULT_SPRITE_HEIGHT) as usize * 4];

    let positions = [vector![0, 1], vector![1, 1], vector![2, 1]];

    for position in positions {
        let idx = idx(position.x, position.y, DEFAULT_SPRITE_WIDTH) as usize * 4;

        for i in 0..4 {
            pixels[idx + i] = DEFAULT_SPRITE_COLOR[i];
        }
    }

    let size = vector![DEFAULT_SPRITE_WIDTH, DEFAULT_SPRITE_HEIGHT];

    Sprite::new(pixels, size)
});
pub const THREE_SPRITE: LazyLock<Sprite> = LazyLock::new(|| {
    let mut pixels = vec![0; (DEFAULT_SPRITE_WIDTH * DEFAULT_SPRITE_HEIGHT) as usize * 4];

    let positions = [vector![0, 1], vector![1, 1], vector![2, 1], vector![1, 0]];

    for position in positions {
        let idx = idx(position.x, position.y, DEFAULT_SPRITE_WIDTH) as usize * 4;

        for i in 0..4 {
            pixels[idx + i] = DEFAULT_SPRITE_COLOR[i];
        }
    }

    let size = vector![DEFAULT_SPRITE_WIDTH, DEFAULT_SPRITE_HEIGHT];

    Sprite::new(pixels, size)
});

pub static TILE_SPRITE_MAP: LazyLock<HashMap<Tile, (u8, f32)>> = LazyLock::new(|| {
    let mut map = HashMap::new();

    let rot_0 = 0.0;
    let rot_1 = PI / 2.0;
    let rot_2 = PI;
    let rot_3 = PI + PI / 2.0;

    let tblr = 0;
    let one = 1;
    let corner = 2;
    let long = 3;
    let three = 4;

    map.insert(TBLR.clone(), (tblr, rot_0));

    map.insert(TOP.clone(), (one, rot_0));
    map.insert(RIGHT.clone(), (one, rot_1));
    map.insert(BOTTOM.clone(), (one, rot_2));
    map.insert(LEFT.clone(), (one, rot_3));

    map.insert(TR.clone(), (corner, rot_0));
    map.insert(BR.clone(), (corner, rot_1));
    map.insert(BL.clone(), (corner, rot_2));
    map.insert(TL.clone(), (corner, rot_3));

    map.insert(LR.clone(), (long, rot_0));
    map.insert(TB.clone(), (long, rot_1));

    map.insert(TLR.clone(), (three, rot_0));
    map.insert(TBR.clone(), (three, rot_1));
    map.insert(BLR.clone(), (three, rot_2));
    map.insert(TBL.clone(), (three, rot_3));

    map
});
pub struct App<'a> {
    pub window: Option<Arc<Window>>,
    pub pixels: Option<Pixels<'a>>,
    pub input: WinitInputHelper,
    pub sprites: HashMap<u8, Sprite>,
    pub map: HashMap<Vector2<usize>, Tile>,
    pub rotation_map: HashMap<Vector2<usize>, f32>,
    pub tween_manager: TweenManager,
    pub currently_selected: Vector2<usize>,
    pub up_input: InputAction,
    pub down_input: InputAction,
    pub left_input: InputAction,
    pub right_input: InputAction,
    pub rotate_input: InputAction,
    pub map_generator: WaveFunctionCollapse,
    pub state: GameState,
    timer: f32,
}
impl<'a> App<'a> {
    pub fn new() -> Self {
        let brick_sprite = Sprite::from("src/brick.png");

        let mut sprites = HashMap::new();

        if let Ok(sprite) = brick_sprite {
            sprites.insert(u8::MAX, sprite.scale(vector![SPRITE_WIDTH, SPRITE_HEIGHT]));
        }

        sprites.insert(0, TBLR_SPRITE.clone());
        sprites.insert(1, ONE_SPRITE.clone());
        sprites.insert(2, CORNER_SPRITE.clone());
        sprites.insert(3, LONG_SPRITE.clone());
        sprites.insert(4, THREE_SPRITE.clone());
        sprites.insert(5, CURRENTLY_SELECTED_SPRITE.clone());

        let map_generator = WaveFunctionCollapse::new();

        let map = HashMap::with_capacity(MAP_WIDTH * MAP_HEIGHT);

        let rotation_map = HashMap::with_capacity(MAP_WIDTH * MAP_HEIGHT);

        let tween_manager = TweenManager::new();

        let up_input = InputAction::new(KeyCode::KeyW);
        let down_input = InputAction::new(KeyCode::KeyS);
        let left_input = InputAction::new(KeyCode::KeyA);
        let right_input = InputAction::new(KeyCode::KeyD);
        let rotate_input = InputAction::new(KeyCode::Space);

        Self {
            window: None,
            pixels: None,
            input: WinitInputHelper::new(),
            sprites,
            map,
            rotation_map,
            tween_manager,
            currently_selected: vector![0, 0],
            up_input,
            down_input,
            left_input,
            right_input,
            rotate_input,
            map_generator,
            state: GameState::Running,
            timer: 0.0,
        }
    }
    pub fn setup(&mut self) {
        self.map = self.map_generator.generate();
        self.setup_rotation_map();
    }
    fn setup_rotation_map(&mut self) {
        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let pos = vector![x, y];
                let tile = if let Some(x) = self.map.get(&pos) {
                    x
                } else {
                    println!("No tile entry for [X: {}; Y: {}]", pos.x, pos.y);
                    continue;
                };

                let rot = if let Some(x) = TILE_SPRITE_MAP.get(tile) {
                    x.1
                } else {
                    0.0
                };

                self.rotation_map.insert(pos, rot);
            }
        }
    }
    pub fn update(&mut self) {
        self.state_machine();
    }
    fn state_machine(&mut self) {
        match self.state {
            GameState::Running => self.running_coroutine(),
            GameState::Solved => self.solved_coroutine(),
        }
    }
    fn running_coroutine(&mut self) {
        self.clear(GAME_RUNNING_BG_COLOR);
        self.move_selected();
        self.rotate_selected();
        self.update_tweens();
        self.render_currently_selected();
        self.render_map();

        if self.solved() {
            self.state = GameState::Solved;
        }
    }
    fn solved_coroutine(&mut self) {
        self.clear(GAME_SOLVED_BG_COLOR);
        self.update_tweens();
        self.render_map();

        if let Some(delta) = self.input.delta_time() {
            self.timer += delta.as_secs_f32();
        }

        if self.timer > SOLVED_TIMER_DURATION {
            self.timer = 0.0;
            self.setup();
            self.state = GameState::Running;
        }
    }
    fn update_tweens(&mut self) {
        let delta_time = if let Some(x) = self.input.delta_time() {
            x.as_secs_f32()
        } else {
            return;
        };

        self.tween_manager.update(delta_time);

        for (pos, tween) in self.tween_manager.tweens.iter() {
            if let Some(rot) = self.rotation_map.get_mut(pos) {
                *rot = tween.value;
            }
        }

        self.tween_manager.clear();
    }
    fn clear(&mut self, color: [u8; 4]) {
        if let Some(pixels) = self.pixels.as_mut() {
            let frame = pixels.frame_mut();

            for pixel in frame.chunks_exact_mut(4) {
                for i in 0..4 {
                    pixel[i] = color[i];
                }
            }
        }
    }
    fn render_map(&mut self) {
        let size = vector![SPRITE_WIDTH, SPRITE_HEIGHT];

        for y in 0..MAP_HEIGHT {
            for x in 0..MAP_WIDTH {
                let pos = vector![x, y];

                let dirs = if let Some(x) = self.map.get(&pos) {
                    x
                } else {
                    return;
                };

                let (id, _) = if let Some(x) = TILE_SPRITE_MAP.get(dirs) {
                    *x
                } else {
                    return;
                };

                let rot = if let Some(x) = self.rotation_map.get(&pos) {
                    *x
                } else {
                    return;
                };

                let pos_x = (pos.x as u32 * SPRITE_WIDTH + SPRITE_WIDTH / 2) as i32;
                let pos_y = (pos.y as u32 * SPRITE_HEIGHT + SPRITE_HEIGHT / 2) as i32;

                let draw_pos = vector![pos_x, pos_y];

                self.draw_sprite(id, draw_pos, size, rot);
            }
        }
    }
    fn render_currently_selected(&mut self) {
        let size = vector![SPRITE_WIDTH, SPRITE_HEIGHT];

        let currently_selected_draw_pos_x =
            (self.currently_selected.x as u32 * SPRITE_WIDTH + SPRITE_WIDTH / 2) as i32;

        let currently_selected_draw_pos_y =
            (self.currently_selected.y as u32 * SPRITE_HEIGHT + SPRITE_HEIGHT / 2) as i32;

        let currently_selected_draw_pos =
            vector![currently_selected_draw_pos_x, currently_selected_draw_pos_y];

        self.draw_sprite(5, currently_selected_draw_pos, size, 0.0);
    }
    fn draw_sprite(&mut self, id: u8, pos: Vector2<i32>, size: Vector2<u32>, rot: f32) {
        let sprite = if let Some(x) = self.sprites.get_mut(&id) {
            x
        } else {
            return;
        };

        let pixels = if let Some(x) = self.pixels.as_mut() {
            x.frame_mut()
        } else {
            return;
        };

        let rotated = sprite.scale(size).rotate(rot);

        let half_width = rotated.size.x / 2;
        let half_height = rotated.size.y / 2;

        for sprite_x in 0..rotated.size.x {
            for sprite_y in 0..rotated.size.y {
                let pos_x = pos.x as i32 + sprite_x as i32 - half_width as i32;
                let pos_y = pos.y as i32 + sprite_y as i32 - half_height as i32;

                if pos_x >= 0
                    && pos_x < SCREEN_WIDTH as i32
                    && pos_y >= 0
                    && pos_y < SCREEN_HEIGHT as i32
                {
                    let pixels_idx = idx(pos_x, pos_y, SCREEN_WIDTH as i32) as usize * 4;
                    let sprite_idx = idx(sprite_x, sprite_y, rotated.size.x) as usize * 4;

                    if rotated.pixels[sprite_idx + 3] != 255 {
                        continue;
                    }

                    for i in 0..4 {
                        pixels[pixels_idx + i] = rotated.pixels[sprite_idx + i];
                    }
                }
            }
        }
    }
    fn move_selected(&mut self) {
        let mut new_pos = vector![
            self.currently_selected.x as i32,
            self.currently_selected.y as i32
        ];

        if self.right_input.check(&self.input) {
            new_pos.x += 1;
        }

        if self.left_input.check(&self.input) {
            new_pos.x -= 1;
        }

        if self.down_input.check(&self.input) {
            new_pos.y += 1;
        }

        if self.up_input.check(&self.input) {
            new_pos.y -= 1;
        }

        if new_pos.x < 0 {
            new_pos.x = 0;
        }

        if new_pos.x >= MAP_WIDTH as i32 {
            new_pos.x = MAP_WIDTH as i32 - 1;
        }

        if new_pos.y < 0 {
            new_pos.y = 0;
        }

        if new_pos.y >= MAP_HEIGHT as i32 {
            new_pos.y = MAP_HEIGHT as i32 - 1;
        }

        let final_pos = vector![new_pos.x as usize, new_pos.y as usize];

        self.currently_selected = final_pos;
    }
    fn rotate_selected(&mut self) {
        if let Some(tile) = self.map.get_mut(&self.currently_selected) {
            if self.rotate_input.check(&self.input) {
                let old_rot = if let Some(x) = TILE_SPRITE_MAP.get(tile) {
                    x.1
                } else {
                    return;
                };

                tile.rotate();

                let new_rot = if let Some(x) = TILE_SPRITE_MAP.get(tile) {
                    x.1
                } else {
                    return;
                };

                let tween = Tween::new(old_rot, new_rot, TILE_ROTATION_DURATION);
                self.tween_manager
                    .tweens
                    .insert(self.currently_selected, tween);
            }
        }
    }
    fn solved(&self) -> bool {
        for (pos, tile_1) in self.map.iter() {
            for (dir, offset) in PROPAGATE_OFFSETS.clone() {
                let temp_pos = vector![pos.x as i32, pos.y as i32] + offset;

                if temp_pos.x >= 0
                    && temp_pos.x < MAP_WIDTH as i32
                    && temp_pos.y >= 0
                    && temp_pos.y < MAP_HEIGHT as i32
                {
                    let final_pos = vector![temp_pos.x as usize, temp_pos.y as usize];

                    let tile_2 = if let Some(x) = self.map.get(&final_pos) {
                        x
                    } else {
                        continue;
                    };

                    let opposite_dir = dir.opposite();

                    if tile_2.contains(&opposite_dir) != tile_1.contains(&dir) {
                        return false;
                    }
                }
            }
        }

        true
    }
}
impl<'a> ApplicationHandler for App<'a> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = Arc::new(
            event_loop
                .create_window(
                    Window::default_attributes()
                        .with_inner_size(PhysicalSize::new(SCREEN_WIDTH, SCREEN_HEIGHT)),
                )
                .unwrap(),
        );

        let pixels = {
            let temp = Arc::clone(&window);
            let size = window.inner_size();
            let surface = SurfaceTexture::new(size.width, size.height, temp);
            Pixels::new(size.width, size.height, surface).unwrap()
        };

        self.window = Some(window);
        self.pixels = Some(pixels);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        self.input.process_window_event(&event);

        match event {
            WindowEvent::CloseRequested => {
                self.window = None;
                self.pixels = None;
                event_loop.exit();
            }
            WindowEvent::RedrawRequested => {
                if let Some(pixels) = self.pixels.as_mut() {
                    pixels.render().ok();
                }
            }
            _ => {}
        }
    }
    fn device_event(&mut self, _: &ActiveEventLoop, _: DeviceId, event: DeviceEvent) {
        self.input.process_device_event(&event);
    }
    fn new_events(&mut self, _: &ActiveEventLoop, _: StartCause) {
        self.input.step();
    }
    fn about_to_wait(&mut self, _: &ActiveEventLoop) {
        self.input.end_step();

        self.update();

        if let Some(window) = &self.window {
            window.request_redraw();
        }
    }
}
pub fn fill_rectangular(
    buffer: &mut [u8],
    size: Vector2<u32>,
    rectangle_size: Vector2<u32>,
    center: Vector2<u32>,
    color: [u8; 4],
) {
    let start_x = center.x - rectangle_size.x / 2;
    let start_y = center.y - rectangle_size.y / 2;

    let end_x = center.x + rectangle_size.x / 2;
    let end_y = center.y + rectangle_size.y / 2;

    for x in start_x..end_x {
        for y in start_y..end_y {
            let idx = idx(x, y, size.x) as usize * 4;

            for i in 0..4 {
                buffer[idx + i] = color[i];
            }
        }
    }
}
