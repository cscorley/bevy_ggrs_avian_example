mod colliders;
mod frames;
mod log_plugin;
mod network;
mod physics;
mod random_movement;
mod rollback;
mod startup;

// A prelude to simplify other file imports
mod prelude {
    pub use crate::colliders::*;
    pub use crate::frames::*;
    pub use crate::log_plugin::LogSettings;
    pub use crate::network::*;
    pub use crate::physics::*;
    pub use crate::random_movement::*;
    pub use crate::rollback::*;
    pub use crate::startup::*;
    pub use avian2d::prelude::*;
    pub use bevy::log::*;
    pub use bevy::prelude::*;
    pub use bevy_framepace::{FramepacePlugin, FramepaceSettings, Limiter};
    pub use bevy_ggrs::ggrs::{Frame, InputStatus, PlayerType, SessionBuilder};
    pub use bevy_ggrs::prelude::*;
    pub use bevy_inspector_egui::quick::WorldInspectorPlugin;
    pub use bytemuck::{Pod, Zeroable};
    pub use rand::{thread_rng, Rng};

    pub const NUM_PLAYERS: usize = 2;
    pub const FPS: usize = 60;
    pub const MAX_PREDICTION: usize = 5;
    pub const INPUT_DELAY: usize = 3;

    // Having a "load screen" time helps with initial desync issues.  No idea why,
    // but this tests well. There is also sometimes a bug when a rollback to frame 0
    // occurs if two clients have high latency.  Having this in place at least for 1
    // frame helps prevent that :-)
    pub const LOAD_SECONDS: usize = 1;

    // TODO: Hey you!!! You, the one reading this!  Yes, you.

    // Buy gschup a coffee next time you get the chance.
    // https://ko-fi.com/gschup
    // They host this match making service for us to use FOR FREE.
    // It has been an incredibly useful thing I don't have to think about while working
    // and learning how to implement this stuff and I guarantee it will be for you too.
    // pub const MATCHBOX_ADDR: &str = "wss://match.gschup.dev/bevy-ggrs-avian-example?next=2";
    // Unfortunately, this matchbox is too out of date to work with the latest plugin.

    // So, use Johan's compatible matchbox.
    // Check out their work on "Cargo Space", especially the blog posts, which are incredibly enlightening!
    // https://johanhelsing.studio/cargospace
    pub const MATCHBOX_ADDR: &str = "wss://match-0-7.helsing.studio/bevy-ggrs-avian-example?next=2";
    // Care to run your own matchbox?  Great!
    // pub const MATCHBOX_ADDR: &str = "ws://localhost:3536/bevy-ggrs-avian-example?next=2";
    // TODO: Maybe update this room name (bevy-ggrs-avian-example) so we don't test with each other :-)
}

use bevy::ecs::schedule::ScheduleBuildSettings;
use bevy_ggrs::{GgrsApp, GgrsPlugin};

use crate::prelude::*;

fn main() {
    let mut app = App::new();

    // Something smaller so we can put these side by side
    let window_info = Window {
        title: "Example".into(),
        resolution: (800.0, 600.0).into(),
        ..default()
    };

    // DefaultPlugins will use window descriptor
    app.insert_resource(ClearColor(Color::BLACK))
        .insert_resource(LogSettings {
            level: Level::INFO,
            ..default()
        })
        .add_plugins(
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(window_info),
                    ..default()
                })
                .build()
                .disable::<LogPlugin>(),
        )
        // Add our own log plugin to help with comparing desync output
        .add_plugins(log_plugin::LogPlugin)
        .add_systems(Startup, startup)
        .add_systems(Startup, connect)
        .add_systems(Update, toggle_random_input)
        .add_systems(Update, close_on_esc)
        .add_systems(Update, update_matchbox_socket)
        .add_systems(Update, handle_p2p_events);

    app.add_plugins(GgrsPlugin::<ExampleGgrsConfig>::default())
        .set_rollback_schedule_fps(FPS)
        .add_systems(bevy_ggrs::ReadInputs, input)
        // We must add a specific checksum check for everything we want to include in desync detection.
        // You are welcome to checksum more than this, but I feel just checking the Avian Position is enough.
        .checksum_component::<Position>(|t| {
            let mut bytes: Vec<u8> = Vec::new();
            bytes.extend(t.x.to_ne_bytes());
            bytes.extend(t.y.to_ne_bytes());
            fletcher16(&bytes) as u64
        })
        // https://github.com/Jondolf/avian/issues/478
        //.rollback_component_with_copy::<GlobalTransform>()
        //.rollback_component_with_copy::<Transform>()
        .rollback_component_with_copy::<LinearVelocity>()
        .rollback_component_with_copy::<AngularVelocity>()
        // automatic
        //.rollback_component_with_clone::<Collider>()
        .rollback_component_with_clone::<CollidingEntities>()
        //.rollback_component_with_copy::<AccumulatedTranslation>()
        //.rollback_component_with_copy::<CenterOfMass>()
        //.rollback_component_with_copy::<ColliderAabb>()
        //.rollback_component_with_copy::<ColliderDensity>()
        //.rollback_component_with_copy::<ColliderMarker>()
        //.rollback_component_with_copy::<ColliderMassProperties>()
        //.rollback_component_with_copy::<ColliderParent>()
        //.rollback_component_with_copy::<ColliderTransform>()
        .rollback_component_with_copy::<ExternalAngularImpulse>()
        .rollback_component_with_copy::<ExternalForce>()
        .rollback_component_with_copy::<ExternalImpulse>()
        .rollback_component_with_copy::<ExternalTorque>()
        //.rollback_component_with_copy::<Friction>()
        //.rollback_component_with_copy::<Inertia>()
        //.rollback_component_with_copy::<InverseInertia>()
        //.rollback_component_with_copy::<InverseMass>()
        //.rollback_component_with_copy::<LockedAxes>()
        //.rollback_component_with_copy::<Mass>()
        .rollback_component_with_copy::<Position>()
        //.rollback_component_with_copy::<Restitution>()
        //.rollback_component_with_copy::<RigidBody>()
        .rollback_component_with_copy::<Rotation>()
        .rollback_component_with_copy::<Sleeping>()
        .rollback_component_with_copy::<SleepingDisabled>()
        .rollback_component_with_copy::<TimeSleeping>()
        //.rollback_component_with_copy::<avian2d::position::PreSolveAccumulatedTranslation>()
        //.rollback_component_with_copy::<avian2d::position::PreviousRotation>()
        //.rollback_component_with_copy::<avian2d::sync::PreviousGlobalTransform>()
        //.rollback_component_with_copy::<PreSolveAngularVelocity>() // pub(crate)
        //.rollback_component_with_copy::<PreSolveLinearVelocity>() // pub(crate)
        //.rollback_component_with_copy::<PreviousColliderTransform>() // pub(crate)
        // TODO: not sure if rolling these back are necessary
        //.rollback_resource_with_copy::<Time<()>>()
        //.rollback_resource_with_copy::<Time<Fixed>>()
        .rollback_resource_with_copy::<Time<Physics>>() // GgrsTime is cloned
        //.rollback_resource_with_copy::<Time<Real>>()
        .rollback_resource_with_copy::<Time<Substeps>>()
        //.rollback_resource_with_copy::<Time<Virtual>>()
        //.rollback_component_with_copy::<SphericalJoint>() // 3d
        //.rollback_component_with_clone::<ColliderConstructor>()
        //.rollback_component_with_clone::<ColliderConstructorHierarchy>()
        //.rollback_component_with_clone::<RayCaster>()
        //.rollback_component_with_clone::<RayHits>()
        //.rollback_component_with_clone::<Sensor>()
        //.rollback_component_with_clone::<ShapeCaster>()
        //.rollback_component_with_clone::<ShapeHits>()
        .rollback_component_with_clone::<broad_phase::AabbIntersections>()
        //.rollback_component_with_copy::<AngularDamping>()
        //.rollback_component_with_copy::<CollisionLayers>()
        //.rollback_component_with_copy::<CollisionMargin>()
        //.rollback_component_with_copy::<DebugRender>()
        //.rollback_component_with_copy::<DistanceJoint>()
        //.rollback_component_with_copy::<Dominance>()
        //.rollback_component_with_copy::<FixedJoint>()
        //.rollback_component_with_copy::<GravityScale>()
        //.rollback_component_with_copy::<LinearDamping>()
        //.rollback_component_with_copy::<PrismaticJoint>()
        //.rollback_component_with_copy::<RevoluteJoint>()
        //.rollback_component_with_copy::<SpeculativeMargin>()
        //.rollback_component_with_copy::<SweptCcd>()
        //.rollback_component_with_copy::<avian2d::sync::ancestor_marker::AncestorMarker<ColliderMarker>>()
        //.rollback_component_with_copy::<avian2d::sync::ancestor_marker::AncestorMarker<RigidBody>>()
        //.rollback_resource_with_clone::<NarrowPhaseConfig>()
        //.rollback_resource_with_clone::<avian2d::sync::SyncConfig>()
        //.rollback_resource_with_clone::<dynamics::solver::SolverConfig>()
        .rollback_resource_with_clone::<Collisions>()
        //.rollback_resource_with_copy::<DeactivationTime>()
        //.rollback_resource_with_copy::<SleepingThreshold>()
        //.rollback_resource_with_copy::<SubstepCount>()
        //.rollback_resource_with_reflect::<BroadCollisionPairs>()
        //.rollback_resource_with_reflect::<Gravity>()
        // Game stuff
        .rollback_resource_with_reflect::<EnablePhysicsAfter>();

    // We need to add a bunch of systems into the GGRSSchedule.
    // Remove ambiguity detection, avian is in conflict with the GGRS default
    app.get_schedule_mut(bevy_ggrs::GgrsSchedule)
        .unwrap() // We just configured the plugin -- this is probably fine
        .set_build_settings(ScheduleBuildSettings::default());

    app.add_plugins(PhysicsPlugins::new(bevy_ggrs::GgrsSchedule));
    app.add_systems(
        bevy_ggrs::GgrsSchedule,
        (
            copy_time,
            log_start_frame,
            update_current_session_frame,
            log_confirmed_frame,
            // the three above must actually come before we update rollback status
            update_rollback_status,
            // these three must actually come after we update rollback status
            force_update_rollbackables,
            toggle_physics,
            apply_inputs,
            apply_deferred,
        )
            .chain()
            .before(PhysicsSet::Prepare),
    );
    app.add_systems(
        bevy_ggrs::GgrsSchedule,
        (
            pause_physics_test,
            log_end_frame,
            apply_deferred, // Flushing again
        )
            .chain()
            .after(PhysicsSet::Sync),
    );

    // We don't really draw anything ourselves, just show us the raw physics colliders
    app.add_plugins(PhysicsDebugPlugin::default())
        .insert_gizmo_config(PhysicsGizmos::default(), GizmoConfig::default());

    app.add_plugins(WorldInspectorPlugin::new());

    // I have found that since GGRS is limiting the movement FPS anyway,
    // there isn't much of a point in rendering more frames than necessary.
    // One thing I've yet to prove out is if this is actually detrimental or
    // not to resimulation, since we're basically taking up time that GGRS
    // would use already to pace itself.
    // You may find this useless, or bad.  Submit a PR if it is!
    /*
    app.add_plugins(FramepacePlugin)
        .insert_resource(FramepaceSettings {
            limiter: Limiter::from_framerate(FPS as f64),
        });
    */
    app.run();
}

pub fn close_on_esc(
    mut commands: Commands,
    focused_windows: Query<(Entity, &Window)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    for (window, focus) in focused_windows.iter() {
        if !focus.focused {
            continue;
        }

        if input.just_pressed(KeyCode::Escape) {
            commands.entity(window).despawn();
        }
    }
}

pub fn fletcher16(data: &[u8]) -> u16 {
    let mut sum1: u16 = 0;
    let mut sum2: u16 = 0;

    for byte in data {
        sum1 = (sum1 + *byte as u16) % 255;
        sum2 = (sum2 + sum1) % 255;
    }

    (sum2 << 8) | sum1
}

pub fn copy_time(ggrs_time: Res<Time<GgrsTime>>, mut physics_time: ResMut<Time<Physics>>) {
    /*
       *physics_time = Time::new_with(Physics::default());
       physics_time.advance_to(ggrs_time.elapsed() - ggrs_time.delta());
       physics_time.advance_by(ggrs_time.delta());
    */

    log::info!("ggrs time {:?}", ggrs_time);
    log::info!("phys time {:?}", physics_time);
}

pub fn force_update_rollbackables(
    mut av_query: Query<&mut AngularVelocity, With<Rollback>>,
    mut lv_query: Query<&mut LinearVelocity, With<Rollback>>,
    mut p_query: Query<&mut Position, With<Rollback>>,
    mut r_query: Query<&mut Rotation, With<Rollback>>,
    mut ce_query: Query<&mut CollidingEntities, With<Rollback>>,
) {
    for mut c in av_query.iter_mut() {
        c.set_changed();
    }
    for mut c in lv_query.iter_mut() {
        c.set_changed();
    }
    for mut c in p_query.iter_mut() {
        c.set_changed();
    }
    for mut c in r_query.iter_mut() {
        c.set_changed();
    }
    for mut c in ce_query.iter_mut() {
        c.set_changed();
    }
}
