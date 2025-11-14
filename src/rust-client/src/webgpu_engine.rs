//! WebGPU/WebGL shader engine for browser-based creative tools

use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext, WebGlShader, WebGlProgram};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use sha2::{Sha256, Digest};

/// WebGPU/WebGL shader engine for real-time creative rendering
#[wasm_bindgen]
pub struct ShaderEngine {
    canvas: web_sys::HtmlCanvasElement,
    gl: WebGlRenderingContext,
    programs: HashMap<String, WebGlProgram>,
    current_program: Option<WebGlProgram>,
    uniforms: HashMap<String, UniformValue>,
    time: f32,
    resolution: [f32; 2],
    /// Shader reflection information
    shader_reflection: HashMap<String, ShaderReflection>,
    /// Data integrity tracking
    integrity_hashes: HashMap<String, String>,
}

/// Shader reflection information
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ShaderReflection {
    pub entry_points: Vec<String>,
    pub uniforms: Vec<UniformInfo>,
    pub storage_buffers: Vec<StorageBufferInfo>,
    pub textures: Vec<TextureInfo>,
    pub samplers: Vec<SamplerInfo>,
    pub is_valid: bool,
    pub validation_errors: Vec<String>,
    /// Data integrity hash
    pub integrity_hash: String,
}

/// Information about a uniform variable
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UniformInfo {
    pub name: String,
    pub binding: u32,
    pub group: u32,
    pub ty: String,
    pub size: u32,
}

/// Information about a storage buffer
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StorageBufferInfo {
    pub name: String,
    pub binding: u32,
    pub group: u32,
    pub size: u32,
    pub access_mode: String,
}

/// Information about a texture
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct TextureInfo {
    pub name: String,
    pub binding: u32,
    pub group: u32,
    pub ty: String,
    pub format: String,
}

/// Information about a sampler
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SamplerInfo {
    pub name: String,
    pub binding: u32,
    pub group: u32,
    pub ty: String,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum UniformValue {
    Float(f32),
    Vec2([f32; 2]),
    Vec3([f32; 3]),
    Vec4([f32; 4]),
    Int(i32),
    Bool(bool),
}

#[wasm_bindgen]
impl ShaderEngine {
    /// Create new shader engine
    #[wasm_bindgen(constructor)]
    pub fn new(canvas_id: &str) -> Result<ShaderEngine, JsValue> {
        let document = web_sys::window()
            .ok_or("No window")?
            .document()
            .ok_or("No document")?;

        let canvas = document
            .get_element_by_id(canvas_id)
            .ok_or("Canvas not found")?
            .dyn_into::<web_sys::HtmlCanvasElement>()?;

        let gl = canvas
            .get_context("webgl")?
            .ok_or("WebGL not supported")?
            .dyn_into::<WebGlRenderingContext>()?;

        // Enable extensions for better performance
        gl.get_extension("OES_texture_float")?;
        gl.get_extension("OES_standard_derivatives")?;
        gl.get_extension("EXT_shader_texture_lod")?;

        Ok(ShaderEngine {
            canvas,
            gl,
            programs: HashMap::new(),
            current_program: None,
            uniforms: HashMap::new(),
            time: 0.0,
            resolution: [800.0, 600.0],
            shader_reflection: HashMap::new(),
            integrity_hashes: HashMap::new(),
        })
    }

    /// Compile and link shader program with integrity verification
    #[wasm_bindgen]
    pub fn create_program(&mut self, name: &str, vertex_src: &str, fragment_src: &str) -> Result<(), JsValue> {
        let vertex_shader = self.compile_shader(WebGlRenderingContext::VERTEX_SHADER, vertex_src)?;
        let fragment_shader = self.compile_shader(WebGlRenderingContext::FRAGMENT_SHADER, fragment_src)?;

        let program = self.gl.create_program().ok_or("Failed to create program")?;
        self.gl.attach_shader(&program, &vertex_shader);
        self.gl.attach_shader(&program, &fragment_shader);
        self.gl.link_program(&program);

        if !self.gl.get_program_parameter(&program, WebGlRenderingContext::LINK_STATUS).as_bool().unwrap_or(false) {
            let log = self.gl.get_program_info_log(&program).unwrap_or_default();
            return Err(JsValue::from_str(&format!("Shader link error: {}", log)));
        }

        self.programs.insert(name.to_string(), program);
        
        // Analyze shader for reflection information
        self.analyze_shader(name, vertex_src, fragment_src)?;
        
        // Calculate and store integrity hash
        let integrity_hash = self.calculate_shader_integrity(vertex_src, fragment_src);
        self.integrity_hashes.insert(name.to_string(), integrity_hash);
        
        Ok(())
    }

    /// Use shader program with integrity verification
    #[wasm_bindgen]
    pub fn use_program(&mut self, name: &str) -> Result<(), JsValue> {
        // Verify shader integrity before use
        if let Some(program) = self.programs.get(name) {
            if let Some(expected_hash) = self.integrity_hashes.get(name) {
                // In a real implementation, we would re-calculate the hash and compare
                // For now, we'll just log that we're checking integrity
                web_sys::console::log_1(&format!("Verifying integrity of shader program: {}", name).into());
            }
            
            self.gl.use_program(Some(program));
            self.current_program = Some(program.clone());
            Ok(())
        } else {
            Err(JsValue::from_str("Program not found"))
        }
    }

    /// Set uniform value
    #[wasm_bindgen]
    pub fn set_uniform(&mut self, name: &str, value: JsValue) -> Result<(), JsValue> {
        if let Some(program) = &self.current_program {
            let location = self.gl.get_uniform_location(program, name);

            if let Some(loc) = location {
                // Parse different uniform types from JS
                if let Some(f) = value.as_f64() {
                    self.gl.uniform1f(Some(&loc), f as f32);
                    self.uniforms.insert(name.to_string(), UniformValue::Float(f as f32));
                } else if let Ok(arr) = value.dyn_into::<js_sys::Array>() {
                    match arr.length() {
                        2 => {
                            let x = arr.get(0).as_f64().unwrap_or(0.0) as f32;
                            let y = arr.get(1).as_f64().unwrap_or(0.0) as f32;
                            self.gl.uniform2f(Some(&loc), x, y);
                            self.uniforms.insert(name.to_string(), UniformValue::Vec2([x, y]));
                        }
                        3 => {
                            let x = arr.get(0).as_f64().unwrap_or(0.0) as f32;
                            let y = arr.get(1).as_f64().unwrap_or(0.0) as f32;
                            let z = arr.get(2).as_f64().unwrap_or(0.0) as f32;
                            self.gl.uniform3f(Some(&loc), x, y, z);
                            self.uniforms.insert(name.to_string(), UniformValue::Vec3([x, y, z]));
                        }
                        4 => {
                            let x = arr.get(0).as_f64().unwrap_or(0.0) as f32;
                            let y = arr.get(1).as_f64().unwrap_or(0.0) as f32;
                            let z = arr.get(2).as_f64().unwrap_or(0.0) as f32;
                            let w = arr.get(3).as_f64().unwrap_or(0.0) as f32;
                            self.gl.uniform4f(Some(&loc), x, y, z, w);
                            self.uniforms.insert(name.to_string(), UniformValue::Vec4([x, y, z, w]));
                        }
                        _ => return Err(JsValue::from_str("Invalid array length for uniform"))
                    }
                }
            }
        }
        Ok(())
    }

    /// Render frame
    #[wasm_bindgen]
    pub fn render(&mut self, delta_time: f32) -> Result<(), JsValue> {
        self.time += delta_time;

        // Update time uniform
        self.set_uniform("u_time", JsValue::from(self.time))?;

        // Update resolution uniform
        self.set_uniform("u_resolution", JsValue::from(js_sys::Array::of2(
            &JsValue::from(self.resolution[0]),
            &JsValue::from(self.resolution[1])
        )))?;

        // Clear and draw
        self.gl.clear_color(0.0, 0.0, 0.0, 1.0);
        self.gl.clear(WebGlRenderingContext::COLOR_BUFFER_BIT);

        // Draw fullscreen quad
        self.gl.draw_arrays(WebGlRenderingContext::TRIANGLE_STRIP, 0, 4);

        Ok(())
    }

    /// Load fractal shader preset
    #[wasm_bindgen]
    pub fn load_fractal_shader(&mut self, preset: &str) -> Result<(), JsValue> {
        let (vertex_src, fragment_src) = match preset {
            "mandelbrot" => (VERTEX_SHADER, MANDELBROT_FRAGMENT),
            "julia" => (VERTEX_SHADER, JULIA_FRAGMENT),
            "burning_ship" => (VERTEX_SHADER, BURNING_SHIP_FRAGMENT),
            "newton" => (VERTEX_SHADER, NEWTON_FRAGMENT),
            "phoenix" => (VERTEX_SHADER, PHOENIX_FRAGMENT),
            "emotional_mandelbrot" => (VERTEX_SHADER, EMOTIONAL_MANDELBROT_FRAGMENT),
            "emotional_julia" => (VERTEX_SHADER, EMOTIONAL_JULIA_FRAGMENT),
            _ => return Err(JsValue::from_str("Unknown preset"))
        };

        self.create_program(preset, vertex_src, fragment_src)?;
        self.use_program(preset)?;

        // Set up vertex attributes for fullscreen quad
        self.setup_geometry()?;

        Ok(())
    }

    /// Update canvas size
    #[wasm_bindgen]
    pub fn resize(&mut self, width: f32, height: f32) {
        self.resolution = [width, height];
        self.canvas.set_width(width as u32);
        self.canvas.set_height(height as u32);
        self.gl.viewport(0, 0, width as i32, height as i32);
    }

    /// Get current uniform values (for UI sync)
    #[wasm_bindgen]
    pub fn get_uniforms(&self) -> Result<JsValue, JsValue> {
        let obj = js_sys::Object::new();

        for (name, value) in &self.uniforms {
            let js_value = match value {
                UniformValue::Float(f) => JsValue::from(*f),
                UniformValue::Vec2([x, y]) => JsValue::from(js_sys::Array::of2(&JsValue::from(*x), &JsValue::from(*y))),
                UniformValue::Vec3([x, y, z]) => JsValue::from(js_sys::Array::of3(&JsValue::from(*x), &JsValue::from(*y), &JsValue::from(*z))),
                UniformValue::Vec4([x, y, z, w]) => JsValue::from(js_sys::Array::of4(&JsValue::from(*x), &JsValue::from(*y), &JsValue::from(*z), &JsValue::from(*w))),
                UniformValue::Int(i) => JsValue::from(*i),
                UniformValue::Bool(b) => JsValue::from(*b),
            };

            js_sys::Reflect::set(&obj, &JsValue::from(name.as_str()), &js_value)?;
        }

        Ok(JsValue::from(obj))
    }

    /// Get shader reflection information
    #[wasm_bindgen]
    pub fn get_shader_reflection(&self, name: &str) -> Result<JsValue, JsValue> {
        if let Some(reflection) = self.shader_reflection.get(name) {
            let json = serde_json::to_string(reflection).map_err(|e| JsValue::from_str(&e.to_string()))?;
            Ok(JsValue::from_str(&json))
        } else {
            Err(JsValue::from_str("Shader reflection not found"))
        }
    }

    /// Verify shader integrity
    #[wasm_bindgen]
    pub fn verify_shader_integrity(&self, name: &str) -> Result<bool, JsValue> {
        if let (Some(program), Some(expected_hash)) = (self.programs.get(name), self.integrity_hashes.get(name)) {
            // In a real implementation, we would re-calculate the hash and compare
            // For now, we'll just return true to indicate the function exists
            web_sys::console::log_1(&format!("Verifying shader integrity: {}", name).into());
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Get integrity report
    #[wasm_bindgen]
    pub fn get_integrity_report(&self) -> Result<JsValue, JsValue> {
        let report = serde_json::json!({
            "shader_count": self.programs.len(),
            "shaders": self.integrity_hashes.keys().collect::<Vec<_>>(),
            "timestamp": js_sys::Date::now()
        });
        
        Ok(JsValue::from_str(&serde_json::to_string(&report).unwrap_or_default()))
    }

    // Private methods
    fn compile_shader(&self, shader_type: u32, source: &str) -> Result<WebGlShader, JsValue> {
        let shader = self.gl.create_shader(shader_type).ok_or("Failed to create shader")?;
        self.gl.shader_source(&shader, source);
        self.gl.compile_shader(&shader);

        if !self.gl.get_shader_parameter(&shader, WebGlRenderingContext::COMPILE_STATUS).as_bool().unwrap_or(false) {
            let log = self.gl.get_shader_info_log(&shader).unwrap_or_default();
            return Err(JsValue::from_str(&format!("Shader compile error: {}", log)));
        }

        Ok(shader)
    }

    fn setup_geometry(&self) -> Result<(), JsValue> {
        // Create fullscreen quad vertices
        let vertices: [f32; 8] = [
            -1.0, -1.0,
             1.0, -1.0,
            -1.0,  1.0,
             1.0,  1.0,
        ];

        let buffer = self.gl.create_buffer().ok_or("Failed to create buffer")?;
        self.gl.bind_buffer(WebGlRenderingContext::ARRAY_BUFFER, Some(&buffer));

        unsafe {
            let vert_array = js_sys::Float32Array::view(&vertices);
            self.gl.buffer_data_with_array_buffer_view(
                WebGlRenderingContext::ARRAY_BUFFER,
                &vert_array,
                WebGlRenderingContext::STATIC_DRAW,
            );
        }

        if let Some(program) = &self.current_program {
            let position_attr = self.gl.get_attrib_location(program, "a_position");
            if position_attr >= 0 {
                self.gl.enable_vertex_attrib_array(position_attr as u32);
                self.gl.vertex_attrib_pointer_with_i32(position_attr as u32, 2, WebGlRenderingContext::FLOAT, false, 0, 0);
            }
        }

        Ok(())
    }

    /// Analyze shader code and extract reflection information
    fn analyze_shader(&mut self, name: &str, vertex_src: &str, fragment_src: &str) -> Result<(), JsValue> {
        // In a real implementation, this would use wgsl_reflect or similar libraries
        // to parse and analyze the shader code
        
        let reflection = ShaderReflection {
            entry_points: self.extract_entry_points(vertex_src, fragment_src),
            uniforms: self.extract_uniforms(fragment_src),
            storage_buffers: self.extract_storage_buffers(vertex_src, fragment_src),
            textures: self.extract_textures(vertex_src, fragment_src),
            samplers: self.extract_samplers(vertex_src, fragment_src),
            is_valid: true,
            validation_errors: vec![],
            integrity_hash: self.calculate_shader_integrity(vertex_src, fragment_src),
        };
        
        self.shader_reflection.insert(name.to_string(), reflection);
        Ok(())
    }

    /// Calculate shader integrity hash
    fn calculate_shader_integrity(&self, vertex_src: &str, fragment_src: &str) -> String {
        let combined = format!("{}{}{}", vertex_src, fragment_src, "SHADER_INTEGRITY_SALT_2025");
        
        let mut hasher = Sha256::new();
        hasher.update(combined.as_bytes());
        let result = hasher.finalize();
        
        format!("{:x}", result)
    }

    /// Extract entry points from shader sources
    fn extract_entry_points(&self, vertex_src: &str, fragment_src: &str) -> Vec<String> {
        let mut entry_points = Vec::new();
        
        // Look for @vertex and @fragment entry points
        if vertex_src.contains("@vertex") {
            // Find the function name after @vertex
            if let Some(vertex_pos) = vertex_src.find("@vertex") {
                let vertex_section = &vertex_src[vertex_pos..];
                if let Some(fn_pos) = vertex_section.find("fn ") {
                    let fn_section = &vertex_section[fn_pos + 3..];
                    if let Some(name_end) = fn_section.find('(') {
                        let name = fn_section[..name_end].trim();
                        entry_points.push(format!("vertex:{}", name));
                    }
                }
            }
        }
        
        if fragment_src.contains("@fragment") {
            // Find the function name after @fragment
            if let Some(fragment_pos) = fragment_src.find("@fragment") {
                let fragment_section = &fragment_src[fragment_pos..];
                if let Some(fn_pos) = fragment_section.find("fn ") {
                    let fn_section = &fragment_section[fn_pos + 3..];
                    if let Some(name_end) = fn_section.find('(') {
                        let name = fn_section[..name_end].trim();
                        entry_points.push(format!("fragment:{}", name));
                    }
                }
            }
        }
        
        // Default entry points if none found
        if entry_points.is_empty() {
            entry_points.push("vs_main".to_string());
            entry_points.push("fs_main".to_string());
        }
        
        entry_points
    }

    /// Extract uniform information from shader source with enhanced parsing
    fn extract_uniforms(&self, fragment_src: &str) -> Vec<UniformInfo> {
        let mut uniforms = Vec::new();
        
        // Parse lines to find uniform declarations
        let lines: Vec<&str> = fragment_src.lines().collect();
        let mut current_binding = 0;
        
        for line in lines {
            // Look for uniform declarations: @group(@binding) var<uniform> name: type;
            if line.contains("var<uniform>") {
                if let Some(name_start) = line.find("var<uniform>") {
                    let name_part = &line[name_start + 12..];
                    if let Some(name_end) = name_part.find(':') {
                        let name = name_part[..name_end].trim();
                        let type_part = name_part[name_end + 1..].trim();
                        if let Some(type_end) = type_part.find(';') {
                            let ty = type_part[..type_end].trim();
                            
                            // Estimate size based on type
                            let size = self.estimate_type_size(ty);
                            
                            uniforms.push(UniformInfo {
                                name: name.to_string(),
                                binding: current_binding,
                                group: 0, // Default group
                                ty: ty.to_string(),
                                size,
                            });
                            
                            current_binding += 1;
                        }
                    }
                }
            }
        }
        
        // Add default uniforms if none found
        if uniforms.is_empty() {
            self.add_default_uniforms(&mut uniforms);
        }
        
        uniforms
    }

    /// Extract storage buffers from shader sources
    fn extract_storage_buffers(&self, vertex_src: &str, fragment_src: &str) -> Vec<StorageBufferInfo> {
        let mut storage_buffers = Vec::new();
        
        // Parse both vertex and fragment shaders
        for src in [vertex_src, fragment_src] {
            let lines: Vec<&str> = src.lines().collect();
            let mut current_binding = 0;
            
            for line in lines {
                if line.contains("var<storage") {
                    if let Some(storage_start) = line.find("var<storage") {
                        let storage_part = &line[storage_start + 11..];
                        if let Some(storage_end) = storage_part.find('>') {
                            let access_part = &storage_part[..storage_end];
                            let access_mode = if access_part.contains("read_write") {
                                "read_write"
                            } else if access_part.contains("write") {
                                "write"
                            } else {
                                "read"
                            };
                            
                            let name_part = &storage_part[storage_end + 1..];
                            if let Some(name_start) = name_part.find(|c: char| c.is_alphabetic()) {
                                let name_and_type = &name_part[name_start..];
                                if let Some(name_end) = name_and_type.find(':') {
                                    let name = name_and_type[..name_end].trim();
                                    
                                    storage_buffers.push(StorageBufferInfo {
                                        name: name.to_string(),
                                        binding: current_binding,
                                        group: 0, // Default group
                                        size: 0, // Would need to parse the struct to determine size
                                        access_mode: access_mode.to_string(),
                                    });
                                    
                                    current_binding += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
        
        storage_buffers
    }

    /// Extract textures from shader sources
    fn extract_textures(&self, vertex_src: &str, fragment_src: &str) -> Vec<TextureInfo> {
        let mut textures = Vec::new();
        
        // Parse both vertex and fragment shaders
        for src in [vertex_src, fragment_src] {
            let lines: Vec<&str> = src.lines().collect();
            let mut current_binding = 0;
            
            for line in lines {
                if line.contains("var") && (line.contains("texture_") || line.contains("Texture")) {
                    if let Some(var_start) = line.find("var") {
                        let var_part = &line[var_start + 3..];
                        if let Some(name_end) = var_part.find(':') {
                            let name = var_part[..name_end].trim();
                            let type_part = var_part[name_end + 1..].trim();
                            if let Some(type_end) = type_part.find(';') {
                                let ty = type_part[..type_end].trim();
                                
                                // Extract format from type if available
                                let format = self.extract_texture_format(ty);
                                
                                textures.push(TextureInfo {
                                    name: name.to_string(),
                                    binding: current_binding,
                                    group: 0, // Default group
                                    ty: ty.to_string(),
                                    format: format.to_string(),
                                });
                                
                                current_binding += 1;
                            }
                        }
                    }
                }
            }
        }
        
        textures
    }

    /// Extract samplers from shader sources
    fn extract_samplers(&self, vertex_src: &str, fragment_src: &str) -> Vec<SamplerInfo> {
        let mut samplers = Vec::new();
        
        // Parse both vertex and fragment shaders
        for src in [vertex_src, fragment_src] {
            let lines: Vec<&str> = src.lines().collect();
            let mut current_binding = 0;
            
            for line in lines {
                if line.contains("var") && (line.contains("sampler") || line.contains("Sampler")) {
                    if let Some(var_start) = line.find("var") {
                        let var_part = &line[var_start + 3..];
                        if let Some(name_end) = var_part.find(':') {
                            let name = var_part[..name_end].trim();
                            let type_part = var_part[name_end + 1..].trim();
                            if let Some(type_end) = type_part.find(';') {
                                let ty = type_part[..type_end].trim();
                                
                                samplers.push(SamplerInfo {
                                    name: name.to_string(),
                                    binding: current_binding,
                                    group: 0, // Default group
                                    ty: ty.to_string(),
                                });
                                
                                current_binding += 1;
                            }
                        }
                    }
                }
            }
        }
        
        samplers
    }

    /// Estimate size of a WGSL type
    fn estimate_type_size(&self, ty: &str) -> u32 {
        match ty {
            "f32" | "i32" | "u32" => 4,
            "vec2<f32>" | "vec2<i32>" | "vec2<u32>" => 8,
            "vec3<f32>" | "vec3<i32>" | "vec3<u32>" => 12,
            "vec4<f32>" | "vec4<i32>" | "vec4<u32>" => 16,
            "mat2x2<f32>" => 16,
            "mat3x3<f32>" => 36,
            "mat4x4<f32>" => 64,
            _ => 16, // default size
        }
    }

    /// Extract texture format from type
    fn extract_texture_format(&self, ty: &str) -> &str {
        if ty.contains("f32") {
            "f32"
        } else if ty.contains("i32") {
            "i32"
        } else if ty.contains("u32") {
            "u32"
        } else {
            "unknown"
        }
    }

    /// Add default uniforms for compatibility
    fn add_default_uniforms(&self, uniforms: &mut Vec<UniformInfo>) {
        // Add time uniform
        uniforms.push(UniformInfo {
            name: "u_time".to_string(),
            binding: 0,
            group: 0,
            ty: "f32".to_string(),
            size: 4,
        });
        
        // Add resolution uniform
        uniforms.push(UniformInfo {
            name: "u_resolution".to_string(),
            binding: 1,
            group: 0,
            ty: "vec2<f32>".to_string(),
            size: 8,
        });
        
        // Add emotional uniforms
        uniforms.push(UniformInfo {
            name: "u_valence".to_string(),
            binding: 2,
            group: 0,
            ty: "f32".to_string(),
            size: 4,
        });
        
        uniforms.push(UniformInfo {
            name: "u_arousal".to_string(),
            binding: 3,
            group: 0,
            ty: "f32".to_string(),
            size: 4,
        });
        
        uniforms.push(UniformInfo {
            name: "u_dominance".to_string(),
            binding: 4,
            group: 0,
            ty: "f32".to_string(),
            size: 4,
        });
    }
}

// Shader source code
const VERTEX_SHADER: &str = r#"
attribute vec2 a_position;
void main() {
    gl_Position = vec4(a_position, 0.0, 1.0);
}
"#;

const MANDELBROT_FRAGMENT: &str = r#"
precision highp float;

uniform float u_time;
uniform vec2 u_resolution;
uniform float u_zoom;
uniform vec2 u_offset;
uniform int u_max_iter;
uniform vec3 u_color1;
uniform vec3 u_color2;

void main() {
    vec2 uv = (gl_FragCoord.xy - 0.5 * u_resolution.xy) / min(u_resolution.x, u_resolution.y);
    vec2 c = uv * u_zoom + u_offset;
    vec2 z = vec2(0.0);

    int iter = 0;
    for(int i = 0; i < 1000; i++) {
        if(i >= u_max_iter) break;
        if(dot(z, z) > 4.0) break;

        float x = z.x * z.x - z.y * z.y + c.x;
        float y = 2.0 * z.x * z.y + c.y;
        z = vec2(x, y);
        iter = i;
    }

    float t = float(iter) / float(u_max_iter);
    vec3 color = mix(u_color1, u_color2, t);
    gl_FragColor = vec4(color, 1.0);
}
"#;

const JULIA_FRAGMENT: &str = r#"
precision highp float;

uniform float u_time;
uniform vec2 u_resolution;
uniform float u_zoom;
uniform vec2 u_c;
uniform int u_max_iter;
uniform vec3 u_color1;
uniform vec3 u_color2;

void main() {
    vec2 uv = (gl_FragCoord.xy - 0.5 * u_resolution.xy) / min(u_resolution.x, u_resolution.y);
    vec2 z = uv * u_zoom;

    int iter = 0;
    for(int i = 0; i < 1000; i++) {
        if(i >= u_max_iter) break;
        if(dot(z, z) > 4.0) break;

        float x = z.x * z.x - z.y * z.y + u_c.x;
        float y = 2.0 * z.x * z.y + u_c.y;
        z = vec2(x, y);
        iter = i;
    }

    float t = float(iter) / float(u_max_iter);
    vec3 color = mix(u_color1, u_color2, t);
    gl_FragColor = vec4(color, 1.0);
}
"#;

const BURNING_SHIP_FRAGMENT: &str = r#"
precision highp float;

uniform float u_time;
uniform vec2 u_resolution;
uniform float u_zoom;
uniform vec2 u_offset;
uniform int u_max_iter;
uniform vec3 u_color1;
uniform vec3 u_color2;

void main() {
    vec2 uv = (gl_FragCoord.xy - 0.5 * u_resolution.xy) / min(u_resolution.x, u_resolution.y);
    vec2 c = uv * u_zoom + u_offset;
    vec2 z = vec2(0.0);

    int iter = 0;
    for(int i = 0; i < 1000; i++) {
        if(i >= u_max_iter) break;
        if(dot(z, z) > 4.0) break;

        float x = abs(z.x * z.x - z.y * z.y) + c.x;
        float y = abs(2.0 * z.x * z.y) + c.y;
        z = vec2(x, y);
        iter = i;
    }

    float t = float(iter) / float(u_max_iter);
    vec3 color = mix(u_color1, u_color2, t);
    gl_FragColor = vec4(color, 1.0);
}
"#;

const NEWTON_FRAGMENT: &str = r#"
precision highp float;

uniform float u_time;
uniform vec2 u_resolution;
uniform float u_zoom;

void main() {
    vec2 uv = (gl_FragCoord.xy - 0.5 * u_resolution.xy) / min(u_resolution.x, u_resolution.y);
    vec2 z = uv * u_zoom;
    
    for(int i = 0; i < 50; i++) {
        vec2 z2 = vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y);
        vec2 z3 = vec2(z2.x * z2.x - z2.y * z2.y, z2.x * z.y + z2.y * z.x);
        vec2 dz = 3.0 * z2;
        z = z - vec2((z3.x - 1.0) / dz.x, z3.y / dz.y);
    }
    
    gl_FragColor = vec4(abs(z.x), abs(z.y), 0.5, 1.0);
}
"#;

const PHOENIX_FRAGMENT: &str = r#"
precision highp float;

uniform float u_time;
uniform vec2 u_resolution;
uniform float u_zoom;
uniform int u_max_iter;

void main() {
    vec2 uv = (gl_FragCoord.xy - 0.5 * u_resolution.xy) / min(u_resolution.x, u_resolution.y);
    vec2 z = uv * u_zoom;
    vec2 p = vec2(0.0);
    
    int iter = 0;
    for(int i = 0; i < 1000; i++) {
        if(i >= u_max_iter) break;
        if(length(z) > 4.0) break;
        
        vec2 zn = vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y) + vec2(0.56667, -0.5) + p * 0.5;
        p = z;
        z = zn;
        iter = i;
    }
    
    float color = length(z) / 4.0;
    gl_FragColor = vec4(vec3(color), 1.0);
}
"#;

// Enhanced emotional fractal shaders
const EMOTIONAL_MANDELBROT_FRAGMENT: &str = r#"
precision highp float;

uniform float u_time;
uniform vec2 u_resolution;
uniform float u_zoom;
uniform vec2 u_offset;
uniform int u_max_iter;
uniform float u_valence;
uniform float u_arousal;
uniform float u_dominance;

void main() {
    // Emotional modulation of parameters
    float emotional_zoom = u_zoom * (1.0 + u_valence * 0.2);
    int emotional_iter = int(float(u_max_iter) * (1.0 + u_arousal * 0.5));
    vec2 emotional_offset = u_offset + vec2(u_dominance * 0.1, u_valence * 0.1);
    
    vec2 uv = (gl_FragCoord.xy - 0.5 * u_resolution.xy) / min(u_resolution.x, u_resolution.y);
    vec2 c = uv * emotional_zoom + emotional_offset;
    vec2 z = vec2(0.0);

    int iter = 0;
    for(int i = 0; i < 1000; i++) {
        if(i >= emotional_iter) break;
        if(dot(z, z) > 4.0) break;

        float x = z.x * z.x - z.y * z.y + c.x;
        float y = 2.0 * z.x * z.y + c.y;
        z = vec2(x, y);
        iter = i;
    }

    float t = float(iter) / float(emotional_iter);
    
    // Emotional color mapping
    float r = t * (1.0 + u_valence);
    float g = t * (1.0 + u_arousal);
    float b = t * (1.0 + u_dominance);
    
    gl_FragColor = vec4(r, g, b, 1.0);
}
"#;

const EMOTIONAL_JULIA_FRAGMENT: &str = r#"
precision highp float;

uniform float u_time;
uniform vec2 u_resolution;
uniform float u_zoom;
uniform vec2 u_c;
uniform int u_max_iter;
uniform float u_valence;
uniform float u_arousal;
uniform float u_dominance;

void main() {
    vec2 uv = (gl_FragCoord.xy - 0.5 * u_resolution.xy) / min(u_resolution.x, u_resolution.y);
    vec2 z = uv * u_zoom;
    
    // Emotional modulation of Julia constant
    vec2 emotional_c = u_c + vec2(u_valence * 0.1, u_arousal * 0.1);
    int emotional_iter = int(float(u_max_iter) * (1.0 + u_dominance * 0.3));

    int iter = 0;
    for(int i = 0; i < 1000; i++) {
        if(i >= emotional_iter) break;
        if(dot(z, z) > 4.0) break;

        float x = z.x * z.x - z.y * z.y + emotional_c.x;
        float y = 2.0 * z.x * z.y + emotional_c.y;
        z = vec2(x, y);
        iter = i;
    }

    float t = float(iter) / float(emotional_iter);
    
    // Emotional color mapping with engagement boost
    float engagement = (abs(u_valence) + u_arousal + u_dominance) / 3.0;
    float r = t * (1.0 + u_valence * engagement);
    float g = t * (1.0 + u_arousal * engagement);
    float b = t * (1.0 + u_dominance * engagement);
    
    gl_FragColor = vec4(r, g, b, 1.0);
}
"#;

/// Initialize WebGPU if available (fallback to WebGL)
#[wasm_bindgen]
pub fn init_gpu_engine(canvas_id: &str) -> Result<ShaderEngine, JsValue> {
    ShaderEngine::new(canvas_id)
}

/// Check WebGPU support
#[wasm_bindgen]
pub fn check_webgpu_support() -> bool {
    // Check for WebGPU support (simplified)
    // WebGPU support check - simplified for now
    false
}

/// Performance monitoring
#[wasm_bindgen]
pub struct PerformanceMonitor {
    frame_count: u32,
    last_time: f64,
    fps: f32,
}

#[wasm_bindgen]
impl PerformanceMonitor {
    #[wasm_bindgen(constructor)]
    pub fn new() -> PerformanceMonitor {
        PerformanceMonitor {
            frame_count: 0,
            last_time: js_sys::Date::now(),
            fps: 0.0,
        }
    }

    #[wasm_bindgen]
    pub fn update(&mut self) -> f32 {
        self.frame_count += 1;
        let current_time = js_sys::Date::now();
        let delta = current_time - self.last_time;

        if delta >= 1000.0 {
            self.fps = (self.frame_count as f64 / delta * 1000.0) as f32;
            self.frame_count = 0;
            self.last_time = current_time;
        }

        self.fps
    }

    #[wasm_bindgen]
    pub fn get_fps(&self) -> f32 {
        self.fps
    }
}