use bevy::{
    log::LogSettings,
    prelude::*,
    window::{CreateWindow, PresentMode, WindowId, WindowSettings},
};
use bevy::math::ivec2;
use bevy::winit::WinitWindows;

fn main() {
    let mut app = App::new();

    info!("Starting...");

    app.insert_resource(WindowSettings {
        add_primary_window: false,
        ..default()
    });

    #[cfg(debug_assertions)]
    app.insert_resource(LogSettings {
        filter: "info,wgpu_core=warn,wgpu_hal=warn,bbysjam=trace".into(),
        level: bevy::log::Level::DEBUG,
    });

    #[cfg(not(debug_assertions))]
    app.insert_resource(LogSettings {
        filter: "warn".into(),
        level: bevy::log::Level::WARN,
    });

    app.insert_resource(ClearColor(Color::rgba(0.0, 1.0, 0.0, 0.50)));
    app.add_plugins(DefaultPlugins);
    app.add_plugin(TransparentWindowPlugin);
    app.add_startup_system(init_window);
    app.add_startup_system(setup);

    app.run();
}

fn setup(mut commands: Commands) {
    commands.spawn_bundle(Camera3dBundle {
        transform: Transform::from_xyz(20.0, 20.0, 20.0).looking_at(Vec3::ZERO, Vec3::Y),
        ..default()
    });
}

fn init_window(mut windows: ResMut<Windows>, winits: NonSend<WinitWindows>) {
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
            width: 800.,
            height: 300.,
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

