#version 450 core

// Stores the image width and height
struct Globals {
    uint width;
    uint height;
};

layout(binding = 0) uniform Globals_block_0Fragment { Globals globals; };
layout(binding = 1) uniform texture2D color;
layout(binding = 2) uniform sampler colSampler;

// uv is mapped as x horizontal and y vertical with (0.0, 0.0) as top left and
// (1.0, 1.0) as bottom right respectively
sample in vec2 v_uv;
layout(location = 0) out vec4 o_color;

void main() {
	// This example just samples the texture at the current coordinate (basically performing a noop)
    vec3 tex = texture(sampler2D(color, colSampler), vec2(v_uv)).xyz;
    o_color = vec4(tex, 1.0);
    return;
}

// vim: ft=glsl
