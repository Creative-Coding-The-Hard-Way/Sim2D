#version 460
#pragma shader_stage(vertex)
#extension GL_EXT_buffer_reference : enable

struct Vertex {
    vec2 pos;
    vec2 uv;
    vec4 color;
};

// frame data
layout(set = 0, binding = 0) uniform UBO {
    mat4 projection;
} ubo;

layout(buffer_reference, std430) readonly buffer VertexBuffer {
    Vertex data[];
};

layout(push_constant) uniform constants {
    VertexBuffer vertices;
} PushConstants;

// Per-Vertex outputs
layout(location = 0) out vec4 vertexColor;

void main() {
    vec2 pos = PushConstants.vertices.data[gl_VertexIndex].pos;
    vertexColor = PushConstants.vertices.data[gl_VertexIndex].color;
    gl_Position = ubo.projection * vec4(pos.x, pos.y, 0.0, 1.0);
}
