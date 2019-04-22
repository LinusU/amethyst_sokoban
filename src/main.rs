use amethyst::{
    core::transform::TransformBundle,
    input::InputBundle,
    prelude::*,
    renderer::{
        Blend, ColorMask, DepthMode, DisplayConfig, DrawFlat, DrawFlat2D, Equation, Factor,
        Pipeline, PosNormTex, RenderBundle, Stage, ALPHA,
    },
    utils::application_root_dir,
};

mod level;
mod sokoban;
mod systems;

use crate::sokoban::Sokoban;

fn main() -> amethyst::Result<()> {
    amethyst::start_logger(Default::default());

    let display_path = format!("{}/resources/display_config.ron", application_root_dir());
    let config = DisplayConfig::load(&display_path);

    let binding_path = format!("{}/resources/bindings_config.ron", application_root_dir());

    let input_bundle =
        InputBundle::<String, String>::new().with_bindings_from_file(binding_path)?;

    let pipe = Pipeline::build().with_stage(
        Stage::with_backbuffer()
            .clear_target([0.00196, 0.23726, 0.21765, 1.0], 1.0)
            // .with_pass(DrawFlat::<PosNormTex>::new())
            .with_pass(DrawFlat2D::new().with_transparency(
                ColorMask::all(),
                ALPHA,
                Some(DepthMode::LessEqualWrite),
            )),
    );

    let game_data = GameDataBuilder::default()
        .with_bundle(TransformBundle::new())?
        .with_bundle(
            RenderBundle::new(pipe, Some(config))
                .with_sprite_sheet_processor()
                .with_sprite_visibility_sorting(&["transform_system"]),
        )?
        .with_bundle(input_bundle)?
        .with(systems::MoveSystem, "move_system", &[])
        .with(systems::PlayerSystem, "player_system", &["move_system"]);
    let mut game = Application::new("./", Sokoban, game_data)?;

    game.run();

    Ok(())
}
