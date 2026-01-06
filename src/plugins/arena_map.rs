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
        // Increased from 50.0 to 75.0 for better gameplay space
        // This gives room for modules, enemies, tracks, and vesicles
        Self { radius: 75.0 }
    }
}

#[derive(Component)]
pub struct MembraneRing;

#[derive(Component, Debug, Clone, Copy, PartialEq, Eq)]
pub enum OrganelleType {
    Nucleus,
    ErGolgi,
    Mitochondria,
}

#[derive(Component, Clone)]
pub struct Organelle {
    pub organelle_type: OrganelleType,
    pub radius: f32,
}

impl Organelle {
    pub fn color(&self) -> Color {
        match self.organelle_type {
            OrganelleType::Nucleus => Color::rgb(0.4, 0.2, 0.6), // purple
            OrganelleType::ErGolgi => Color::rgb(0.9, 0.7, 0.2), // yellow/orange
            OrganelleType::Mitochondria => Color::rgb(0.8, 0.3, 0.2), // red/orange
        }
    }
}

#[derive(Resource)]
pub struct OrganelleLayout {
    pub nucleus: OrganellePosition,
    pub er_golgi: OrganellePosition,
    pub mitochondria: Vec<OrganellePosition>,
}

#[derive(Debug, Clone, Copy)]
pub struct OrganellePosition {
    pub position: Vec2,
    pub radius: f32,
    pub organelle_type: OrganelleType,
}

fn create_default_layout(cell_radius: f32) -> OrganelleLayout {
    // Scale organelles proportionally to cell size
    let scale_factor = cell_radius / 50.0; // Base size was 50.0
    
    // Nucleus: center, scaled radius
    let nucleus = OrganellePosition {
        position: Vec2::ZERO,
        radius: 9.0 * scale_factor,
        organelle_type: OrganelleType::Nucleus,
    };

    // ER/Golgi: offset from center at 45° angle, scaled distance and radius
    let er_golgi_angle = std::f32::consts::PI / 4.0; // 45 degrees
    let er_golgi_distance = 15.0 * scale_factor;
    let er_golgi = OrganellePosition {
        position: Vec2::new(
            er_golgi_distance * er_golgi_angle.cos(),
            er_golgi_distance * er_golgi_angle.sin(),
        ),
        radius: 7.0 * scale_factor,
        organelle_type: OrganelleType::ErGolgi,
    };

    // Mitochondria: 3 placed in annulus band, scaled positions and sizes
    let mut mitochondria = Vec::new();
    let mito_angles = [
        std::f32::consts::PI * 0.25,  // 45°
        std::f32::consts::PI * 1.25,  // 225°
        std::f32::consts::PI * 1.75,  // 315°
    ];
    let mito_distance = 25.0 * scale_factor; // Scaled distance
    
    for angle in mito_angles {
        mitochondria.push(OrganellePosition {
            position: Vec2::new(
                mito_distance * angle.cos(),
                mito_distance * angle.sin(),
            ),
            radius: 6.0 * scale_factor,
            organelle_type: OrganelleType::Mitochondria,
        });
    }

    OrganelleLayout {
        nucleus,
        er_golgi,
        mitochondria,
    }
}

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
    
    // Spawn the cell membrane as a simple circle (z = 0, background layer)
    commands.spawn((
        MembraneRing,
        MaterialMesh2dBundle {
            mesh: mesh_handle.into(),
            material: material_handle,
            transform: Transform::from_translation(Vec3::new(0.0, 0.0, 0.0)),
            ..default()
        },
    ));

    // Create the organelle layout
    let layout = create_default_layout(cell_radius.radius);
    
    // Spawn nucleus (z = 1, in front of membrane)
    let nucleus_circle = bevy::render::mesh::shape::Circle::new(layout.nucleus.radius);
    let nucleus_mesh = meshes.add(nucleus_circle.into());
    let nucleus_organelle = Organelle {
        organelle_type: OrganelleType::Nucleus,
        radius: layout.nucleus.radius,
    };
    let nucleus_color = nucleus_organelle.color();
    commands.spawn((
        nucleus_organelle,
        MaterialMesh2dBundle {
            mesh: nucleus_mesh.into(),
            material: materials.add(ColorMaterial::from(nucleus_color)),
            transform: Transform::from_translation(layout.nucleus.position.extend(1.0)),
            ..default()
        },
    ));

    // Spawn ER/Golgi (z = 1, same layer as other organelles)
    let er_golgi_circle = bevy::render::mesh::shape::Circle::new(layout.er_golgi.radius);
    let er_golgi_mesh = meshes.add(er_golgi_circle.into());
    let er_golgi_organelle = Organelle {
        organelle_type: OrganelleType::ErGolgi,
        radius: layout.er_golgi.radius,
    };
    let er_golgi_color = er_golgi_organelle.color();
    commands.spawn((
        er_golgi_organelle,
        MaterialMesh2dBundle {
            mesh: er_golgi_mesh.into(),
            material: materials.add(ColorMaterial::from(er_golgi_color)),
            transform: Transform::from_translation(layout.er_golgi.position.extend(1.0)),
            ..default()
        },
    ));

    // Spawn mitochondria (z = 1, same layer as other organelles)
    for mito_pos in &layout.mitochondria {
        let mito_circle = bevy::render::mesh::shape::Circle::new(mito_pos.radius);
        let mito_mesh = meshes.add(mito_circle.into());
        let mito_organelle = Organelle {
            organelle_type: OrganelleType::Mitochondria,
            radius: mito_pos.radius,
        };
        let mito_color = mito_organelle.color();
        commands.spawn((
            mito_organelle,
            MaterialMesh2dBundle {
                mesh: mito_mesh.into(),
                material: materials.add(ColorMaterial::from(mito_color)),
                transform: Transform::from_translation(mito_pos.position.extend(1.0)),
                ..default()
            },
        ));
    }
}

