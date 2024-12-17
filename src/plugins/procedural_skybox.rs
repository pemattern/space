use bevy::{
    prelude::*,
    render::render_resource::{AsBindGroup, FrontFace, ShaderRef},
};

pub struct ProceduralSkyboxPlugin;

impl Plugin for ProceduralSkyboxPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(MaterialPlugin::<ProceduralSkyboxMaterial>::default())
            .add_systems(Startup, setup)
            .add_systems(Update, move_with_camera)
            .add_systems(Update, update_material);
    }
}

#[derive(Component)]
pub struct ProceduralSkybox {
    pub material: Handle<ProceduralSkyboxMaterial>,
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ProceduralSkyboxMaterial>>,
) {
    let material = materials.add(ProceduralSkyboxMaterial {
        camera_position: Vec3::ZERO,
    });

    commands.spawn((
        ProceduralSkybox {
            material: material.clone(),
        },
        Mesh3d(meshes.add(Mesh::from(Sphere { radius: 500.0 }))),
        MeshMaterial3d(material),
    ));
}

fn move_with_camera(
    mut skybox_query: Query<&mut Transform, (With<ProceduralSkybox>, Without<Camera>)>,
    camera_query: Query<&Transform, (With<Camera>, Without<ProceduralSkybox>)>,
) {
    if let Ok(mut skybox_transform) = skybox_query.get_single_mut() {
        if let Ok(camera_transform) = camera_query.get_single() {
            skybox_transform.translation = camera_transform.translation;
        }
    }
}

fn update_material(
    mut materials: ResMut<Assets<ProceduralSkyboxMaterial>>,
    skybox_query: Query<&ProceduralSkybox>,
    camera_query: Query<&Transform, With<Camera>>,
) {
    if let Ok(skybox) = skybox_query.get_single() {
        if let Some(material) = materials.get_mut(&skybox.material) {
            if let Ok(camera_transform) = camera_query.get_single() {
                material.camera_position = camera_transform.translation;
            }
        }
    }
}

#[derive(AsBindGroup, TypePath, Asset, Clone)]
pub struct ProceduralSkyboxMaterial {
    #[uniform(0)]
    camera_position: Vec3,
}

impl Material for ProceduralSkyboxMaterial {
    fn fragment_shader() -> ShaderRef {
        "shaders/procedural_skybox.wgsl".into()
    }

    fn specialize(
        _pipeline: &bevy::pbr::MaterialPipeline<Self>,
        descriptor: &mut bevy::render::render_resource::RenderPipelineDescriptor,
        _layout: &bevy::render::mesh::MeshVertexBufferLayoutRef,
        _key: bevy::pbr::MaterialPipelineKey<Self>,
    ) -> Result<(), bevy::render::render_resource::SpecializedMeshPipelineError> {
        descriptor.primitive.front_face = FrontFace::Cw;
        Ok(())
    }
}
