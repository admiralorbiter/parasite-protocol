use bevy::prelude::*;
use bevy::sprite::MaterialMesh2dBundle;

pub struct ArenaMapPlugin;

impl Plugin for ArenaMapPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CellRadius>()
            .add_systems(Startup, setup_arena);
    }
}

#[derive(Resource)]
pub struct CellRadius {
    pub radius: f32,
}

impl Default for CellRadius {
    fn default() -> Self {
        Self { radius: 50.0 }
    }
}

#[derive(Component)]
pub struct MembraneRing;

fn setup_arena(
    mut commands: Commands,
    cell_radius: Res<CellRadius>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    // Create a circle mesh for the cell membrane using the shape module
    let circle = bevy::render::mesh::shape::Circle::new(cell_radius.radius);
    let mesh_handle = meshes.add(circle.into());
    let material_handle = materials.add(ColorMaterial::from(Color::rgb(0.2, 0.4, 0.8)));
    
    // Spawn the cell membrane as a simple circle
    commands.spawn((
        MembraneRing,
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            transform: Transform::from_translation(Vec3::ZERO),
            ..default()
        },
    ));
}

