
use bevy::{
    camera_controller::free_camera::{FreeCamera, FreeCameraPlugin}, prelude::*
};
use bevy_random_loop::RandomLoop;

fn main () {
    App::new()
        .insert_resource(ClearColor(Color::BLACK))
        .add_plugins((
            DefaultPlugins,
            FreeCameraPlugin,
        ))
        .add_systems(Startup, startup)
        .run();
}

// ---

fn startup(
    mut cmd: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    cmd.spawn((
        Camera3d::default(),
        Camera::default(),
        FreeCamera::default(),
        Transform::from_xyz(0., -300., 0.1).looking_at(Vec3::ZERO , Vec3::Y),
    ));

    // Convex Hull

    // let mut rpath = vec![
    //     vec3(-92.24923, 0.0, -97.42763), vec3(74.314735, -0.0, -76.35796), vec3(79.84702, 0.0, -32.513664), vec3(-7.4177504, 0.0, 95.739075),
    //     vec3(-25.661873, -0.0, 97.137), vec3(-59.890247, -0.0, 94.272064), vec3(-97.59946, 0.0, 49.56999), vec3(-97.69504, -0.0, 49.337864),
    //     vec3(-93.61068, 0.0, -60.736256)
    // ];

    let mut rpath = RandomLoop::generate(12, vec3(100., 0., 100.));
    cmd.spawn((
        Mesh3d(meshes.add(Polyline3d::new(rpath.clone()))),
        MeshMaterial3d(materials.add(StandardMaterial{
            emissive: LinearRgba::rgb(10., 0., 0.),
            ..default()
        }))
    ));


    // Let's fluff this up a bit

    RandomLoop::vary(&mut rpath, 30.);
    cmd.spawn((
        Mesh3d(meshes.add(Polyline3d::new(rpath.clone()))),
        MeshMaterial3d(materials.add(StandardMaterial{
            emissive: LinearRgba::rgb(10., 10., 0.),
            ..default()
        }))
    ));

    //And smooth out too sharp corners and too short segments

    RandomLoop::smooth_out(&mut rpath, 120f32.to_radians(), 30.);
    cmd.spawn((
        Mesh3d(meshes.add(Polyline3d::new(rpath.clone()))),
        MeshMaterial3d(materials.add(StandardMaterial{
            emissive: LinearRgba::rgb(0., 10., 0.),
            ..default()
        }))
    ));

    let cr = CubicBSpline::new(rpath).to_curve_cyclic().unwrap();
    let spline = cr.iter_positions(120).collect::<Vec<_>>();
    cmd.spawn((
        Mesh3d(meshes.add(Polyline3d::new(spline))),
        MeshMaterial3d(materials.add(StandardMaterial{
            emissive: LinearRgba::rgb(0., 0., 10.),
            ..default()
        }))
    ));
}
