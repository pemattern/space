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
            binding_types::{
                sampler, texture_2d, texture_2d_multisampled, texture_depth_2d_multisampled,
                uniform_buffer,
            },
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

pub struct OutlinePlugin;

fn add_components_main_camera(
    mut commands: Commands,
    main_camera_query: Query<Entity, With<MainCamera>>,
) {
    if let Ok(main_camera_entity) = main_camera_query.single() {
        commands
            .entity(main_camera_entity)
            .insert(OutlineSettings { cutoff: 0.5 });
    }
}

impl Plugin for OutlinePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExtractComponentPlugin::<OutlineSettings>::default(),
            UniformComponentPlugin::<OutlineSettings>::default(),
        ))
        .add_systems(Startup, add_components_main_camera);
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };
        render_app
            .add_render_graph_node::<ViewNodeRunner<OutlineNode>>(Core3d, OutlineLabel)
            .add_render_graph_edges(
                Core3d,
                (
                    Node3d::Tonemapping,
                    OutlineLabel,
                    Node3d::EndMainPassPostProcessing,
                ),
            );
    }

    fn finish(&self, app: &mut App) {
        let Some(render_app) = app.get_sub_app_mut(RenderApp) else {
            return;
        };

        render_app.init_resource::<OutlinePipeline>();
    }
}

#[derive(Component, Default, Clone, Copy, ExtractComponent, ShaderType)]
pub struct OutlineSettings {
    pub cutoff: f32,
}

#[derive(Debug, Hash, PartialEq, Eq, Clone, RenderLabel)]
struct OutlineLabel;

#[derive(Default)]
struct OutlineNode;

impl ViewNode for OutlineNode {
    type ViewQuery = (
        &'static ViewTarget,
        &'static OutlineSettings,
        &'static DynamicUniformIndex<OutlineSettings>,
        &'static ViewPrepassTextures,
    );

    fn run(
        &self,
        _graph: &mut RenderGraphContext,
        render_context: &mut RenderContext,
        (view_target, _post_process_settings, settings_index, view_prepass_textures): QueryItem<
            Self::ViewQuery,
        >,
        world: &World,
    ) -> Result<(), NodeRunError> {
        let post_process_pipeline = world.resource::<OutlinePipeline>();
        let pipeline_cache = world.resource::<PipelineCache>();

        let Some(pipeline) = pipeline_cache.get_render_pipeline(post_process_pipeline.pipeline_id)
        else {
            return Ok(());
        };

        let settings_uniforms = world.resource::<ComponentUniforms<OutlineSettings>>();
        let Some(settings_binding) = settings_uniforms.uniforms().binding() else {
            return Ok(());
        };

        let post_process = view_target.post_process_write();
        let depth_view = view_prepass_textures.depth_view().expect("no depth view");
        let normals_view = view_prepass_textures
            .normal_view()
            .expect("no normals view");
        let bind_group = render_context.render_device().create_bind_group(
            "post_process_bind_group",
            &post_process_pipeline.layout,
            &BindGroupEntries::sequential((
                post_process.source,
                &post_process_pipeline.color_sampler,
                depth_view,
                normals_view,
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

impl FromWorld for OutlinePipeline {
    fn from_world(world: &mut World) -> Self {
        let render_device = world.resource::<RenderDevice>();
        let layout = render_device.create_bind_group_layout(
            "post_process_bind_group_layout",
            &BindGroupLayoutEntries::sequential(
                ShaderStages::FRAGMENT,
                (
                    texture_2d(TextureSampleType::Float { filterable: true }),
                    sampler(SamplerBindingType::Filtering),
                    texture_depth_2d_multisampled(),
                    texture_2d_multisampled(TextureSampleType::Float { filterable: false }),
                    uniform_buffer::<OutlineSettings>(true),
                ),
            ),
        );

        let color_sampler = render_device.create_sampler(&SamplerDescriptor::default());
        let shader = world.load_asset("shaders/outline.wgsl");
        let vertex_state = world.resource::<FullscreenShader>().to_vertex_state();
        let pipeline_id =
            world
                .resource_mut::<PipelineCache>()
                .queue_render_pipeline(RenderPipelineDescriptor {
                    label: Some("outline_pipline".into()),
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
struct OutlinePipeline {
    layout: BindGroupLayout,
    color_sampler: Sampler,
    pipeline_id: CachedRenderPipelineId,
}
