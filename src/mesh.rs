use crate::shader::Shader;
use js_sys::{Float32Array, Uint16Array};
use wasm_bindgen::JsValue;
use web_sys::WebGlProgram;
use web_sys::WebGlRenderingContext as GL;

type MeshResult<T> = Result<T, JsValue>;

pub enum BufferUsage {
    Static,
    Dynamic,
    Stream,
}

impl BufferUsage {
    pub fn as_u32(&self) -> u32 {
        match self {
            BufferUsage::Static => GL::STATIC_DRAW,
            BufferUsage::Dynamic => GL::DYNAMIC_DRAW,
            BufferUsage::Stream => GL::STREAM_DRAW,
        }
    }
}

pub struct Attribute {
    name: String,
    data: Vec<f32>,
    buffer_usage: BufferUsage,
    pub num_component: u8,
    stride: Option<u8>,
}

impl Attribute {
    pub fn new(
        name: String,
        data: Vec<f32>,
        buffer_usage: BufferUsage,
        num_component: u8,
        stride: Option<u8>
    ) -> Self {
        Self { name, data, buffer_usage, num_component, stride }
    }

    pub fn get_stride(&self) -> u8 {
        self.stride.unwrap_or(0)
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }

    pub fn get_data(&self) -> &[f32] {
        self.data.as_slice()
    }

    pub fn get_attrib_location(&self, gl: &GL, program: &WebGlProgram) -> u32 {
        gl.get_attrib_location(program, self.get_name()) as u32
    }

    pub fn get_buffer_usage(&self) -> u32 {
        self.buffer_usage.as_u32()
    }
}

pub struct Indices {
    data: Vec<u16>,
    buffer_usage: BufferUsage,
}

impl Indices {
    pub fn get_data(&self) -> &[u16] {
        self.data.as_slice()
    }

    pub fn get_buffer_usage(&self) -> u32 {
        self.buffer_usage.as_u32()
    }
}

pub struct Mesh {
    pub name: String,
    pub shader: Shader,
    pub position_attr: Attribute,
    pub normal_attr: Option<Attribute>,
    pub color_attr: Option<Attribute>,
    pub texture_attr: Option<Attribute>,
    pub indices: Option<Indices>,
}

impl Mesh {
    pub fn new(
        gl: &GL,
        name: String,
        shader: Shader,
        position_attr: Attribute,
        normal_attr: Option<Attribute>,
        color_attr: Option<Attribute>,
        texture_attr: Option<Attribute>,
        indices: Option<Indices>,
    ) -> MeshResult<Self> {
        Self::init_vertex_data(gl, &shader, &name, &position_attr)?;

        if let Some(ref attr) = normal_attr {
            Self::init_vertex_data(gl, &shader, &name, attr)?
        }

        if let Some(ref attr) = color_attr {
            Self::init_vertex_data(gl, &shader, &name, attr)?
        }

        if let Some(ref attr) = texture_attr {
            Self::init_vertex_data(gl, &shader, &name, attr)?
        }

        if let Some(ref attr) = indices {
            Self::init_indices(gl, &name, attr)?
        }

        Ok(Self {
            name,
            shader,
            position_attr,
            normal_attr,
            color_attr,
            texture_attr,
            indices,
        })
    }

    fn init_vertex_data(
        gl: &GL,
        shader: &Shader,
        mesh_name: &str,
        attr: &Attribute
    ) -> MeshResult<()> {
        let vbo = gl.create_buffer()
            .ok_or_else(|| JsValue::from(format!["Failed to initialize VBO for {}", mesh_name]))?;

        let buffer_data = Float32Array::from(attr.get_data()).buffer();

        gl.bind_buffer(GL::ARRAY_BUFFER, Some(&vbo));
        gl.buffer_data_with_opt_array_buffer(
            GL::ARRAY_BUFFER, Some(&buffer_data), attr.get_buffer_usage()
        );

        let attr_pos = attr.get_attrib_location(gl, shader.get_program());

        gl.vertex_attrib_pointer_with_i32(
            attr_pos,
            attr.num_component.into(),
            GL::FLOAT,
            false,
            attr.get_stride().into(),
            0
        );
        gl.enable_vertex_attrib_array(attr_pos);

        Ok(())
    }

    fn init_indices(gl: &GL, mesh_name: &str, indices: &Indices) -> MeshResult<()> {
        let index_buffer = gl.create_buffer()
            .ok_or_else(|| JsValue::from(format!("Failed to initialize index buffer for {}", mesh_name)))?;

        gl.bind_buffer(GL::ELEMENT_ARRAY_BUFFER, Some(&index_buffer));

        let index_data = Uint16Array::from(indices.get_data()).buffer();

        gl.buffer_data_with_opt_array_buffer(
            GL::ELEMENT_ARRAY_BUFFER, Some(&index_data), indices.get_buffer_usage()
        );

        Ok(())
    }
}
