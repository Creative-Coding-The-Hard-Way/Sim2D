#version 460

vec2 vertices[6] = vec2[] (
    // top triangle
    vec2(-0.5, 0.5),  // top left
    vec2(0.5, 0.5),   // top right
    vec2(-0.5, -0.5), // bottom left

    // bottom triangle
    vec2(-0.5, -0.5), // bottom left
    vec2(0.5, 0.5),   // top right
    vec2(0.5, -0.5)   // bottom right
);

vec2 uvs[6] = vec2[] (
    // top triangle
    vec2(0.0, 0.0), // top left
    vec2(1.0, 0.0), // top right
    vec2(0.0, 1.0), // bottom left

    // bottom triangle
    vec2(0.0, 1.0), // bottom left
    vec2(1.0, 0.0), // top right
    vec2(1.0, 1.0)  // bottom right
);

struct SpriteData {
    vec2 pos;
    vec2 size;
    vec4 rgba;
    float texture_id;
    float angle;
    vec2 center_offset;
    vec2 uv_offset;
    vec2 uv_scale;
};

layout(set = 0, binding = 0) readonly buffer SpriteBlock {
  SpriteData sprites[];
};

layout(set = 0, binding = 1) uniform UniformData {
    mat4 projection;
} uniformData;

layout(location = 0) out vec2 uv;
layout(location = 1) out vec4 rgba;
layout(location = 2) flat out int texture_index;

void main() {
    const uint sprite_vertex_count = 6;
    const uint sprite_index = gl_VertexIndex / sprite_vertex_count;
    const uint vertex_index = gl_VertexIndex % 6;

    SpriteData sprite = sprites[sprite_index];

    uv = sprite.uv_offset + (uvs[vertex_index] * sprite.uv_scale);

    rgba = sprite.rgba;
    texture_index = int(sprite.texture_id);

    float c = cos(-sprite.angle);
    float s = sin(-sprite.angle);
    mat2 rotate = mat2(c, -s, s, c);

    vec2 raw_vertex_pos = (vertices[vertex_index] + sprite.center_offset)*sprite.size;
    vec2 vertex_pos = sprite.pos + (rotate*raw_vertex_pos);
    gl_Position =
        uniformData.projection * vec4(vertex_pos.x, vertex_pos.y, 0.0, 1.0);
}
