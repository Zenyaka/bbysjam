use std::fs::File;
use ron::de::from_reader;
use serde::Deserialize;


use bevy::{
    log::LogSettings,
    prelude::*,
    window::{CreateWindow, PresentMode, WindowId, WindowSettings},
};
use bevy::winit::WinitWindows;
use bevy_inspector_egui::WorldInspectorPlugin;

#[derive(Debug, Deserialize, Copy, Clone)]
pub struct GameConfig {
    pub editor_mode: bool,
}

impl GameConfig {
    fn new() -> GameConfig {
        let config_path = format!("{}/assets/config/game_config.ron", env!("CARGO_MANIFEST_DIR"));
        let f = File::open(&config_path).unwrap();
        let config: GameConfig = match from_reader(f) {
            Ok(x) => x,
            Err(e) => {
                error!("Failed to load config: {}", e);

                panic!()
            }
        };


        config
    }
}

fn main() {
    let mut app = App::new();

    // have to use println here because bevy hasn't yet initialized logging
    // on wasm this will disappear into void
    println!("Starting...");

    let config = GameConfig::new();

    app.insert_resource(config);
    app.insert_resource(WindowSettings {
        add_primary_window: false,
        ..default()
    });

    #[cfg(debug_assertions)]
    app.insert_resource(LogSettings {
        filter: "info,bevy_render=warn,wgpu_core=warn,wgpu_hal=error,bbysjam=trace".into(),
        level: bevy::log::Level::DEBUG,
    });

    #[cfg(not(debug_assertions))]
    app.insert_resource(LogSettings {
        filter: "warn".into(),
        level: bevy::log::Level::WARN,
    });

    app.insert_resource(ClearColor(Color::rgba(0.3, 0.3, 0.3, 1.0)));
    app.add_plugins(DefaultPlugins);
    app.add_plugin(TransparentWindowPlugin);
    if config.editor_mode {
        app.add_plugin(WorldInspectorPlugin::new());
    } else {
        app.add_startup_system(resize_window);
    }
    app.add_startup_system(setup);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(20.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn resize_window(mut windows: ResMut<Windows>, winits: NonSend<WinitWindows>) {
    trace!("Setting window res and position");
    let window = winits.get_window(WindowId::primary()).expect("Primary winit window doesn't exist");
    let monitor = window.current_monitor().unwrap();
    let monitor_size = monitor.size();
    let bevy_win = windows.primary_mut();
    //make a stripe across bottom of the monitor, up 40 to account for taskbar whatever, and monitor wide
    bevy_win.set_resolution(monitor_size.width as f32, 300.);
    bevy_win.set_position(IVec2::new(0, (monitor_size.height - 340) as i32));
}

struct TransparentWindowPlugin;

impl Plugin for TransparentWindowPlugin {
    fn build(&self, app: &mut App) {
        let window_descriptor = WindowDescriptor {
            transparent: true,
            decorations: false,
            position: WindowPosition::At(Vec2::new(0., 0.)),
            width: 1920.,
            height: 1080.,
            present_mode: PresentMode::Immediate,
            ..default()
        };

        let mut create_window_event = app.world.resource_mut::<Events<CreateWindow>>();

        create_window_event.send(CreateWindow {
            id: WindowId::primary(),
            descriptor: window_descriptor,
        });
    }
}

