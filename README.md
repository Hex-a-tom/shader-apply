# Shader-Apply
Shader-apply is a utility for applying wgsl and glsl fragment shaders to images. Shader-apply either accepts an image in the formats PNG, JPG, AVIF, GIF, EXR, TIFF, WebP and BMP or a blank canvas of custom dimensions.

## Usage
```
Usage: shader-apply <--input <INPUT>|--blank <DIMENSIONS>> <SHADER> [OUTPUT]

Arguments:
  <SHADER>  The wgsl or glsl shader to be used
  [OUTPUT]  The output location of the image [default: output.png]

Options:
  -i, --input <INPUT>       Image for the shader to be applied on
      --blank <DIMENSIONS>  Accepts dimensions in the format: [width]x[height] (ex: 512x256)
  -h, --help                Print help
  -V, --version             Print version
```

## Examples

### Wgsl example

```wgsl
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

// uv is mapped as x horizontal any y vertical with (0.0, 0.0) as top left and
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
```

Run this shader with:
```sh
shader-apply --blank 256x256 shader.wgsl
```

and it will produce a completely transparent output.png file with the dimensions 256x256.


### Glsl example
(Glsl files need to use a version of 440 or newer.)
```glsl
#version 450 core

// Stores the image width and height
struct Globals {
    uint width;
    uint height;
};

layout(binding = 0) uniform Globals_block_0Fragment { Globals globals; };
layout(binding = 1) uniform texture2D color;
layout(binding = 2) uniform sampler colSampler;

// uv is mapped as x horizontal any y vertical with (0.0, 0.0) as top left and
// (1.0, 1.0) as bottom right respectively
sample in vec2 v_uv;
layout(location = 0) out vec4 o_color;

void main() {
	// This example just samples the texture at the current coordinate (basically performing a noop)
    vec3 tex = texture(sampler2D(color, colSampler), vec2(v_uv)).xyz;
    o_color = vec4(tex, 1.0);
    return;
}
```

Run this shader with:
```sh
shader-apply --blank 256x256 shader.frag
```
<sub>(the file ending needs to be .frag)<sub>

and it will produce a completely transparent output.png file with the dimensions 256x256.
