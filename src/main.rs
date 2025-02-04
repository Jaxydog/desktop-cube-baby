// Disable the console in release builds.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::process::ExitCode;

use bevy::asset::embedded_asset;
use bevy::asset::io::embedded::EmbeddedAssetRegistry;
use bevy::image::ImageSampler;
use bevy::prelude::*;
use bevy::window::{
    CompositeAlphaMode, EnabledButtons, ExitCondition, PresentMode, PrimaryWindow, WindowLevel, WindowResolution,
};
use bevy::winit::{UpdateMode, WinitSettings, WinitWindows};

use self::components::{CubeBaby, Distance, Position, PushDelay, Velocity};
use self::resources::{DisplayProperties, TextureMetadata};
use self::states::{ApplicationLoadingMarker, DisplayLoadingMarker, LoadingState, TextureLoadingMarker};

pub mod components;
pub mod resources;
pub mod states;

/// The number of frames in the baby's texture atlas animation.
pub const ATLAS_FRAMES: u32 = 8;
/// The image scale of the sprite.
pub const SPRITE_SCALE: f32 = 2.0;
/// The size of one side of the spawned window.
pub const WINDOW_SIZE: f32 = 32.0 * SPRITE_SCALE;
/// The strength that the cube baby is pushed at when touched by the cursor.
pub const PUSH_STRENGTH: f32 = 16.0;
/// The amount of time in seconds between possible cube baby pushes.
pub const PUSH_DELAY: f64 = 0.25;
/// The amount of drag applied whilst sliding.
pub const SLIDE_DRAG: f32 = 0.25;
/// The distance required before updating the cube baby's sprite.
pub const SLIDE_SPIN_DISTANCE: f32 = 10.0;

/// Returns a new settings object for the primary window of this application.
#[inline]
pub fn window_settings() -> Window {
    Window {
        present_mode: PresentMode::Mailbox,
        resolution: WindowResolution::new(WINDOW_SIZE, WINDOW_SIZE),
        title: env!("CARGO_BIN_NAME").to_string(),
        composite_alpha_mode: if cfg!(target_os = "linux") {
            CompositeAlphaMode::PreMultiplied
        } else if cfg!(target_os = "macos") {
            CompositeAlphaMode::PostMultiplied
        } else {
            CompositeAlphaMode::Auto
        },
        resize_constraints: WindowResizeConstraints {
            min_width: WINDOW_SIZE,
            min_height: WINDOW_SIZE,
            max_width: WINDOW_SIZE,
            max_height: WINDOW_SIZE,
        },
        resizable: false,
        enabled_buttons: EnabledButtons { minimize: false, maximize: false, close: false },
        decorations: false,
        transparent: true,
        focused: true,
        window_level: WindowLevel::AlwaysOnTop,
        visible: false,
        has_shadow: false,
        titlebar_shown: false,
        ..Window::default()
    }
}

/// The application's entrypoint.
pub fn main() -> ExitCode {
    let mut application = App::new();

    // Initialize required components on startup.
    application.add_plugins(DefaultPlugins.set(WindowPlugin {
        primary_window: Some(self::window_settings()),
        exit_condition: ExitCondition::OnPrimaryClosed,
        close_when_requested: true,
    }));
    application.insert_resource(WinitSettings {
        focused_mode: UpdateMode::Continuous,
        unfocused_mode: UpdateMode::Continuous,
    });
    application.add_systems(Startup, self::startup_initialize);

    // Handle display property loading.
    application.init_state::<LoadingState<DisplayLoadingMarker>>();
    application.init_resource::<DisplayProperties>();
    application.add_systems(Update, {
        // Attempt to update the display properties until fully loaded.
        self::update_display_loading.run_if(in_state(LoadingState::<DisplayLoadingMarker>::loading()))
    });

    // Handle texture asset loading.
    application.init_state::<LoadingState<TextureLoadingMarker>>();
    application.init_resource::<EmbeddedAssetRegistry>();
    application.add_systems(Update, {
        // Attempt to update the texture assets until fully loaded.
        self::update_texture_loading.run_if(in_state(LoadingState::<TextureLoadingMarker>::loading()))
    });

    embedded_asset!(application, "cube_baby.png");

    // Handle application-wide loading state.
    application.init_state::<LoadingState<ApplicationLoadingMarker>>();
    application.add_systems(Update, {
        // Attempt to update the application loading state until fully loaded.
        self::update_application_loading.run_if(in_state(LoadingState::<ApplicationLoadingMarker>::loading()))
    });
    application.add_systems(OnEnter(LoadingState::<ApplicationLoadingMarker>::finished()), {
        // Handle final registration of components.
        self::on_application_load_finished
    });

    // Handle rendering and window motion.
    application.insert_resource(ClearColor(Color::NONE));
    application.add_systems(FixedUpdate, {
        // Handle cursor-to-window collision.
        fixed_update_mouse_collision.run_if(in_state(LoadingState::<ApplicationLoadingMarker>::finished()))
    });
    application.add_systems(FixedUpdate, {
        // Handle space-bar knocking.
        fixed_update_spacebar_knocking.run_if(in_state(LoadingState::<ApplicationLoadingMarker>::finished()))
    });
    application.add_systems(Update, {
        // Handle moving the window.
        update_window_movement.run_if(in_state(LoadingState::<ApplicationLoadingMarker>::finished()))
    });
    application.add_systems(Update, {
        // Handle rotating the cube baby.
        update_sprite_rotation.run_if(in_state(LoadingState::<ApplicationLoadingMarker>::finished()))
    });

    // Return an exit code that is representative of the execution's result.
    match application.run() {
        AppExit::Success => ExitCode::SUCCESS,
        AppExit::Error(non_zero) => ExitCode::from(non_zero.get()),
    }
}

/// Initializes components on startup.
pub fn startup_initialize(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2d);

    commands.insert_resource(TextureMetadata {
        image_handle: asset_server.load(concat!("embedded://", env!("CARGO_CRATE_NAME"), "/cube_baby.png")),
        layout_handle: Handle::default(),
        size: UVec2::ZERO,
    });
}

/// Attempts to load the current display's properties on application load.
pub fn update_display_loading(
    primary_window: Single<Entity, With<PrimaryWindow>>,
    winit_windows: NonSend<WinitWindows>,
    mut display_properties: ResMut<DisplayProperties>,
    mut display_state: ResMut<NextState<LoadingState<DisplayLoadingMarker>>>,
) {
    if let Some(current_monitor) = winit_windows.get_window(*primary_window).and_then(|v| v.current_monitor()) {
        display_properties.position = IVec2::new(current_monitor.position().x, current_monitor.position().y);
        display_properties.resolution = UVec2::new(current_monitor.size().width, current_monitor.size().height);

        display_state.set(LoadingState::finished());
    }
}

/// Attempts to load the assets related to all required textures on application load.
pub fn update_texture_loading(
    asset_server: Res<AssetServer>,
    mut image_assets: ResMut<Assets<Image>>,
    mut layout_assets: ResMut<Assets<TextureAtlasLayout>>,
    mut texture_metadata: ResMut<TextureMetadata>,
    mut texture_state: ResMut<NextState<LoadingState<TextureLoadingMarker>>>,
) {
    if asset_server.is_loaded(&texture_metadata.image_handle) {
        let image = image_assets.get_mut(&texture_metadata.image_handle).expect("failed to resolve image");

        image.sampler = ImageSampler::nearest();

        texture_metadata.size = image.size();

        let layout = TextureAtlasLayout::from_grid(texture_metadata.frame_size(), ATLAS_FRAMES, 1, None, None);

        texture_metadata.layout_handle = layout_assets.add(layout);

        texture_state.set(LoadingState::finished());
    }
}

/// Updates the application's loading state to reflect whether all values are loaded.
pub fn update_application_loading(
    display_state: Res<State<LoadingState<DisplayLoadingMarker>>>,
    texture_state: Res<State<LoadingState<TextureLoadingMarker>>>,
    mut application_state: ResMut<NextState<LoadingState<ApplicationLoadingMarker>>>,
) {
    if display_state.get().is_finished() && texture_state.get().is_finished() {
        application_state.set(LoadingState::finished());
    }
}

/// Finishes initializing the application once all prerequisite loading has finished.
pub fn on_application_load_finished(
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    mut commands: Commands,
    display_properties: Res<DisplayProperties>,
    texture_metadata: Res<TextureMetadata>,
) {
    let texture_atlas = TextureAtlas { index: 0, layout: texture_metadata.layout_handle.clone_weak() };
    let sprite = Sprite::from_atlas_image(texture_metadata.image_handle.clone_weak(), texture_atlas);
    let transform = Transform::from_scale(texture_metadata.sprite_scale().xyy());
    let position = Position(display_properties.center_position().as_vec2() - (WINDOW_SIZE / 2.0));

    commands.spawn((CubeBaby, sprite, transform, position, Velocity::ZERO, PushDelay::ZERO, Distance::ZERO));

    window.position.set(position.round().as_ivec2());
    window.visible = true;
}

/// Handles knocking the cube baby when the space bar is pressed.
pub fn fixed_update_spacebar_knocking(
    button_input: Res<ButtonInput<KeyCode>>,
    mut velocity: Single<&mut Velocity, With<CubeBaby>>,
) {
    const MAX_STRENGTH: f32 = PUSH_STRENGTH * PUSH_STRENGTH * 4.0;

    if button_input.just_pressed(KeyCode::Space) {
        let x = (fastrand::f32() * 2.0) - 1.0;
        let y = (fastrand::f32() * 2.0) - 1.0;
        let strength = ((fastrand::f32() * MAX_STRENGTH) - PUSH_STRENGTH) + PUSH_STRENGTH;

        velocity.0 += Vec2::new(x, y).normalize_or_zero() * strength * SPRITE_SCALE;
    }
}

/// Handles updating the cube baby's velocity based off of mouse interactions.
pub fn fixed_update_mouse_collision(
    time: Res<Time>,
    query: Single<(&mut Velocity, &mut PushDelay), With<CubeBaby>>,
    mut cursor_moved_events: EventReader<CursorMoved>,
) {
    let (mut velocity, mut push_delay) = query.into_inner();

    if *push_delay > PushDelay::ZERO {
        push_delay.0 -= time.delta_secs_f64();

        return;
    }

    // We only care about the start and end positions, which are used to roughly gauge the push direction.
    let mut event_iterator = cursor_moved_events.read().map(|v| v.position);
    let start_position = event_iterator.next();
    let final_position = event_iterator.last();

    if let Some((start_position, final_position)) = start_position.zip(final_position) {
        let delta_position = final_position - start_position;
        let mut delta_position = delta_position * PUSH_STRENGTH * SPRITE_SCALE;

        // Ensure that the cube baby is always pushed with a minimum strength.
        if delta_position.length() < PUSH_STRENGTH * SPRITE_SCALE {
            delta_position = delta_position.normalize_or_zero() * PUSH_STRENGTH * SPRITE_SCALE;
        }

        velocity.0 += delta_position;
        push_delay.0 = PUSH_DELAY;
    }
}

/// Updates the window's position to follow the current velocity.
pub fn update_window_movement(
    mut window: Single<&mut Window, With<PrimaryWindow>>,
    time: Res<Time>,
    query: Single<(&mut Velocity, &mut Position, &mut Distance), With<CubeBaby>>,
    display_properties: Res<DisplayProperties>,
) {
    let (mut velocity, mut position, mut distance) = query.into_inner();

    let minimum_position = display_properties.minimum_position().as_vec2();
    let maximum_position = display_properties.maximum_position().as_vec2();

    if position.x < minimum_position.x {
        position.x = minimum_position.x;
        velocity.x = velocity.x.abs();
    } else if position.x + WINDOW_SIZE > maximum_position.x {
        position.x = maximum_position.x - WINDOW_SIZE;
        velocity.x = -velocity.x.abs();
    }

    if position.y < minimum_position.y {
        position.y = minimum_position.y;
        velocity.y = velocity.y.abs();
    } else if position.y + WINDOW_SIZE > maximum_position.y {
        position.y = maximum_position.y - WINDOW_SIZE;
        velocity.y = -velocity.y.abs();
    }

    let start_position = position.0;

    position.0 += velocity.0 * time.delta_secs();
    velocity.0 *= (1.0 - (SLIDE_DRAG * SPRITE_SCALE * time.delta_secs())).clamp(0.0, 1.0);
    distance.0 += start_position.distance(position.0);

    window.position.set(position.round().as_ivec2());
}

/// Updates the sprite's atlas index to make the cube baby rotate as it moves.
pub fn update_sprite_rotation(query: Single<(&mut Sprite, &mut Distance), With<CubeBaby>>) {
    let (mut sprite, mut distance) = query.into_inner();

    if distance.0 >= SLIDE_SPIN_DISTANCE * SPRITE_SCALE {
        let texture_atlas = sprite.texture_atlas.as_mut().expect("missing texture atlas");

        texture_atlas.index = (texture_atlas.index + 1) % ATLAS_FRAMES as usize;

        distance.0 -= SLIDE_SPIN_DISTANCE * SPRITE_SCALE;
    }
}
