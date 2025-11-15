//! WebGPU/WebGL shader engine for browser-based creative tools
//!
//! Enhanced with emotional computing integration and advanced rendering capabilities.

use wasm_bindgen::prelude::*;
use web_sys::{WebGlRenderingContext, WebGlShader, WebGlProgram};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use chrono::{DateTime, Utc};

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
    // Add emotional computing integration
    emotional_state: Option<EmotionalVector>,
    emotional_modulation_enabled: bool,
    // Enhanced fields
    emotional_history: Vec<EmotionalVector>,
    emotional_complexity: f32,
    creativity_index: f32,
}

/// Enhanced emotional vector for creative modulation
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmotionalVector {
    pub valence: f32,     // -1.0 to 1.0 (negative to positive emotions)
    pub arousal: f32,     // 0.0 to 1.0 (calm to excited)
    pub dominance: f32,   // 0.0 to 1.0 (submissive to dominant)
    pub confidence: f32,  // Confidence in emotional assessment (0 to 1)
    pub timestamp: DateTime<Utc>,   // When emotional data was captured
    // Enhanced fields
    pub emotional_category: String, // Human-readable emotional category
    pub emotional_trajectory: Vec<EmotionalPoint>, // Historical emotional path
    pub predicted_emotion: Option<Box<EmotionalVector>>, // Predicted next emotional state
    pub emotional_complexity: f32, // Complexity of emotional journey
}

/// Point in emotional trajectory
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct EmotionalPoint {
    pub valence: f32,
    pub arousal: f32,
    pub timestamp: DateTime<Utc>,
}

impl EmotionalVector {
    /// Create new emotional vector with enhanced fields
    pub fn new(valence: f32, arousal: f32, dominance: f32) -> Self {
        let timestamp = Utc::now();
        let category = Self::get_emotional_category(valence, arousal);
        
        Self {
            valence: valence.clamp(-1.0, 1.0),
            arousal: arousal.clamp(0.0, 1.0),
            dominance: dominance.clamp(0.0, 1.0),
            confidence: 0.8, // Default confidence
            timestamp,
            emotional_category: category,
            emotional_trajectory: vec![],
            predicted_emotion: None,
            emotional_complexity: 0.0,
        }
    }
    
    /// Get human-readable emotional category
    pub fn get_emotional_category(valence: f32, arousal: f32) -> String {
        match (valence, arousal) {
            (v, a) if v > 0.5 && a > 0.5 => "Excited".to_string(),
            (v, a) if v > 0.5 && a <= 0.5 => "Happy".to_string(),
            (v, a) if v <= 0.5 && a > 0.5 => "Anxious".to_string(),
            _ => "Calm".to_string(),
        }
    }
    
    /// Add point to emotional trajectory
    pub fn add_trajectory_point(&mut self, valence: f32, arousal: f32) {
        self.emotional_trajectory.push(EmotionalPoint {
            valence,
            arousal,
            timestamp: Utc::now(),
        });
    }
    
    /// Calculate emotional complexity based on trajectory
    pub fn calculate_complexity(&mut self) {
        if self.emotional_trajectory.len() < 2 {
            self.emotional_complexity = 0.0;
            return;
        }
        
        let mut total_distance = 0.0;
        for i in 1..self.emotional_trajectory.len() {
            let prev = &self.emotional_trajectory[i-1];
            let curr = &self.emotional_trajectory[i];
            let distance = ((curr.valence - prev.valence).powi(2) + 
                           (curr.arousal - prev.arousal).powi(2)).sqrt();
            total_distance += distance;
        }
        
        // Normalize by number of points
        self.emotional_complexity = (total_distance / self.emotional_trajectory.len() as f32).clamp(0.0, 1.0);
    }
    
    /// Predict next emotional state
    pub fn predict_next_emotion(&self) -> Option<EmotionalVector> {
        if self.emotional_trajectory.len() < 3 {
            return None;
        }
        
        let len = self.emotional_trajectory.len();
        let latest = &self.emotional_trajectory[len - 1];
        let previous = &self.emotional_trajectory[len - 2];
        let older = &self.emotional_trajectory[len - 3];
        
        // Simple linear extrapolation
        let valence_delta = (latest.valence - previous.valence) * 0.7 + (previous.valence - older.valence) * 0.3;
        let arousal_delta = (latest.arousal - previous.arousal) * 0.7 + (previous.arousal - older.arousal) * 0.3;
        
        Some(EmotionalVector {
            valence: (latest.valence + valence_delta).clamp(-1.0, 1.0),
            arousal: (latest.arousal + arousal_delta).clamp(0.0, 1.0),
            dominance: self.dominance,
            confidence: (self.confidence - 0.1).max(0.0), // Confidence decreases with prediction
            timestamp: Utc::now(),
            emotional_category: EmotionalVector::get_emotional_category(latest.valence + valence_delta, latest.arousal + arousal_delta),
            emotional_trajectory: self.emotional_trajectory.clone(),
            predicted_emotion: None, // Would need recursive handling in a real implementation
            emotional_complexity: self.emotional_complexity,
        })
    }
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
        let _ = gl.get_extension("OES_texture_float");
        let _ = gl.get_extension("OES_standard_derivatives");
        let _ = gl.get_extension("EXT_shader_texture_lod");

        Ok(ShaderEngine {
            canvas,
            gl,
            programs: HashMap::new(),
            current_program: None,
            uniforms: HashMap::new(),
            time: 0.0,
            resolution: [800.0, 600.0],
            emotional_state: None,
            emotional_modulation_enabled: false,
            emotional_history: vec![],
            emotional_complexity: 0.0,
            creativity_index: 0.0,
        })
    }

    /// Compile and link shader program
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
        Ok(())
    }

    /// Use shader program
    #[wasm_bindgen]
    pub fn use_program(&mut self, name: &str) -> Result<(), JsValue> {
        if let Some(program) = self.programs.get(name) {
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

        // Update emotional uniforms if enabled
        // Clone the emotional state values to avoid borrowing conflicts
        let emotional_values = if self.emotional_modulation_enabled {
            self.emotional_state.as_ref().map(|emotion| {
                (emotion.valence, emotion.arousal, emotion.dominance, emotion.confidence, emotion.emotional_complexity)
            })
        } else {
            None
        };

        if let Some((valence, arousal, dominance, confidence, complexity)) = emotional_values {
            self.set_uniform("u_emotion_valence", JsValue::from(valence))?;
            self.set_uniform("u_emotion_arousal", JsValue::from(arousal))?;
            self.set_uniform("u_emotion_dominance", JsValue::from(dominance))?;
            self.set_uniform("u_emotion_confidence", JsValue::from(confidence))?;
            self.set_uniform("u_emotion_complexity", JsValue::from(complexity))?;
        }

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

    /// Set emotional state for modulation
    #[wasm_bindgen]
    pub fn set_emotional_state(&mut self, valence: f32, arousal: f32, dominance: f32) {
        let mut emotional_vector = EmotionalVector::new(valence, arousal, dominance);
        
        // Add to history
        self.emotional_history.push(emotional_vector.clone());
        
        // Keep only the last 100 emotional states
        if self.emotional_history.len() > 100 {
            self.emotional_history.remove(0);
        }
        
        // Update complexity
        emotional_vector.add_trajectory_point(valence, arousal);
        emotional_vector.calculate_complexity();
        self.emotional_complexity = emotional_vector.emotional_complexity;
        
        // Update creativity index based on emotional diversity
        self.update_creativity_index();
        
        self.emotional_state = Some(emotional_vector);
    }
    
    /// Update creativity index based on emotional history
    fn update_creativity_index(&mut self) {
        if self.emotional_history.len() < 2 {
            self.creativity_index = 0.0;
            return;
        }
        
        // Calculate variance in emotional dimensions
        let len = self.emotional_history.len() as f32;
        let avg_valence: f32 = self.emotional_history.iter().map(|e| e.valence).sum::<f32>() / len;
        let avg_arousal: f32 = self.emotional_history.iter().map(|e| e.arousal).sum::<f32>() / len;
        
        let valence_variance: f32 = self.emotional_history.iter().map(|e| (e.valence - avg_valence).powi(2)).sum::<f32>() / len;
        let arousal_variance: f32 = self.emotional_history.iter().map(|e| (e.arousal - avg_arousal).powi(2)).sum::<f32>() / len;
        
        // Creativity index is higher when there's more variation
        let variance = (valence_variance + arousal_variance).sqrt();
        self.creativity_index = variance.clamp(0.0, 1.0);
    }

    /// Enable/disable emotional modulation
    #[wasm_bindgen]
    pub fn set_emotional_modulation(&mut self, enabled: bool) {
        self.emotional_modulation_enabled = enabled;
    }

    /// Get current emotional state
    #[wasm_bindgen]
    pub fn get_emotional_state(&self) -> Option<JsValue> {
        if let Some(emotion) = &self.emotional_state {
            let obj = js_sys::Object::new();
            let _ = js_sys::Reflect::set(&obj, &"valence".into(), &JsValue::from(emotion.valence));
            let _ = js_sys::Reflect::set(&obj, &"arousal".into(), &JsValue::from(emotion.arousal));
            let _ = js_sys::Reflect::set(&obj, &"dominance".into(), &JsValue::from(emotion.dominance));
            let _ = js_sys::Reflect::set(&obj, &"confidence".into(), &JsValue::from(emotion.confidence));
            let _ = js_sys::Reflect::set(&obj, &"emotional_category".into(), &JsValue::from(&emotion.emotional_category));
            let _ = js_sys::Reflect::set(&obj, &"emotional_complexity".into(), &JsValue::from(emotion.emotional_complexity));
            Some(JsValue::from(obj))
        } else {
            None
        }
    }
    
    /// Get emotional history
    #[wasm_bindgen]
    pub fn get_emotional_history(&self) -> JsValue {
        let arr = js_sys::Array::new();
        for emotion in &self.emotional_history {
            let obj = js_sys::Object::new();
            let _ = js_sys::Reflect::set(&obj, &"valence".into(), &JsValue::from(emotion.valence));
            let _ = js_sys::Reflect::set(&obj, &"arousal".into(), &JsValue::from(emotion.arousal));
            let _ = js_sys::Reflect::set(&obj, &"dominance".into(), &JsValue::from(emotion.dominance));
            let _ = js_sys::Reflect::set(&obj, &"timestamp".into(), &JsValue::from(emotion.timestamp.timestamp()));
            arr.push(&obj);
        }
        JsValue::from(arr)
    }
    
    /// Get emotional complexity
    #[wasm_bindgen]
    pub fn get_emotional_complexity(&self) -> f32 {
        self.emotional_complexity
    }
    
    /// Get creativity index
    #[wasm_bindgen]
    pub fn get_creativity_index(&self) -> f32 {
        self.creativity_index
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

        float x = z.x * z.x - z.y * z.y + c.x;
        float y = 2.0 * abs(z.x * z.y) + c.y;
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
uniform vec2 u_offset;
uniform int u_max_iter;
uniform vec3 u_color1;
uniform vec3 u_color2;

void main() {
    vec2 uv = (gl_FragCoord.xy - 0.5 * u_resolution.xy) / min(u_resolution.x, u_resolution.y);
    vec2 z = uv * u_zoom + u_offset;

    int iter = 0;
    for(int i = 0; i < 1000; i++) {
        if(i >= u_max_iter) break;
        
        // Newton's method for f(z) = z^3 - 1
        vec2 z2 = vec2(z.x * z.x - z.y * z.y, 2.0 * z.x * z.y);
        vec2 z3 = vec2(z2.x * z.x - z2.y * z.y, z2.x * z.y + z2.y * z.x);
        vec2 fz = vec2(z3.x - 1.0, z3.y);
        
        // f'(z) = 3z^2
        vec2 dfz = vec2(3.0 * z2.x, 3.0 * z2.y);
        
        // Avoid division by zero
        float denom = dfz.x * dfz.x + dfz.y * dfz.y;
        if(denom < 0.0001) break;
        
        vec2 new_z = vec2(
            z.x - (fz.x * dfz.x + fz.y * dfz.y) / denom,
            z.y - (fz.y * dfz.x - fz.x * dfz.y) / denom
        );
        
        if(distance(z, new_z) < 0.0001) {
            iter = i;
            break;
        }
        
        z = new_z;
    }

    float t = float(iter) / float(u_max_iter);
    vec3 color = mix(u_color1, u_color2, t);
    gl_FragColor = vec4(color, 1.0);
}
"#;

const PHOENIX_FRAGMENT: &str = r#"
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
    vec2 z1 = vec2(0.0);

    int iter = 0;
    for(int i = 0; i < 1000; i++) {
        if(i >= u_max_iter) break;
        if(dot(z, z) > 4.0) break;

        float x = z.x * z.x - z.y * z.y + c.x + 0.56667 * z1.x;
        float y = 2.0 * z.x * z.y + c.y - 0.5 * z1.y;
        z1 = z;
        z = vec2(x, y);
        iter = i;
    }

    float t = float(iter) / float(u_max_iter);
    vec3 color = mix(u_color1, u_color2, t);
    gl_FragColor = vec4(color, 1.0);
}
"#;

// Enhanced emotional fractal shader
const EMOTIONAL_MANDELBROT_FRAGMENT: &str = r#"
precision highp float;

uniform float u_time;
uniform vec2 u_resolution;
uniform float u_zoom;
uniform vec2 u_offset;
uniform int u_max_iter;
uniform vec3 u_color1;
uniform vec3 u_color2;
// Emotional uniforms
uniform float u_emotion_valence;
uniform float u_emotion_arousal;
uniform float u_emotion_dominance;
uniform float u_emotion_confidence;
uniform float u_emotion_complexity;

void main() {
    vec2 uv = (gl_FragCoord.xy - 0.5 * u_resolution.xy) / min(u_resolution.x, u_resolution.y);
    vec2 c = uv * u_zoom + u_offset;
    
    // Modulate based on emotional state
    c.x += u_emotion_valence * 0.1 * u_emotion_confidence;
    c.y += u_emotion_arousal * 0.05 * u_emotion_dominance;
    
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
    
    // Emotional color modulation
    vec3 base_color = mix(u_color1, u_color2, t);
    vec3 emotion_color = vec3(
        abs(u_emotion_valence) * u_emotion_confidence,
        u_emotion_arousal * u_emotion_dominance,
        u_emotion_complexity
    );
    
    // Blend based on emotional intensity
    float emotional_blend = 0.3 * length(vec2(u_emotion_valence, u_emotion_arousal));
    vec3 final_color = mix(base_color, emotion_color, emotional_blend);
    gl_FragColor = vec4(final_color, 1.0);
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

/// Utility function to create emotional vector
#[wasm_bindgen]
pub fn create_emotional_vector(valence: f32, arousal: f32, dominance: f32) -> JsValue {
    let mut emotion = EmotionalVector::new(valence, arousal, dominance);
    
    // Add to trajectory
    emotion.add_trajectory_point(valence, arousal);
    emotion.calculate_complexity();
    
    let obj = js_sys::Object::new();
    let _ = js_sys::Reflect::set(&obj, &"valence".into(), &JsValue::from(emotion.valence));
    let _ = js_sys::Reflect::set(&obj, &"arousal".into(), &JsValue::from(emotion.arousal));
    let _ = js_sys::Reflect::set(&obj, &"dominance".into(), &JsValue::from(emotion.dominance));
    let _ = js_sys::Reflect::set(&obj, &"confidence".into(), &JsValue::from(emotion.confidence));
    let _ = js_sys::Reflect::set(&obj, &"emotional_category".into(), &JsValue::from(&emotion.emotional_category));
    let _ = js_sys::Reflect::set(&obj, &"emotional_complexity".into(), &JsValue::from(emotion.emotional_complexity));
    
    JsValue::from(obj)
}

/// Utility function to get emotional category
#[wasm_bindgen]
pub fn get_emotional_category(valence: f32, arousal: f32, dominance: f32) -> String {
    EmotionalVector::get_emotional_category(valence, arousal)
}