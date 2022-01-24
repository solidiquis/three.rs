use regex::Regex;
use std::convert::AsRef;
use std::collections::HashMap;
use wasm_bindgen::JsValue;
use web_sys::WebGlProgram;
use web_sys::WebGlRenderingContext as GL;
use web_sys::WebGlShader;
use web_sys::WebGlUniformLocation;

enum ShaderType {
    Vertex,
    Fragment,
}

impl AsRef<str> for ShaderType {
    fn as_ref(&self) -> &str {
        match self {
            ShaderType::Vertex => "vertex shader",
            ShaderType::Fragment => "fragment shader",
        }
    }
}

pub struct Shader {
    pub program: WebGlProgram,
    pub vertex_shader_uniform_locations: Option<HashMap<String, WebGlUniformLocation>>,
    pub vertex_shader_attribute_locations: Option<HashMap<String, i32>>,
    pub fragment_shader_uniform_locations: Option<HashMap<String, WebGlUniformLocation>>,
}

type ShaderResult<T> = Result<T, JsValue>;

impl Shader {
    pub fn new<S: AsRef<str>>(
        gl: &GL,
        program_name: S,
        vertex_src: S,
        fragment_src: S,
    ) -> ShaderResult<Self> {
        let vs_src = vertex_src.as_ref();
        let fs_src = fragment_src.as_ref();

        let program = gl
            .create_program()
            .ok_or_else(|| JsValue::from("Failed to initialize shader program."))?;

        let pname = program_name.as_ref();

        let vs = gl
            .create_shader(GL::VERTEX_SHADER)
            .ok_or_else(|| JsValue::from_str("Failed to initialize vertex shader."))?;

        Self::compile_shader(gl, pname, &vs, vs_src, ShaderType::Vertex)?;

        let fs = gl
            .create_shader(GL::FRAGMENT_SHADER)
            .ok_or_else(|| JsValue::from_str("Failed to initialize fragment shader."))?;

        Self::compile_shader(gl, pname, &fs, fs_src, ShaderType::Fragment)?;

        gl.attach_shader(&program, &vs);
        gl.attach_shader(&program, &fs);

        Self::link_gpu_program(&gl, pname, &program)?;

        // Program successfully shipped to GPU; no need to keep in CPU.
        gl.delete_shader(Some(&vs));
        gl.delete_shader(Some(&fs));

        let vertex_shader_uniform_locations = Self::get_uniform_locations(&gl, &program, vs_src);
        let vertex_shader_attribute_locations = Self::get_attribute_locations(&gl, &program, vs_src);
        let fragment_shader_uniform_locations = Self::get_uniform_locations(&gl, &program, fs_src);

        Ok(Self {
            program,
            vertex_shader_uniform_locations,
            vertex_shader_attribute_locations,
            fragment_shader_uniform_locations
        })
    }

    pub fn use_shader(&self, gl: &GL) {
        gl.use_program(Some(&self.program));
    }

    pub fn get_program(&self) -> &WebGlProgram {
        &self.program
    }

    fn get_attribute_locations(
        gl: &GL,
        program: &WebGlProgram,
        shader_src: &str
    ) -> Option<HashMap<String, i32>> {
        let re = Regex::new(r"attribute\s+[^ ]+\s+(?P<attr>[^;]+)").unwrap();

        if re.captures_iter(shader_src).count() == 0 {
            return None
        }
        let mut locations = HashMap::<String, i32>::new();

        for capture in re.captures_iter(shader_src) {
            let location = gl.get_attrib_location(program, &capture["attr"]);
            if location == -1 {
                continue
            }
            locations.insert(capture["attr"].to_string(), location);
        }

        Some(locations)
    }

    fn get_uniform_locations(
        gl: &GL,
        program: &WebGlProgram,
        shader_src: &str
    ) -> Option<HashMap<String, WebGlUniformLocation>> {
        let re = Regex::new(r"uniform\s+[^ ]+\s+(?P<uni>[^;]+)").unwrap();

        if re.captures_iter(shader_src).count() == 0 {
            return None
        }
        let mut locations = HashMap::<String, WebGlUniformLocation>::new();

        for capture in re.captures_iter(shader_src) {
            if let Some(location) = gl.get_uniform_location(program, &capture["uni"]) {
                locations.insert(capture["uni"].to_string(), location);
            }
        }

        Some(locations)
    }

    fn link_gpu_program(
        gl: &GL,
        program_name: &str,
        program: &WebGlProgram,
    ) -> ShaderResult<()> {
        gl.link_program(program);
        gl.validate_program(program);

        if let true = gl
            .get_program_parameter(program, GL::LINK_STATUS)
            .as_bool()
            .unwrap()
        {
            return Ok(());
        }

        let log = gl.get_program_info_log(program).unwrap();
        let msg = format![
            "Failed to link shader program, {}, with err: {}",
            program_name, log
        ];
        let err = JsValue::from(msg);

        Err(err)
    }

    fn compile_shader(
        gl: &GL,
        program_name: &str,
        shader: &WebGlShader,
        shader_src: &str,
        shader_type: ShaderType,
    ) -> ShaderResult<()> {
        gl.shader_source(shader, shader_src);
        gl.compile_shader(shader);

        if let true = gl
            .get_shader_parameter(shader, GL::COMPILE_STATUS)
            .as_bool()
            .unwrap()
        {
            return Ok(());
        }

        let log = gl.get_shader_info_log(shader).unwrap();
        let msg = format![
            "Failed to compile {} for {} with err: {}",
            shader_type.as_ref(),
            program_name,
            log
        ];
        let err = JsValue::from(msg);
        gl.delete_shader(Some(shader));

        Err(err)
    }
}
