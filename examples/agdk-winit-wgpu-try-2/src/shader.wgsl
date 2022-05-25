struct VertexInput {
	[[location(0)]] position: vec2<f32>;
};

struct VertexOutput {
	[[builtin(position)]] clip_position: vec4<f32>;
	[[location(1)]] uv: vec2<f32>;
};

struct CameraUniform {
	view_proj: mat4x4<f32>;
	eye: vec3<f32>;
};

[[group(0), binding(0)]]
var<uniform> camera: CameraUniform;

struct Uniforms {
	time: f32;
};

[[group(1), binding(0)]]
var<uniform> uniforms: Uniforms;



[[stage(vertex)]]
fn vs_main(model: VertexInput) -> VertexOutput {
	var out: VertexOutput;

	out.clip_position = vec4<f32>(model.position.x, model.position.y, 0.0, 1.0);
	out.uv = model.position;

	return out;
}



[[stage(fragment)]]
fn fs_main(in: VertexOutput) -> [[location(0)]] vec4<f32> {
	// return vec4<f32>(in.uv.x, in.uv.y, 0.0, 1.0);

	var u_time: f32 = uniforms.time;
	return vec4<f32>(sin(u_time), 0.0, 0.0, 1.0);
}