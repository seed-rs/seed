use crate::geometry::*;
use nalgebra::{Matrix4, Vector3};
use web_sys::HtmlImageElement;

use awsm_web::webgl::{
    AttributeOptions, BeginMode, BlendFactor, BufferData, BufferMask, BufferTarget, BufferUsage,
    DataType, GlToggle, Id, PixelFormat, SimpleTextureOptions, TextureTarget, WebGl1Renderer,
    WebGlTextureSource,
};

pub struct SceneRenderer {
    pub renderer: WebGl1Renderer,
    ids: SceneIds,
}

struct SceneIds {
    program_id: Id,
    texture_id: Id,
    instance_id: Id,
}
impl SceneRenderer {
    pub fn new(
        mut renderer: WebGl1Renderer,
        vertex: &str,
        fragment: &str,
        img: &HtmlImageElement,
    ) -> Result<Self, awsm_web::errors::Error> {
        let ids = {
            //This demo is specifically using webgl1, which needs to register the extension
            //Everything else is the same API as webgl2 :)
            renderer.register_extension_instanced_arrays()?;

            //compile the shaders and get a program id
            let program_id = renderer.compile_program(vertex, fragment)?;

            //create quad data and get a buffer id
            let geom_id = renderer.create_buffer()?;

            renderer.upload_buffer_to_attribute(
                geom_id,
                BufferData::new(
                    &QUAD_GEOM_UNIT,
                    BufferTarget::ArrayBuffer,
                    BufferUsage::StaticDraw,
                ),
                "a_vertex",
                &AttributeOptions::new(2, DataType::Float),
            )?;

            //create texture data and get a texture id
            let texture_id = renderer.create_texture()?;
            renderer.assign_simple_texture(
                texture_id,
                TextureTarget::Texture2d,
                &SimpleTextureOptions {
                    pixel_format: PixelFormat::Rgba,
                    ..SimpleTextureOptions::default()
                },
                &WebGlTextureSource::ImageElement(&img),
            )?;

            //create an instance buffer and get the id
            let instance_id = renderer.create_buffer()?;

            SceneIds {
                program_id,
                texture_id,
                instance_id,
            }
        };

        renderer.gl.clear_color(0.3, 0.3, 0.3, 1.0);

        Ok(Self { renderer, ids })
    }

    pub fn clear(&mut self) {
        self.renderer
            .clear(&[BufferMask::ColorBufferBit, BufferMask::DepthBufferBit]);
    }
    pub fn render(
        &mut self,
        len: usize,
        img_area: &Area,
        stage_area: &Area,
        instance_positions: &[f32],
    ) -> Result<(), awsm_web::errors::Error> {
        self.clear();

        if len == 0 {
            return Ok(());
        }

        let renderer = &mut self.renderer;
        let SceneIds {
            program_id,
            texture_id,
            instance_id,
            ..
        } = self.ids;

        //set blend mode. this will be a noop internally if already set
        renderer.toggle(GlToggle::Blend, true);
        renderer.toggle(GlToggle::DepthTest, false);
        renderer.set_blend_func(BlendFactor::SrcAlpha, BlendFactor::OneMinusSrcAlpha);

        //will already be activated but internally that's a noop if true
        renderer.activate_program(program_id)?;

        //enable texture
        renderer.activate_texture_for_sampler(texture_id, "u_sampler")?;

        //Build our matrices (must cast to f32)
        let scaling_mat = Matrix4::new_nonuniform_scaling(&Vector3::new(
            img_area.width as f32,
            img_area.height as f32,
            0.0,
        ));
        let camera_mat = Matrix4::new_orthographic(
            0.0,
            stage_area.width as f32,
            0.0,
            stage_area.height as f32,
            0.0,
            1.0,
        );

        //Upload them to the GPU
        renderer.upload_uniform_mat_4("u_size", &scaling_mat.as_slice())?;
        renderer.upload_uniform_mat_4("u_camera", &camera_mat.as_slice())?;

        //need the location for the attrib_divisor below
        let loc = renderer.get_attribute_location_value("a_position")?;
        //upload instance positions
        renderer.upload_buffer(
            instance_id,
            BufferData::new(
                &instance_positions,
                BufferTarget::ArrayBuffer,
                BufferUsage::StaticDraw,
            ),
        )?;

        renderer.activate_attribute_loc(loc, &AttributeOptions::new(2, DataType::Float));

        renderer.vertex_attrib_divisor(loc, 1)?;
        renderer.draw_arrays_instanced(BeginMode::TriangleStrip, 0, 4, len as u32)?;

        Ok(())
    }
}
