#version 460

struct Vertex {
  vec4 rgba;
  vec2 position;
};

layout(set = 0, binding = 0) readonly buffer Vertices {
    Vertex vertices[];
};

layout(location = 0) out vec4 vertex_color;

void main() {
    Vertex v = vertices[gl_VertexIndex];
    vertex_color = v.rgba;
    gl_Position = vec4(v.position, 0.0, 1.0);
}
