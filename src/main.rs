use bevy::prelude::*;
use bevy::render::{mesh::Indices, render_resource::PrimitiveTopology};

pub mod aoc;

#[derive(Debug, Copy, Clone, Component)]
pub struct Ray {
    pub p: Vec3,
    pub v: Vec3,
}

#[derive(Resource)]
pub struct Rays(pub Vec<Ray>, pub f32);

fn main() {
    let mut rays = aoc::INPUT
        .lines()
        .map(aoc::ints_n)
        .map(|[a, b, c, d, e, f]| Ray {
            p: Vec3 { x: a, y: b, z: c },
            v: Vec3 { x: d, y: e, z: f },
        })
        .collect::<Vec<_>>();

    let scale = rays
        .iter()
        .flat_map(|r| [r.p.x, r.p.y, r.p.z])
        .min_by(f32::total_cmp)
        .unwrap();

    for r in &mut rays {
        r.p /= scale;
        r.v *= 0.1;
    }

    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(bevy_flycam::PlayerPlugin)
        .insert_resource(Rays(rays, scale))
        .add_systems(Startup, setup)
        .add_systems(Update, system)
        .run();
}
fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut ambient: ResMut<AmbientLight>,
    rays: Res<Rays>,
) {
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(shape::Plane::from_size(5.0))),
        material: materials.add(Color::rgb(0.3, 0.5, 0.3).into()),
        ..default()
    });

    let material = materials.add(StandardMaterial::default());

    ambient.color = Color::WHITE;
    ambient.brightness = 0.75;

    for &ray in &rays.0 {
        let bundle = PbrBundle {
            mesh: meshes.add(shape::Cube::new(0.2).into()),
            material: material.clone(),
            transform: Transform::from_translation(ray.p),
            ..default()
        };
        commands.spawn((bundle, ray));
    }

    let bounds = [200000000000000f32, 400000000000000f32];
    for x in bounds {
        for y in bounds {
            for z in bounds {
                let xyz = Vec3::new(x, y, z) / rays.1;
                commands.spawn(PbrBundle {
                    mesh: meshes.add(shape::Cube::new(1.0).into()),
                    material: material.clone(),
                    transform: Transform::from_translation(xyz),
                    ..default()
                });
            }
        }
    }

    commands.spawn(PbrBundle {
        mesh: meshes.add(line_list(&rays.0[..])),
        material,
        ..default()
    });
}

fn system(
    mut gizmos: Gizmos,
    time: Res<Time>,
    rays: Res<Rays>,
    mut query: Query<(&mut Transform, &Ray)>,
) {
    let avg = rays.0.iter().map(|r| r.p).sum::<Vec3>() / rays.0.len() as f32;
    gizmos.sphere(avg, Quat::IDENTITY, 1.0, Color::ORANGE);

    let dt = 2.;
    let tx = 10.;
    let t = (time.elapsed_seconds() / tx % dt) / dt;

    for (mut transform, ray) in query.iter_mut() {
        transform.translation = ray.p + t * ray.v;
    }
}

pub fn triangle(a: Vec3, b: Vec3, c: Vec3) -> Mesh {
    Mesh::new(PrimitiveTopology::TriangleList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, vec![a, b, c])
        .with_inserted_attribute(
            Mesh::ATTRIBUTE_COLOR,
            [Color::RED, Color::GREEN, Color::BLUE]
                .map(|c| c.as_rgba_f32())
                .to_vec(),
        )
        .with_indices(Some(Indices::U16(vec![0, 1, 2, 2, 1, 0])))
        .with_duplicated_vertices()
        .with_computed_flat_normals()
}

pub fn line_list(rays: &[Ray]) -> Mesh {
    let positions = rays
        .iter()
        .flat_map(|r| [r.p, r.p + r.v])
        .collect::<Vec<_>>();

    let colors = [Color::RED, Color::BLUE]
        .into_iter()
        .cycle()
        .take(positions.len())
        .map(|c| c.as_rgba_f32())
        .collect::<Vec<_>>();

    Mesh::new(PrimitiveTopology::LineList)
        .with_inserted_attribute(Mesh::ATTRIBUTE_POSITION, positions)
        .with_inserted_attribute(Mesh::ATTRIBUTE_COLOR, colors)
}
