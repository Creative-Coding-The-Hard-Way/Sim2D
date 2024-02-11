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
    float dt;
    VertexBuffer vertexBuffer;
} PushConstants;

layout(set = 0, binding = 0) uniform UniformBufferObject {
    mat4 transform;
} ubo;

layout(location = 0) out vec4 vertex_color;

void main() {
    Vertex v = PushConstants.vertexBuffer.vertices[gl_VertexIndex];
    vertex_color = v.rgba;
    vec2 p = v.position.xy + PushConstants.dt*v.position.zw;
    gl_Position = ubo.transform * vec4(p, 0.0, 1.0);
    gl_PointSize = 1.0f;
}
