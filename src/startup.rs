use crate::prelude::*;

pub fn startup(mut commands: Commands) {
    // frame updating
    commands.insert_resource(CurrentSessionFrame::default());
    commands.insert_resource(RollbackStatus::default());

    // physics toggling
    commands.insert_resource(EnablePhysicsAfter::default());

    // random movement for testing
    commands.insert_resource(RandomInput { on: true });

    commands.spawn(Camera2dBundle::default());

    commands
        .spawn_empty()
        .insert(Name::new("Ball"))
        .insert(DynamicColliderBundle {
            collider: Collider::circle(4.),
            restitution: Restitution::new(2.0),
            //ccd: Ccd::enabled(),
            ..default()
        })
        .insert(TransformBundle {
            local: Transform::from_xyz(0., 10., 0.),
            ..default()
        })
        .add_rollback();

    commands
        .spawn_empty()
        .insert(Name::new("Player 1"))
        .insert(Player { handle: 0 })
        .insert(DynamicColliderBundle {
            collider: Collider::rectangle(16., 16.),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            ..default()
        })
        .insert(TransformBundle {
            local: Transform::from_xyz(-10., -50., 0.),
            ..default()
        })
        .add_rollback();

    commands
        .spawn_empty()
        .insert(Name::new("Player 2"))
        .insert(Player { handle: 1 })
        .insert(DynamicColliderBundle {
            collider: Collider::rectangle(16., 16.),
            locked_axes: LockedAxes::ROTATION_LOCKED,
            ..default()
        })
        .insert(TransformBundle {
            local: Transform::from_xyz(10., -50., 0.),
            ..default()
        })
        .add_rollback();

    let thickness = 10.0;
    let box_length = 200.0;
    let overlapping_box_length = box_length + thickness;

    commands
        .spawn_empty()
        .insert(Name::new("Floor"))
        .insert(FixedColliderBundle {
            collider: Collider::rectangle(overlapping_box_length, thickness),
            ..default()
        })
        .insert(TransformBundle {
            local: Transform::from_xyz(0., -box_length / 2., 0.),
            ..default()
        });

    commands
        .spawn_empty()
        .insert(Name::new("Left Wall"))
        .insert(FixedColliderBundle {
            collider: Collider::rectangle(thickness, overlapping_box_length),
            ..default()
        })
        .insert(TransformBundle {
            local: Transform::from_xyz(-box_length / 2., 0., 0.),
            ..default()
        });

    commands
        .spawn_empty()
        .insert(Name::new("Right Wall"))
        .insert(FixedColliderBundle {
            collider: Collider::rectangle(thickness, overlapping_box_length),
            ..default()
        })
        .insert(TransformBundle {
            local: Transform::from_xyz(box_length / 2., 0., 0.),
            ..default()
        });

    commands
        .spawn_empty()
        .insert(Name::new("Ceiling"))
        .insert(FixedColliderBundle {
            collider: Collider::rectangle(overlapping_box_length, thickness),
            ..default()
        })
        .insert(TransformBundle {
            local: Transform::from_xyz(0., box_length / 2., 0.),
            ..default()
        });

    let corner_position = box_length / 2.;
    commands
        .spawn_empty()
        .insert(Name::new("Southeast Corner"))
        .insert(FixedColliderBundle {
            collider: Collider::triangle(
                Vec2::new(0., 0.),
                Vec2::new(-thickness * 2., 0.),
                Vec2::new(0., thickness * 2.),
            ),
            ..default()
        })
        .insert(TransformBundle {
            local: Transform::from_xyz(corner_position, -corner_position, 0.),
            ..default()
        });

    commands
        .spawn_empty()
        .insert(Name::new("Southwest Corner"))
        .insert(FixedColliderBundle {
            collider: Collider::triangle(
                Vec2::new(0., 0.),
                Vec2::new(thickness * 2., 0.),
                Vec2::new(0., thickness * 2.),
            ),
            ..default()
        })
        .insert(TransformBundle {
            local: Transform::from_xyz(-corner_position, -corner_position, 0.),
            ..default()
        });

    commands
        .spawn_empty()
        .insert(Name::new("Northeast Corner"))
        .insert(FixedColliderBundle {
            collider: Collider::triangle(
                Vec2::new(0., 0.),
                Vec2::new(-thickness * 2., 0.),
                Vec2::new(0., -thickness * 2.),
            ),
            ..default()
        })
        .insert(TransformBundle {
            local: Transform::from_xyz(corner_position, corner_position, 0.),
            ..default()
        });

    commands
        .spawn_empty()
        .insert(Name::new("Northwest Corner"))
        .insert(FixedColliderBundle {
            collider: Collider::triangle(
                Vec2::new(0., 0.),
                Vec2::new(thickness * 2., 0.),
                Vec2::new(0., -thickness * 2.),
            ),
            ..default()
        })
        .insert(TransformBundle {
            local: Transform::from_xyz(-corner_position, corner_position, 0.),
            ..default()
        });
}
