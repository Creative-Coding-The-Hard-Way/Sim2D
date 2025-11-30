#version 460
#pragma shader_stage(vertex)

struct Vertex {
    vec2 pos;
    vec2 uv;
    vec4 color;
};

layout(location = 0) out vec4 vertexColor;

// frame data
layout(set = 0, binding = 0) uniform UBO {
    mat4 projection;
} ubo;

// Mesh buffers
layout(set = 0, binding = 1) readonly buffer MeshVertices {
    Vertex data[];
} mesh_vertices;

void main() {
    vec2 pos = mesh_vertices.data[gl_VertexIndex].pos;
    vertexColor = mesh_vertices.data[gl_VertexIndex].color;
    gl_Position = ubo.projection * vec4(pos.x, pos.y, 0.0, 1.0);
}
