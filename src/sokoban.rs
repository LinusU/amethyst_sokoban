use amethyst::{
    assets::{AssetStorage, Loader},
    core::transform::Transform,
    ecs::prelude::*,
    input::InputBundle,
    prelude::*,
    renderer::{
        Camera, DisplayConfig, DrawFlat, Pipeline, PngFormat, PosNormTex, Projection, RenderBundle,
        SpriteRender, SpriteSheet, SpriteSheetFormat, SpriteSheetHandle, Stage, Texture,
        TextureMetadata, Transparent,
    },
    utils::application_root_dir,
};

use crate::level::Level;

pub const ARENA_WIDTH: f32 = 320.0;
pub const ARENA_HEIGHT: f32 = 256.0;

fn initialise_camera(world: &mut World) {
    let mut transform = Transform::default();
    transform.set_z(1.0);
    world
        .create_entity()
        .with(Camera::from(Projection::orthographic(
            0.0,
            ARENA_WIDTH,
            0.0,
            ARENA_HEIGHT,
        )))
        .with(transform)
        .build();
}

fn load_sprite_sheet(world: &mut World, name: &str) -> SpriteSheetHandle {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let texture_storage = world.read_resource::<AssetStorage<Texture>>();
        loader.load(
            format!("texture/{}_spritesheet.png", name),
            PngFormat,
            TextureMetadata::srgb_scale(),
            (),
            &texture_storage,
        )
    };

    let loader = world.read_resource::<Loader>();
    let sprite_sheet_store = world.read_resource::<AssetStorage<SpriteSheet>>();
    loader.load(
        format!("texture/{}_spritesheet.ron", name), // Here we load the associated ron file
        SpriteSheetFormat,
        texture_handle, // We pass it the handle of the texture we want it to use
        (),
        &sprite_sheet_store,
    )
}

fn create_ground(
    world: &mut World,
    sprite_sheet_handle: SpriteSheetHandle,
    x: usize,
    y: usize,
    sprite_number: usize,
) {
    let mut local_transform = Transform::default();
    local_transform.set_xyz(x as f32 * 16.0, y as f32 * 16.0, -100.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: sprite_number,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(local_transform)
        .build();
}

fn create_player(world: &mut World, sprite_sheet_handle: SpriteSheetHandle, x: usize, y: usize) {
    let mut local_transform = Transform::default();
    local_transform.set_xyz(x as f32 * 16.0, y as f32 * 16.0, 0.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 0,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(local_transform)
        .with(Transparent)
        .with(Player {})
        .with(Movable { moving_to: None })
        .build();
}

fn create_goal(world: &mut World, sprite_sheet_handle: SpriteSheetHandle, x: usize, y: usize) {
    let mut local_transform = Transform::default();
    local_transform.set_xyz(x as f32 * 16.0, y as f32 * 16.0, -50.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 32,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(local_transform)
        .with(Transparent)
        .build();
}

fn create_box(world: &mut World, sprite_sheet_handle: SpriteSheetHandle, x: usize, y: usize) {
    let mut local_transform = Transform::default();
    local_transform.set_xyz(x as f32 * 16.0, y as f32 * 16.0, -50.0);

    let sprite_render = SpriteRender {
        sprite_sheet: sprite_sheet_handle,
        sprite_number: 31,
    };

    world
        .create_entity()
        .with(sprite_render)
        .with(local_transform)
        .with(Transparent)
        .with(Box {})
        .with(Movable { moving_to: None })
        .build();
}

fn load_level(
    world: &mut World,
    outdoor_handle: SpriteSheetHandle,
    character_handle: SpriteSheetHandle,
) {
    // let level = Level::parse("#######\n#     #\n# @   #\n#  $  #\n#   . #\n#     #\n#######");
    let level = Level::parse("        #######\n#########     #\n#..  ##@# ### #####\n#..          $  $ #\n#..  ##### ## #   #\n######   # ## # ###\n         # $ $  #\n         ##$  ###\n          #  $#\n          #   #\n          #####");

    for (y, line) in level.ground().iter().enumerate() {
        for (x, &sprite_number) in line.iter().enumerate() {
            print!("{:x}", sprite_number);
            if sprite_number > 0 {
                create_ground(world, outdoor_handle.clone(), x, y, sprite_number)
            }
        }
        println!("");
    }

    for (x, y) in level.goals_pos() {
        create_goal(world, outdoor_handle.clone(), x, y);
    }

    for (x, y) in level.boxes_pos() {
        create_box(world, outdoor_handle.clone(), x, y);
    }

    let player_pos = level.player_pos();
    create_player(world, character_handle, player_pos.0, player_pos.1);

    world.add_resource(PlayState { level: Some(level) });
}

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

impl Direction {
    pub fn to_velocity(&self, scale: f32) -> (f32, f32) {
        match self {
            Direction::Up => (0.0, scale),
            Direction::Down => (0.0, -scale),
            Direction::Left => (-scale, 0.0),
            Direction::Right => (scale, 0.0),
        }
    }
}

pub struct Box {}

impl Component for Box {
    type Storage = DenseVecStorage<Self>;
}

pub struct Player {}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

pub struct Movable {
    pub moving_to: Option<(usize, usize, Direction)>,
}

impl Component for Movable {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Default)]
pub struct PlayState {
    pub level: Option<Level>,
}

pub struct Sokoban;

impl SimpleState for Sokoban {
    fn on_start(&mut self, data: StateData<'_, GameData<'_, '_>>) {
        let world = data.world;

        // Load the spritesheet necessary to render the graphics.
        let character_handle = load_sprite_sheet(world, "character");
        let outdoor_handle = load_sprite_sheet(world, "outdoor");

        // world.register::<Ground>(); // <- add this line temporarily

        load_level(world, outdoor_handle.clone(), character_handle.clone());
        initialise_camera(world);
    }
}
