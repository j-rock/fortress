use app::StatusOr;
use gl::{
    self,
    types::*,
};
use file;
use render::ShaderProgram;
use std;

// Used for glTexImage2D() call.
struct GBufferTextureFormat {
    internal_format: GLint,
    format: GLenum,
    pixel_data_type: GLenum,
}

pub struct GBuffer {
    frame_buffer: GLuint,
    position_texture: GLuint,
    normal_texture: GLuint,
    color_texture: GLuint,
    depth_render_buffer: GLuint,
    quad_vao: GLuint,
    quad_vbo: GLuint,
    lighting_pass_shader: ShaderProgram,
}

impl GBuffer {
    pub fn new(window_size: &(i32, i32)) -> StatusOr<GBuffer> {
        let vert_path = file::util::resource_path("shaders", "deferred_lighting_vert.glsl");
        let frag_path = file::util::resource_path("shaders", "deferred_lighting_frag.glsl");
        let mut g_buffer = GBuffer {
            frame_buffer: 0,
            position_texture: 0,
            normal_texture: 0,
            color_texture: 0,
            depth_render_buffer: 0,
            quad_vao: 0,
            quad_vbo: 0,
            lighting_pass_shader: ShaderProgram::from_short_pipeline(&vert_path, &frag_path)?
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
        self.lighting_pass_shader.set_i32("position_tex", 0); // Attachment values
        self.lighting_pass_shader.set_i32("normal_tex", 1);
        self.lighting_pass_shader.set_i32("color_tex", 2);

        // Prepare lighting pass quad.
        let vertices: [f32; 20] = [
            // positions        // texture Coords
            -1.0,  1.0, 0.0, 0.0, 1.0,
            -1.0, -1.0, 0.0, 0.0, 0.0,
             1.0,  1.0, 0.0, 1.0, 1.0,
             1.0, -1.0, 0.0, 1.0, 0.0,
        ];
        unsafe {
            gl::GenVertexArrays(1, &mut self.quad_vao);
            gl::GenBuffers(1, &mut self.quad_vbo);
            gl::BindVertexArray(self.quad_vao);
            gl::BindBuffer(gl::ARRAY_BUFFER, self.quad_vbo);
            let float_size = std::mem::size_of::<f32>() as isize;
            let vertex_array_byte_size = vertices.len() as isize * float_size;
            gl::BufferData(gl::ARRAY_BUFFER, vertex_array_byte_size, vertices.as_ptr() as *const GLvoid, gl::STATIC_DRAW);
            // Vertex positions goes into attrib array = 0
            gl::EnableVertexAttribArray(0);
            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 5 * float_size as i32, std::ptr::null());
            // Texture coords goes into attrib array = 1
            let tex_coord_offset = (std::ptr::null() as *const GLvoid).offset(3 * float_size);
            gl::EnableVertexAttribArray(1);
            gl::VertexAttribPointer(1, 2, gl::FLOAT, gl::FALSE, 5 * float_size as i32, tex_coord_offset);
        }
        Ok(())
    }

    pub fn geometry_pass(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, self.frame_buffer);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
    }

    pub fn lighting_pass(&self) {
        unsafe {
            gl::BindFramebuffer(gl::FRAMEBUFFER, 0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
        }
        self.lighting_pass_shader.activate();
        unsafe {
            gl::ActiveTexture(gl::TEXTURE0);
            gl::BindTexture(gl::TEXTURE_2D, self.position_texture);
            gl::ActiveTexture(gl::TEXTURE1);
            gl::BindTexture(gl::TEXTURE_2D, self.normal_texture);
            gl::ActiveTexture(gl::TEXTURE2);
            gl::BindTexture(gl::TEXTURE_2D, self.color_texture);
        }
        unsafe {
            gl::BindVertexArray(self.quad_vao);
            gl::DrawArrays(gl::TRIANGLE_STRIP, 0, 4);
            gl::BindVertexArray(0);
        }
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
            if self.quad_vao != 0 {
                gl::DeleteVertexArrays(1, &self.quad_vao);
                self.quad_vao = 0;
            }
            if self.quad_vbo != 0 {
                gl::DeleteBuffers(1, &self.quad_vbo);
                self.quad_vbo = 0;
            }
        }
    }
}

impl Drop for GBuffer {
    fn drop(&mut self) {
        self.clear_buffers();
    }
}
