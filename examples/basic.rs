  // (-2., -2.),
  // (2., -2.),
  // (-3., 1.),
  // (3., 1.),
  // (0., 3.),
  use bevy::{
      camera_controller::free_camera::{FreeCamera, FreeCameraPlugin},
      prelude::*
  };
  use bevy_random_path::RandomPath;

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
          Transform::from_xyz(6., 8., 4.).looking_at(Vec3::ZERO , Vec3::Y),
      ));

      let rpath = RandomPath::new(10, vec3(10., 0., 5.)).generate();

      cmd.spawn((
          Mesh3d(meshes.add(Polyline3d::new(rpath))),
          MeshMaterial3d(materials.add(Color::WHITE))
      ));
  }
