struct VertexOutput {
    @location(0) @interpolate(perspective, sample) uv: vec2<f32>,
    @builtin(position) position: vec4<f32>,
};

@vertex
fn vs_main(@builtin(vertex_index) in_vertex_index: u32) -> VertexOutput {
    var vertices = array<vec4<f32>, 3>(
        vec4<f32>(-1.0, 3.0, 0.0, 1.0),
        vec4<f32>(-1.0, -1.0, 0.0, 1.0),
        vec4<f32>(3.0, -1.0, 0.0, 1.0)
    );
	var result: VertexOutput;
	var vertex = vertices[in_vertex_index];
	result.uv = vec2<f32>((vertex.x+1)/2, (1-vertex.y)/2);
	result.position = vertex;
    return result;
}
