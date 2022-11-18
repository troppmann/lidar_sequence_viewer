use crate::math::smooth_damp;
use bevy::{input::mouse::MouseMotion, prelude::*, window::CursorGrabMode};

pub struct ObserverPlugin;

impl Plugin for ObserverPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ClearColor(Color::rgb_u8(0, 41, 61)))
            .add_startup_system(setup_camera)
            .add_system(camera_control);
    }
}

fn setup_camera(mut commands: Commands) {
    let transform = Transform::from_xyz(5.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y);
    let (_, yaw, pitch) = transform.rotation.to_euler(EulerRot::ZYX);
    commands
        .spawn(Camera3dBundle {
            transform,
            ..default()
        })
        .insert(CameraController {
            pitch: DampedFloat::init(pitch),
            yaw: DampedFloat::init(yaw),
            ..default()
        });
}

#[derive(Component)]
struct CameraController {
    pub enabled: bool,
    pub sensitivity: f32,
    pub key_forward: KeyCode,
    pub key_back: KeyCode,
    pub key_left: KeyCode,
    pub key_right: KeyCode,
    pub key_up: KeyCode,
    pub key_down: KeyCode,
    pub key_run: KeyCode,
    pub walk_speed: f32,
    pub run_speed: f32,
    pub friction: f32,
    pub pitch: DampedFloat,
    pub yaw: DampedFloat,
    pub smooth_time: f32,
    pub velocity: Vec3,
    pub old_cursor_position: Option<Vec2>,
}

impl Default for CameraController {
    fn default() -> Self {
        Self {
            enabled: true,
            sensitivity: 0.25,
            key_forward: KeyCode::W,
            key_back: KeyCode::S,
            key_left: KeyCode::A,
            key_right: KeyCode::D,
            key_up: KeyCode::E,
            key_down: KeyCode::Q,
            key_run: KeyCode::LShift,
            walk_speed: 10.0,
            run_speed: 30.0,
            friction: 0.5,
            pitch: DampedFloat {
                actual: 0.0,
                target: 0.0,
                velocity: 0.0,
            },
            yaw: DampedFloat {
                actual: 0.0,
                target: 0.0,
                velocity: 0.0,
            },
            smooth_time: 0.01,
            velocity: Vec3::ZERO,
            old_cursor_position: None,
        }
    }
}

struct DampedFloat {
    pub actual: f32,
    pub target: f32,
    pub velocity: f32,
}
impl DampedFloat {
    fn init(value: f32) -> Self {
        Self {
            actual: value,
            target: value,
            velocity: 0.0,
        }
    }
    fn damp_step(&mut self, smooth_time: f32, delta_time: f32) {
        self.actual = smooth_damp(
            self.actual,
            self.target,
            &mut self.velocity,
            smooth_time,
            delta_time,
        );
    }
}

fn camera_control(
    time: Res<Time>,
    mut windows: ResMut<Windows>,
    btn: Res<Input<MouseButton>>,
    mut mouse_events: EventReader<MouseMotion>,
    key_input: Res<Input<KeyCode>>,
    mut query: Query<(&mut Transform, &mut CameraController), With<Camera>>,
) {
    let delta_time = time.delta_seconds();

    // Handle mouse input
    let mut mouse_delta = Vec2::ZERO;
    for mouse_event in mouse_events.iter() {
        mouse_delta += mouse_event.delta;
    }

    for (mut transform, mut options) in query.iter_mut() {
        if !options.enabled {
            continue;
        }

        // Handle key input
        let mut axis_input = Vec3::ZERO;
        if key_input.pressed(options.key_forward) {
            axis_input.z += 1.0;
        }
        if key_input.pressed(options.key_back) {
            axis_input.z -= 1.0;
        }
        if key_input.pressed(options.key_right) {
            axis_input.x += 1.0;
        }
        if key_input.pressed(options.key_left) {
            axis_input.x -= 1.0;
        }
        if key_input.pressed(options.key_up) {
            axis_input.y += 1.0;
        }
        if key_input.pressed(options.key_down) {
            axis_input.y -= 1.0;
        }

        // Apply movement update
        if axis_input != Vec3::ZERO {
            let max_speed = if key_input.pressed(options.key_run) {
                options.run_speed
            } else {
                options.walk_speed
            };
            options.velocity = axis_input.normalize() * max_speed;
        } else {
            let friction = options.friction.clamp(0.0, 1.0);
            options.velocity *= 1.0 - friction;
            if options.velocity.length_squared() < 1e-6 {
                options.velocity = Vec3::ZERO;
            }
        }
        let forward = transform.forward();
        let right = transform.right();
        transform.translation += options.velocity.x * delta_time * right
            + options.velocity.y * delta_time * Vec3::Y
            + options.velocity.z * delta_time * forward;

        let Some(window )= windows.get_primary_mut() else {
            return;
        };
        // Apply look update
        if btn.just_pressed(MouseButton::Right) {
            window.set_cursor_grab_mode(CursorGrabMode::Confined);
            window.set_cursor_visibility(false);
            options.old_cursor_position = window.cursor_position();
        }

        if mouse_delta != Vec2::ZERO && btn.pressed(MouseButton::Right) {
            options.pitch.target = (options.pitch.target
                - mouse_delta.y * 0.5 * options.sensitivity * delta_time)
                .clamp(
                    -0.99 * std::f32::consts::FRAC_PI_2,
                    0.99 * std::f32::consts::FRAC_PI_2,
                );
            options.yaw.target -= mouse_delta.x * options.sensitivity * delta_time;
        }
        let smooth_time = options.smooth_time;
        options.pitch.damp_step(smooth_time, delta_time);
        options.yaw.damp_step(smooth_time, delta_time);
        transform.rotation =
            Quat::from_euler(EulerRot::ZYX, 0.0, options.yaw.actual, options.pitch.actual);

        if btn.just_released(MouseButton::Right) {
            window.set_cursor_grab_mode(CursorGrabMode::None);
            window.set_cursor_visibility(true);
            if let Some(pos) = options.old_cursor_position {
                window.set_cursor_position(pos);
            }
        }
    }
}