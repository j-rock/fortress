use app::StatusOr;
use file;
use gl::{
    self,
    types::*
};
use glm;
use render::{
    attribute,
    Attribute,
    AttributeProgram,
    ShaderProgram
};

pub struct BoxData {
    pub position: glm::Vec2,
    pub half_size: glm::Vec2,
    pub rgba_tl: glm::Vec4,
    pub rgba_tr: glm::Vec4,
    pub rgba_bl: glm::Vec4,
    pub rgba_br: glm::Vec4,
}

pub struct BoxRenderer {
    shader_program: ShaderProgram,
    attribute_program: AttributeProgram,
    attr_pos: Attribute<BoxPositionAttr>,
    attr_tl: Attribute<BoxTopLeftAttr>,
    attr_tr: Attribute<BoxTopRightAttr>,
    attr_bl: Attribute<BoxBottomLeftAttr>,
    attr_br: Attribute<BoxBottomRightAttr>,
}

impl BoxRenderer {
    pub fn new() -> StatusOr<BoxRenderer> {
        let vertex = file::util::resource_path("shaders", "box_vert.glsl");
        let geometry = file::util::resource_path("shaders", "box_geo.glsl");
        let fragment = file::util::resource_path("shaders", "box_frag.glsl");
        let shader_program = ShaderProgram::from_long_pipeline(&vertex, &geometry, &fragment)?;

        let mut attribute_program_builder = AttributeProgram::new();
        let attr_pos = attribute_program_builder.add_attribute();
        let attr_tl = attribute_program_builder.add_attribute();
        let attr_tr = attribute_program_builder.add_attribute();
        let attr_bl = attribute_program_builder.add_attribute();
        let attr_br = attribute_program_builder.add_attribute();
        let attribute_program = attribute_program_builder.build();

        Ok(BoxRenderer {
            shader_program,
            attribute_program,
            attr_pos,
            attr_tl,
            attr_tr,
            attr_bl,
            attr_br,
        })
    }

    pub fn queue(&mut self, data: &[BoxData]) {
        for datum in data.iter() {
            self.attr_pos.data.push(BoxPositionAttr {
                position: datum.position,
                half_size: datum.half_size,
            });
            self.attr_tl.data.push(BoxTopLeftAttr {
                rgba: datum.rgba_tl,
            });
            self.attr_tr.data.push(BoxTopRightAttr {
                rgba: datum.rgba_tr,
            });
            self.attr_bl.data.push(BoxBottomLeftAttr {
                rgba: datum.rgba_bl,
            });
            self.attr_br.data.push(BoxBottomRightAttr {
                rgba: datum.rgba_br,
            });
        }
    }

    pub fn draw(&mut self, projection_view: &glm::Mat4) {
        self.shader_program.activate();
        self.shader_program.set_mat4("projection_view", projection_view);
        self.attribute_program.activate();
        self.attr_pos.prepare_buffer();
        self.attr_tl.prepare_buffer();
        self.attr_tr.prepare_buffer();
        self.attr_bl.prepare_buffer();
        self.attr_br.prepare_buffer();

        unsafe {
            gl::DrawArraysInstanced(gl::POINTS, 0, 4, self.attr_pos.data.len() as GLsizei);
        }

        self.attribute_program.deactivate();
        self.shader_program.deactivate();

        self.attr_pos.data.clear();
        self.attr_tl.data.clear();
        self.attr_tr.data.clear();
        self.attr_bl.data.clear();
        self.attr_br.data.clear();
    }
}

#[repr(C)]
struct BoxPositionAttr {
    position: glm::Vec2,
    half_size: glm::Vec2,
}

#[repr(C)]
struct BoxTopLeftAttr {
    rgba: glm::Vec4
}

#[repr(C)]
struct BoxTopRightAttr {
    rgba: glm::Vec4
}

#[repr(C)]
struct BoxBottomLeftAttr {
    rgba: glm::Vec4
}

#[repr(C)]
struct BoxBottomRightAttr {
    rgba: glm::Vec4
}

impl attribute::KnownComponent for BoxPositionAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S4, attribute::ComponentType::FLOAT)
    }
}

impl attribute::KnownComponent for BoxTopLeftAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S4, attribute::ComponentType::FLOAT)
    }
}

impl attribute::KnownComponent for BoxTopRightAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S4, attribute::ComponentType::FLOAT)
    }
}

impl attribute::KnownComponent for BoxBottomLeftAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S4, attribute::ComponentType::FLOAT)
    }
}

impl attribute::KnownComponent for BoxBottomRightAttr {
    fn component() -> (attribute::NumComponents, attribute::ComponentType) {
        (attribute::NumComponents::S4, attribute::ComponentType::FLOAT)
    }
}
