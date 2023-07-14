// MIT License
//
// Copyright (c) 2021 TanTanDev
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in all
// copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN THE
// SOFTWARE.

use macroquad::prelude::*;

#[derive(Default)]
pub struct DrawParam {
    pub flip_y: bool,
}

pub struct Transition {
    pub material: Material,
    pub fade: f32,
}

impl Transition {
    pub fn draw(&mut self, base_texture: Texture2D, into_texture: Texture2D, progress: f32) {
        self.draw_ex(base_texture, into_texture, progress, DrawParam::default());
    }

    pub fn draw_ex(
        &mut self,
        base_texture: Texture2D,
        into_texture: Texture2D,
        progress: f32,
        draw_param: DrawParam,
    ) {
        self.material.set_uniform("cutoff", progress);
        self.material.set_uniform("fade", self.fade);
        self.material.set_texture("tex_into", into_texture);
        gl_use_material(&self.material);
        clear_background(WHITE);
        draw_texture_ex(
            &base_texture,
            -1.,
            -1.,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(2., 2.)),
                flip_y: draw_param.flip_y,
                ..Default::default()
            },
        );
        gl_use_default_material();
    }

    pub fn change_transition_tex(&mut self, texture: Texture2D) {
        self.material.set_texture("tex_transition", texture);
    }

    pub fn new(transition_tex: Texture2D, fade: f32) -> Self {
        let pipeline_params = PipelineParams {
            depth_write: true,
            depth_test: Comparison::LessOrEqual,
            ..Default::default()
        };

        let material = load_material(
            ShaderSource {
                glsl_vertex: Some(DEFAULT_VERTEX_SHADER),
                glsl_fragment: Some(DEFAULT_FRAGMENT_SHADER),
                metal_shader: None,
            },
            MaterialParams {
                textures: vec!["tex_transition".to_string(), "tex_into".to_string()],
                uniforms: vec![
                    ("cutoff".to_string(), UniformType::Float1),
                    ("fade".to_string(), UniformType::Float1),
                ],
                pipeline_params,
            },
        )
        .unwrap();

        material.set_texture("tex_transition", transition_tex);
        Transition { material, fade }
    }
}

const DEFAULT_FRAGMENT_SHADER: &str = "#version 100
    precision lowp float;
    varying vec2 uv;

    uniform float cutoff;
    uniform float fade;
    // base texture
    uniform sampler2D Texture;
    uniform sampler2D tex_into;
    uniform sampler2D tex_transition;

    varying vec4 color;

    void main() {
        float transition = texture2D(tex_transition, uv).r;
        vec4 base_color = texture2D(Texture, uv);
        vec4 into_color = texture2D(tex_into, uv);

        // remap transition from 0-1 to fade -> 1.0-fade
        transition = transition * (1.0 - fade) + fade;
        float f = smoothstep(cutoff, cutoff + fade, transition);
        gl_FragColor = mix(base_color, into_color, f); 
    }
";

const DEFAULT_VERTEX_SHADER: &str = "#version 100
    attribute vec3 position;
    attribute vec2 texcoord;
    varying vec2 uv;

    void main() {
        gl_Position = vec4(position, 1);
        uv = texcoord;
    }
";
