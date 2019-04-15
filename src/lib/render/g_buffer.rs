use crate::{
    app::StatusOr,
    file,
    render::{
        attribute,
        Attribute,
        AttributeAdvance,
        AttributeProgram,
        PointLight,
        ShaderProgram
    }
};
use gl::{
    self,
    types::*,
};

pub struct GBuffer {
    frame_buffer: GLuint,
    position_texture: GLuint,
    normal_texture: GLuint,
    color_texture: GLuint,
    depth_render_buffer: GLuint,
    lighting_pass_shader: ShaderProgram,
    lighting_pass_attribute_program: AttributeProgram,
    attr_pos: Attribute<LightingPosAttr>,
    attr_texel: Attribute<LightingTexelAttr>,
    lights: Vec<PointLight>,
}

impl GBuffer {
    pub fn new(window_size: (i32, i32)) -> StatusOr<GBuffer> {
        let vert_path = file::util::resource_path("shaders", "deferred_lighting_vert.glsl");
        let frag_path = file::util::resource_path("shaders", "deferred_lighting_frag.glsl");

        let mut lighting_pass_attribute_program_builder = AttributeProgram::builder();
        let attr_pos = lighting_pass_attribute_program_builder.add_attribute_with_advance(AttributeAdvance::PerVertex);
        let attr_texel = lighting_pass_attribute_program_builder.add_attribute_with_advance(AttributeAdvance::PerVertex);
        let lighting_pass_attribute_program = lighting_pass_attribute_program_builder.build();

        let mut g_buffer = GBuffer {
            frame_buffer: 0,
            position_texture: 0,
            normal_texture: 0,
            color_texture: 0,
            depth_render_buffer: 0,
            lighting_pass_shader: ShaderProgram::from_short_pipeline(&vert_path, &frag_path)?,
            lighting_pass_attribute_program,
            attr_pos,
            attr_texel,
            lights: vec!(),
        };
        g_buffer.resize(window_size.0, window_size.1)?;
        Ok(g_buffer)
    }

    pub fn resize(&mut self, width: GLsizei, height: GLsizei) -> StatusOr<()> {
        self.clear_buffers();
        unsafe {
            gl::GenFramebuffers(1, &mut self.frame_buffer);
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.frame_buffer);
        }

        let setup_texture =
            move |texture: &mut GLuint, format: GBufferTextureFormat, attachment: GLenum| {
                unsafe {
                    gl::GenTextures(1, texture);
                    gl::BindTexture(gl::TEXTURE_2D, *texture);
                    gl::TexImage2D(gl::TEXTURE_2D, 0, format.internal_format, width, height, 0,
                                   format.format, format.pixel_data_type, std::ptr::null());
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::NEAREST as i32);
                    gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::NEAREST as i32);
                    gl::FramebufferTexture2D(gl::FRAMEBUFFER, attachment, gl::TEXTURE_2D, *texture, 0);
                }
            };
        let position_format = GBufferTextureFormat {
            internal_format: gl::RGB16F as i32,
            format: gl::RGB,
            pixel_data_type: gl::FLOAT,
        };
        setup_texture(&mut self.position_texture, position_format, gl::COLOR_ATTACHMENT0);

        let normal_format = GBufferTextureFormat {
            internal_format: gl::RGB16F as i32,
            format: gl::RGB,
            pixel_data_type: gl::FLOAT,
        };
        setup_texture(&mut self.normal_texture, normal_format, gl::COLOR_ATTACHMENT1);

        let color_format = GBufferTextureFormat {
            internal_format: gl::RGBA as i32,
            format: gl::RGBA,
            pixel_data_type: gl::UNSIGNED_BYTE,
        };
        setup_texture(&mut self.color_texture, color_format, gl::COLOR_ATTACHMENT2);

        let attachments: [GLenum; 3] = [gl::COLOR_ATTACHMENT0, gl::COLOR_ATTACHMENT1, gl::COLOR_ATTACHMENT2];
        unsafe {
            gl::DrawBuffers(attachments.len() as i32, attachments.as_ptr());
            gl::GenRenderbuffers(1, &mut self.depth_render_buffer);
            gl::BindRenderbuffer(gl::RENDERBUFFER, self.depth_render_buffer);
            gl::RenderbufferStorage(gl::RENDERBUFFER, gl::DEPTH_COMPONENT, width, height);
            gl::FramebufferRenderbuffer(gl::FRAMEBUFFER, gl::DEPTH_ATTACHMENT, gl::RENDERBUFFER, self.depth_render_buffer);

            if gl::CheckFramebufferStatus(gl::FRAMEBUFFER) != gl::FRAMEBUFFER_COMPLETE {
                return Err(String::from("Framebuffer not complete!"));
            }
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
        }

        // Set texture uniforms once and for all.
        self.lighting_pass_shader.activate();
        self.lighting_pass_shader.set_i32("position_tex", 0);
        self.lighting_pass_shader.set_i32("normal_tex", 1);
        self.lighting_pass_shader.set_i32("color_tex", 2);

        for pos in [
            glm::vec3(-1.0,  1.0, 0.0),
            glm::vec3(-1.0, -1.0, 0.0),
            glm::vec3( 1.0,  1.0, 0.0),
            glm::vec3( 1.0, -1.0, 0.0)].iter() {
            self.attr_pos.data.push( LightingPosAttr {
                position: *pos
            });
        }
        for tex in [
            glm::vec2(0.0, 1.0),
            glm::vec2(0.0, 0.0),
            glm::vec2(1.0, 1.0),
            glm::vec2(1.0, 0.0)].iter() {
            self.attr_texel.data.push( LightingTexelAttr {
                texel: *tex,
            });
        }

        Ok(())
    }

    pub fn lights_mut(&mut self) -> &mut Vec<PointLight> {
        &mut self.lights
    }

    pub fn geometry_pass(&mut self) {
        self.lights.clear();

        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.frame_buffer);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn lighting_pass(&mut self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.lighting_pass_shader.activate();
        if self.lights.len() > 32 {
            panic!("Need to update deferred fragment shader to support more than {} lights", self.lights.len());
        }
        self.lighting_pass_shader.set_i32("num_lights", self.lights.len() as i32);
        for (idx, point_light) in self.lights.iter().enumerate() {
            let position_str = format!("lights[{}].position", idx);
            let color_str = format!("lights[{}].color", idx);
            let attenuation_str = format!("lights[{}].attenuation", idx);
            self.lighting_pass_shader.set_vec3(position_str.as_str(), &point_light.position);
            self.lighting_pass_shader.set_vec3(color_str.as_str(), &point_light.color);
            self.lighting_pass_shader.set_vec3(attenuation_str.as_str(), &point_light.attenuation);
        }

        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.position_texture);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, self.normal_texture);
            gl::ActiveTexture(gl::TEXTURE2);
            gl::BindTexture(gl::TEXTURE_2D, self.color_texture);
        }
        self.lighting_pass_attribute_program.activate();
        self.attr_pos.prepare_buffer();
        self.attr_texel.prepare_buffer();

        unsafe {
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
        }

        self.lighting_pass_attribute_program.deactivate();
    }

    fn clear_buffers(&mut self) {
        unsafe {
            if self.frame_buffer != 0 {
                gl::DeleteFramebuffers(1, &self.frame_buffer);
                self.frame_buffer = 0;
            }
            if self.position_texture != 0 {
                gl::DeleteTextures(1, &self.position_texture);
                self.position_texture = 0;
            }
            if self.normal_texture != 0 {
                gl::DeleteTextures(1, &self.normal_texture);
                self.normal_texture = 0;
            }
            if self.color_texture != 0 {
                gl::DeleteTextures(1, &self.color_texture);
                self.color_texture = 0;
            }
            if self.depth_render_buffer != 0 {
                gl::DeleteRenderbuffers(1, &self.depth_render_buffer);
                self.depth_render_buffer = 0;
            }
        }
    }
}

impl Drop for GBuffer {
    fn drop(&mut self) {
        self.clear_buffers();
    }
}

struct GBufferTextureFormat {
    internal_format: GLint,
    format: GLenum,
    pixel_data_type: GLenum,
}


#[repr(C)]
struct LightingPosAttr {
    position: glm::Vec3,
}

impl attribute::KnownComponent for LightingPosAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S3, attribute::ComponentType::Float)
    }
}

#[repr(C)]
struct LightingTexelAttr {
    texel: glm::Vec2,
}

impl attribute::KnownComponent for LightingTexelAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S2, attribute::ComponentType::Float)
    }
}
