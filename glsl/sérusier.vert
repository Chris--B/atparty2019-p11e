#version 460

/// Triangular Pyriamid
/// See: https://en.wikipedia.org/wiki/Tetrahedron
const vec3 tetra_pos[] = {
    // vec4(-1.,  0., -1. / 1.41421356237, 1.),
    // vec4( 1.,  0., -1. / 1.41421356237, 1.),
    // vec4( 0., -1.,  1. / 1.41421356237, 1.),
    // vec4( 0.,  1.,  1. / 1.41421356237, 1.),

    // Front
    vec3(-1.0,  1.0, -5.0),
    vec3( 1.0,  1.0, -5.0),
    vec3( 1.0,  1.0,  5.0),
    vec3(-1.0,  1.0,  5.0),

    // Right
    vec3( 1.0,  1.0,  5.0),
    vec3( 1.0, -1.0,  5.0),
    vec3( 1.0, -1.0, -5.0),
    vec3( 1.0,  1.0, -5.0),

    // Back
    vec3(-1.0, -1.0, -5.0),
    vec3( 1.0, -1.0, -5.0),
    vec3( 1.0, -1.0,  5.0),
    vec3(-1.0, -1.0,  5.0),

    // Left
    vec3(-1.0, -1.0, -5.0),
    vec3(-1.0,  1.0, -5.0),
    vec3(-1.0,  1.0,  5.0),
    vec3(-1.0, -1.0,  5.0),

    // Top
    vec3( 1.0,  1.0,  5.0),
    vec3(-1.0,  1.0,  5.0),
    vec3(-1.0, -1.0,  5.0),
    vec3( 1.0, -1.0,  5.0),

    // Bottom
    vec3(-1.0, -1.0, -5.0),
    vec3( 1.0, -1.0, -5.0),
    vec3( 1.0,  1.0, -5.0),
    vec3(-1.0,  1.0, -5.0),
};

const vec3 vert_normals[] = {
    // Front
    vec3( 0.,  1.,  0.),

    // Right
    vec3( 1.,  0.,  0.),

    // Back
    vec3( 0., -1.,  0.),

    // Left
    vec3(-1.,  0.,  0.),

    // Top
    vec3( 0.,  0.,  1.),

    // Bottom
    vec3( 0.,  0., -1.),
};

const vec3 shape_colors[] = {
    vec3(0., 0., 1.),
    vec3(1., 0., 0.),
    vec3(0., 1., 0.),
    vec3(1., 1., 1.),
};

const vec3 shape_offets[4] = {
    vec3(-1., -1.,  0.5),
    vec3(-1.,  1., -0.5),
    vec3( 1., -1.,  0.5),
    vec3( 1.,  1., -0.5),
};

const float shape_time_scales[] = {
    0., // 1.,
    0., // 3.,
    0., // 5.,
    0., // 7.,
};

const uint vert_indices[] = {
     0,  1,  2, /**/  0,  2,  3, // Front
     4,  5,  6, /**/  4,  6,  7, // Right
     8,  9, 10, /**/  8, 10, 11, // Back
    12, 13, 14, /**/ 12, 14, 15, // Left
    16, 17, 18, /**/ 16, 18, 19, // Top
    20, 21, 22, /**/ 20, 22, 23, // Bottom
};


layout(location = 0) uniform mat4  uProjView;
layout(location = 1) uniform float uTime;

layout(location = 0) out vec3 vColor;
layout(location = 1) out vec3 vWorldPos;
layout(location = 2) out vec3 vNormal;

void main() {
    const uint VERTS_PER_SHAPE = 36;

    // Per vertex, regardless of shape
    const uint vert_id  = vert_indices[gl_VertexID % VERTS_PER_SHAPE];

    // Per shape, shared by all vertices
    const uint shape_id = gl_VertexID / VERTS_PER_SHAPE;

    vec3 offset = shape_offets[shape_id];
    offset *= 7.0;
    offset.z = 5. * sin(2. * uTime + shape_id);
    mat4 model_offset = mat4(
        vec4(1., 0., 0., 0.),
        vec4(0., 1., 0., 0.),
        vec4(0., 0., 1., 0.),
        vec4(offset,     1.)
    );

    float t = shape_time_scales[shape_id] * uTime;
    float scale = 0.5 + mix(0.5, 1.5, sin(t) + 1.0);
    mat4 model_scale = mat4(
        vec4(scale,    0.,    0.,  0.),
        vec4(0.,    scale,    0.,  0.),
        vec4(0.,       0., scale,  0.),
        vec4(0.,       0.,    0.,  1.)
    );

    const mat4 model = model_offset * model_scale;

    // Per vertex
    const vec4 pos = vec4(tetra_pos[vert_id], 1.0);
    gl_Position = uProjView * model * pos;
    vWorldPos = (model * pos).xyz;

    // Per face - each face is defined with 4 index values, 6 faces makes a shape
    vNormal = (model * vec4(vert_normals[(vert_id / 4) % 6], 0.)).xyz;

    // Per shape
    // vColor = shape_colors[(gl_VertexID / 3) % 4];
    vColor = vec3(0.8, 0.8, 0.8);
}
