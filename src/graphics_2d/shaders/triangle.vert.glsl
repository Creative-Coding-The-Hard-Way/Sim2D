#version 460
#pragma shader_stage(vertex)

// Adds support for buffer references, used for vertex data
#extension GL_EXT_buffer_reference : enable

// Adds support for non-uniform indexing and variable sized descriptor arrays
#extension GL_EXT_nonuniform_qualifier : require

struct Vertex {
    vec2 pos;
    vec2 uv;
    vec4 color;
    int texture_index;
};

// textures bound to set 0
layout(set = 0, binding = 0) uniform sampler u_Sampler;
layout(set = 0, binding = 1) uniform texture2D u_Textures[];

// frame data
layout(set = 1, binding = 0) uniform ubo {
    mat4 projection;
} u_FrameData;

// Push Constants
layout(buffer_reference, std430) readonly buffer VertexBuffer {
    Vertex data[];
};
layout(push_constant) uniform constants {
    VertexBuffer vertices;
} pc_Constants;

// Per-Vertex outputs
layout(location = 0) out vec4 out_VertexColor;
layout(location = 1) out vec2 out_UV;
layout(location = 2) flat out int out_TextureIndex;

void main() {
    Vertex vert = pc_Constants.vertices.data[gl_VertexIndex];
    out_VertexColor = vert.color;
    out_TextureIndex = vert.texture_index;
    out_UV = vert.uv;
    gl_Position =
        u_FrameData.projection * vec4(vert.pos.x, vert.pos.y, 0.0, 1.0);
}
