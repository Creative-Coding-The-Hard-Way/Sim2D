#version 460
#extension GL_EXT_buffer_reference : require

struct Vertex {
  vec4 rgba;
  vec4 position;
};

layout(buffer_reference, std430) readonly buffer VertexBuffer {
    Vertex vertices[];
};

layout(push_constant) uniform constants {
    VertexBuffer vertexBuffer;
} PushConstants;

layout(location = 0) out vec4 vertex_color;

void main() {
    Vertex v = PushConstants.vertexBuffer.vertices[gl_VertexIndex];
    vertex_color = v.rgba;
    gl_Position = vec4(v.position.xy, 0.0, 1.0);
}
