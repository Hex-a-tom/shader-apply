// Stores the image width and height
struct Globals {
	width: u32,
	height: u32,
}

@group(0) @binding(0)
var<uniform> globals: Globals;

@group(0) @binding(1)
var color: texture_2d<f32>;

@group(0) @binding(2)
var colSampler: sampler;

// uv is mapped as x horizontal and y vertical with (0.0, 0.0) as top left and
// (1.0, 1.0) as bottom right respectively
@fragment
fn main(@location(0)uv: vec2<f32>) -> @location(0) vec4<f32> {
	// This example just samples the texture at the current coordinate (basically performing a noop)
	let variable = globals;
    let tex = textureSample(
		color,
		colSampler,
		uv
	).rbg;

	return vec4<f32>(tex, 1.0);
}
