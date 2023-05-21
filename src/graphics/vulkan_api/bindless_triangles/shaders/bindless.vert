#version 460

struct Vertex{
    vec2 pos;
    vec2 uv;
    vec4 rgba;
    int texture_index;
};

layout(std430, set = 0, binding = 0) readonly buffer Data{
    Vertex vertices[];
} data;

layout(set = 0, binding = 1) uniform UniformData {
    mat4 projection;
} uniformData;

layout(location = 0) out vec2 uv;
layout(location = 1) out vec4 rgba;
layout(location = 2) flat out int texture_index;

void main() {
    Vertex vertex = data.vertices[gl_VertexIndex];

    uv = vertex.uv;
    rgba = vertex.rgba;
    texture_index = vertex.texture_index;

    gl_Position =
        uniformData.projection * vec4(vertex.pos.x, vertex.pos.y, 0.0, 1.0);
}
