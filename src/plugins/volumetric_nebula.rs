use bevy::{
    core_pipeline::{
        core_3d::graph::{Core3d, Node3d},
        prepass::ViewPrepassTextures,
        FullscreenShader,
    },
    ecs::query::QueryItem,
    prelude::*,
    render::{
        extract_component::{
            ComponentUniforms, DynamicUniformIndex, ExtractComponent, ExtractComponentPlugin,
            UniformComponentPlugin,
        },
        render_graph::{
            NodeRunError, RenderGraphContext, RenderGraphExt, RenderLabel, ViewNode, ViewNodeRunner,
        },
        render_resource::{
            binding_types::{sampler, texture_2d, uniform_buffer},
            BindGroupEntries, BindGroupLayout, BindGroupLayoutEntries, CachedRenderPipelineId,
            ColorTargetState, ColorWrites, FragmentState, MultisampleState, Operations,
            PipelineCache, PrimitiveState, RenderPassColorAttachment, RenderPassDescriptor,
            RenderPipelineDescriptor, Sampler, SamplerBindingType, SamplerDescriptor, ShaderStages,
            ShaderType, TextureFormat, TextureSampleType,
        },
        renderer::{RenderContext, RenderDevice},
        view::ViewTarget,
        RenderApp,
    },
};

use crate::core::main_camera::MainCamera;

pub struct VolumetricNebulaPlugin;

fn add_components_main_camera(
    mut commands: Commands,
    main_camera_query: Query<Entity, With<MainCamera>>,
) {
    if let Ok(main_camera_entity) = main_camera_query.single() {
        commands
            .entity(main_camera_entity)
            .insert(VolumetricNebulaSettings {
                time: 0.0,
                camera_position: Vec3::ZERO,
                camera_right: Vec3::ZERO,
                camera_up: Vec3::ZERO,
                camera_forward: Vec3::ZERO,
                light_direction: Vec3::ZERO,
                speed: 2.0,
                scale: 0.1,
                iso_value: 0.86,
                step_count: 30,
                step_distance: 3.0,
            });
    }
}

fn startup_settings(
    light_query: Query<&Transform, With<DirectionalLight>>,
    mut nebula_query: Query<&mut VolumetricNebulaSettings, With<Camera>>,
) {
    if let Ok(light) = light_query.single() {
        if let Ok(mut nebula) = nebula_query.single_mut() {
            nebula.light_direction = light.forward().as_vec3();
        }
    }
}

fn update_settings(
    mut camera_query: Query<(&Transform, &mut VolumetricNebulaSettings), With<MainCamera>>,
    time: Res<Time>,
) {
    if let Ok(camera) = camera_query.single_mut() {
        let mut settings = camera.1;
        settings.camera_position = camera.0.translation;
        settings.camera_right = camera.0.right().as_vec3();
        settings.camera_up = camera.0.up().as_vec3();
        settings.camera_forward = camera.0.forward().as_vec3();
        settings.time = time.elapsed_secs_wrapped();
    }
}

impl Plugin for VolumetricNebulaPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractComponentPlugin::<VolumetricNebulaSettings>::default(),
            UniformComponentPlugin::<VolumetricNebulaSettings>::default(),
        ))
        .add_systems(Startup, add_components_main_camera)
        .add_systems(PostStartup, startup_settings)
        .add_systems(Update, update_settings);
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app
            .add_render_graph_node::<ViewNodeRunner<VolumetricNebulaNode>>(
                Core3d,
                VolumetricNebulaLabel,
            )
            .add_render_graph_edges(
                Core3d,
                (
                    Node3d::Tonemapping,
                    VolumetricNebulaLabel,
                    Node3d::EndMainPassPostProcessing,
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<VolumetricNebulaPipeline>();
    }
}

#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
pub struct VolumetricNebulaSettings {
    pub time: f32,
    pub camera_position: Vec3,
    pub camera_right: Vec3,
    pub camera_up: Vec3,
    pub camera_forward: Vec3,
    pub light_direction: Vec3,
    pub speed: f32,
    pub scale: f32,
    pub iso_value: f32,
    pub step_count: i32,
    pub step_distance: f32,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct VolumetricNebulaLabel;

#[derive(Default)]
struct VolumetricNebulaNode;

impl ViewNode for VolumetricNebulaNode {
    type ViewQuery = (
        &'static ViewTarget,
        &'static VolumetricNebulaSettings,
        &'static DynamicUniformIndex<VolumetricNebulaSettings>,
        &'static ViewPrepassTextures,
    );

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, _post_process_settings, settings_index, _view_prepass_textures): QueryItem<
            Self::ViewQuery,
        >,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let post_process_pipeline = world.resource::<VolumetricNebulaPipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let Some(pipeline) = pipeline_cache.get_render_pipeline(post_process_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        let settings_uniforms = world.resource::<ComponentUniforms<VolumetricNebulaSettings>>();
        let Some(settings_binding) = settings_uniforms.uniforms().binding() else {
            return Ok(());
        };

        let post_process = view_target.post_process_write();
        let bind_group = render_context.render_device().create_bind_group(
            "post_process_bind_group",
            &post_process_pipeline.layout,
            &BindGroupEntries::sequential((
                post_process.source,
                &post_process_pipeline.color_sampler,
                settings_binding.clone(),
            )),
        );

        let mut render_pass = render_context.begin_tracked_render_pass(RenderPassDescriptor {
            label: Some("post_process_pass"),
            color_attachments: &[Some(RenderPassColorAttachment {
                view: post_process.destination,
                resolve_target: None,
                ops: Operations::default(),
                depth_slice: None,
            })],
            depth_stencil_attachment: None,
            timestamp_writes: None,
            occlusion_query_set: None,
        });

        render_pass.set_render_pipeline(pipeline);
        render_pass.set_bind_group(0, &bind_group, &[settings_index.index()]);
        render_pass.draw(0..3, 0..1);

        Ok(())
    }
}

impl FromWorld for VolumetricNebulaPipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let layout = render_device.create_bind_group_layout(
            "post_process_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    uniform_buffer::<VolumetricNebulaSettings>(true),
                ),
            ),
        );

        let color_sampler = render_device.create_sampler(&SamplerDescriptor::default());
        let shader = world.load_asset("shaders/volumetric_nebula.wgsl");
        let vertex_state = world.resource::<FullscreenShader>().to_vertex_state();
        let pipeline_id =
            world
                .resource_mut::<PipelineCache>()
                .queue_render_pipeline(RenderPipelineDescriptor {
                    label: Some("volumetric_nebula_pipeline".into()),
                    layout: vec![layout.clone()],
                    vertex: vertex_state,
                    fragment: Some(FragmentState {
                        shader,
                        shader_defs: vec![],
                        entry_point: Some("fragment".into()),
                        targets: vec![Some(ColorTargetState {
                            format: TextureFormat::bevy_default(),
                            blend: None,
                            write_mask: ColorWrites::ALL,
                        })],
                    }),
                    primitive: PrimitiveState::default(),
                    depth_stencil: None,
                    multisample: MultisampleState::default(),
                    push_constant_ranges: vec![],
                    zero_initialize_workgroup_memory: true,
                });

        Self {
            layout,
            color_sampler,
            pipeline_id,
        }
    }
}

#[derive(Resource)]
struct VolumetricNebulaPipeline {
    layout: BindGroupLayout,
    color_sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}
