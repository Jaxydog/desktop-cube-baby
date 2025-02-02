use std::fmt::Debug;
use std::hash::Hash;
use std::marker::PhantomData;

use bevy::image::ImageSampler;
use bevy::prelude::*;
use bevy::state::state::FreelyMutableState;
use bevy::window::{CompositeAlphaMode, Monitor, PresentMode, PrimaryWindow, WindowLevel, WindowResolution};
use bevy::winit::WinitWindows;

const SPRITE_SIDE_LEN: u32 = 32;
const WINDOW_SIDE_LEN: f32 = 256.0;
const SCALE: f32 = WINDOW_SIDE_LEN / SPRITE_SIDE_LEN as f32;
const SPIN_DISTANCE: f64 = 10.0 * SCALE as f64;

/// A generic loading state.
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq, States)]
enum GenericLoadingState {
    /// The value is still loading.
    #[default]
    Loading,
    /// The value has been loaded.
    Loaded,
}

/// A typed loading state.
#[derive(Debug)]
struct LoadingState<T> {
    /// The actual loading state.
    state: GenericLoadingState,
    /// The associated type marker.
    _type: PhantomData<T>,
}

impl<T> LoadingState<T> {
    /// Creates a new loading state set to loading.
    const fn loading() -> Self {
        Self { state: GenericLoadingState::Loading, _type: PhantomData }
    }

    /// Creates a new loading state set to loaded.
    const fn loaded() -> Self {
        Self { state: GenericLoadingState::Loaded, _type: PhantomData }
    }
}

impl<T> Copy for LoadingState<T> {}

impl<T> Clone for LoadingState<T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T> Default for LoadingState<T> {
    fn default() -> Self {
        Self { state: GenericLoadingState::default(), _type: PhantomData }
    }
}

impl<T> Hash for LoadingState<T> {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.state.hash(state);
    }
}

impl<T> Eq for LoadingState<T> {}

impl<T> PartialEq for LoadingState<T> {
    fn eq(&self, other: &Self) -> bool {
        self.state.eq(&other.state)
    }
}

impl<T: Debug + Send + Sync + 'static> States for LoadingState<T> {}

impl<T: Debug + Send + Sync + 'static> FreelyMutableState for LoadingState<T> {}

/// Stores the baby's state.
#[derive(Clone, Debug, Component)]
struct Baby {
    /// A handle to the baby's texture.
    texture_handle: Handle<Image>,
    /// The baby's screen boundaries.
    boundaries: (Vec2, Vec2),
    /// The baby's current position.
    position: Vec2,
    /// The baby's current velocity.
    velocity: Vec2,
    /// Tracks the time delay between baby hits.
    hit_timer: f64,
    /// Tracks the distance since the last rotation.
    spin_distance: f64,
}

impl Baby {
    /// Creates a new [`Baby`].
    const fn new(texture_handle: Handle<Image>) -> Self {
        Self {
            texture_handle,
            boundaries: (Vec2::ZERO, Vec2::ZERO),
            position: Vec2::ZERO,
            velocity: Vec2::ZERO,
            hit_timer: 0.05,
            spin_distance: 0.0,
        }
    }

    fn window_position(&self) -> IVec2 {
        self.position.round().as_ivec2()
    }
}

fn main() {
    let window = Window {
        resolution: WindowResolution::new(WINDOW_SIDE_LEN, WINDOW_SIDE_LEN),
        position: WindowPosition::new(IVec2::new(WINDOW_SIDE_LEN as i32, WINDOW_SIDE_LEN as i32)),
        present_mode: PresentMode::AutoVsync,
        title: env!("CARGO_BIN_NAME").to_string(),
        #[cfg(target_os = "macos")]
        composite_alpha_mode: CompositeAlphaMode::PostMultiplied,
        #[cfg(target_os = "linux")]
        composite_alpha_mode: CompositeAlphaMode::PreMultiplied,
        resizable: false,
        decorations: false,
        transparent: true,
        window_level: WindowLevel::AlwaysOnBottom,
        visible: cfg!(debug_assertions),
        movable_by_window_background: false,
        has_shadow: false,
        titlebar_shown: false,
        ..Window::default()
    };

    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin { primary_window: Some(window), ..WindowPlugin::default() }))
        .insert_resource(ClearColor(Color::NONE))
        .init_state::<LoadingState<Image>>()
        .init_state::<LoadingState<Monitor>>()
        .add_systems(Startup, initialize)
        .add_systems(OnEnter(LoadingState::<Monitor>::loaded()), render_window)
        .add_systems(
            Update,
            (
                load_texture.run_if(in_state(LoadingState::<Image>::loading())),
                load_monitor.run_if(in_state(LoadingState::<Monitor>::loading())),
                update_position
                    .run_if(in_state(LoadingState::<Image>::loaded()))
                    .run_if(in_state(LoadingState::<Monitor>::loaded())),
                handle_inputs
                    .run_if(in_state(LoadingState::<Image>::loaded()))
                    .run_if(in_state(LoadingState::<Monitor>::loaded())),
            ),
        )
        .run();
}

fn initialize(mut commands: Commands, assets: Res<AssetServer>, mut layouts: ResMut<Assets<TextureAtlasLayout>>) {
    commands.spawn(Camera2d);

    let handle = assets.load("cube_baby.png");
    let layout = layouts.add(TextureAtlasLayout::from_grid(UVec2::splat(SPRITE_SIDE_LEN), 8, 1, None, None));
    let sprite = Sprite {
        image: handle.clone_weak(),
        texture_atlas: Some(TextureAtlas { layout, index: 0 }),
        ..Sprite::default()
    };

    commands.spawn((Baby::new(handle), sprite, Transform::from_scale(Vec3::splat(SCALE))));
}

fn load_texture(
    mut state: ResMut<NextState<LoadingState<Image>>>,
    baby: Single<&Baby>,
    assets: Res<AssetServer>,
    mut images: ResMut<Assets<Image>>,
) {
    if assets.is_loaded(&baby.texture_handle) {
        let image = images.get_mut(&baby.texture_handle).expect("the texture was not actually loaded");

        image.sampler = ImageSampler::nearest();

        state.set(LoadingState::loaded());
    }
}

fn load_monitor(
    mut state: ResMut<NextState<LoadingState<Monitor>>>,
    mut baby: Single<&mut Baby>,
    window: Single<Entity, With<PrimaryWindow>>,
    windows: NonSend<WinitWindows>,
) {
    let Some(monitor) = windows.get_window(*window).and_then(|v| v.current_monitor()) else { return };

    let position = monitor.position();
    let resolution = monitor.size();

    let start_boundary = Vec2::new(position.x as f32, position.y as f32);
    let resolution = Vec2::new(resolution.width as f32, resolution.height as f32);
    let end_boundary = start_boundary + resolution;

    baby.boundaries = (start_boundary, end_boundary);
    baby.position = start_boundary + (resolution / 2.0);

    state.set(LoadingState::loaded());
}

fn render_window(baby: Single<&Baby>, mut window: Single<&mut Window>) {
    window.position.set(baby.window_position());

    window.visible = true;
}

fn handle_inputs(time: Res<Time>, mut baby: Single<&mut Baby>, mut input: EventReader<CursorMoved>) {
    if baby.hit_timer > 0.0 {
        baby.hit_timer -= time.delta_secs_f64();

        return;
    }

    let events: Box<_> = input.read().map(|v| v.position).collect();

    if events.len() >= 2 {
        let delta = events[events.len() - 1] - events[0];
        let mut delta = delta * 16.0 * SCALE;

        if delta.length() <= SCALE {
            delta = delta.normalize_or_zero() * SCALE;
        }

        baby.velocity += delta;
        baby.hit_timer = 0.5;
    }
}

fn update_position(time: Res<Time>, query: Single<(&mut Baby, &mut Sprite)>, mut window: Single<&mut Window>) {
    let (mut baby, mut sprite) = query.into_inner();

    if baby.position.x < baby.boundaries.0.x {
        baby.position.x = baby.boundaries.0.x;
        baby.velocity.x = -baby.velocity.x;
    } else if baby.position.x + WINDOW_SIDE_LEN > baby.boundaries.1.x {
        baby.position.x = baby.boundaries.1.x - WINDOW_SIDE_LEN;
        baby.velocity.x = -baby.velocity.x;
    }

    if baby.position.y < baby.boundaries.0.y {
        baby.position.y = baby.boundaries.0.y;
        baby.velocity.y = -baby.velocity.y;
    } else if baby.position.y + WINDOW_SIDE_LEN > baby.boundaries.1.y {
        baby.position.y = baby.boundaries.1.y - WINDOW_SIDE_LEN;
        baby.velocity.y = -baby.velocity.y;
    }

    let scaled_velocity = baby.velocity * time.delta_secs();
    let last_position = baby.position;

    baby.position += scaled_velocity;
    baby.velocity *= (1.0 - time.delta_secs()).clamp(0.0, 1.0);
    baby.spin_distance += last_position.distance(baby.position) as f64;

    if baby.spin_distance >= SPIN_DISTANCE {
        baby.spin_distance -= SPIN_DISTANCE;

        let atlas = sprite.texture_atlas.as_mut().expect("missing texture atlas");

        atlas.index += 1;
        atlas.index %= 8;
    }

    window.position.set(baby.window_position());
}
