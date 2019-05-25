#version 460

/// Triangular Pyriamid
/// See: https://en.wikipedia.org/wiki/Tetrahedron
const vec4 tetra_pos[4] = {
    // vec4(-1.,  0., -1. / 1.41421356237, 1.),
    // vec4( 1.,  0., -1. / 1.41421356237, 1.),
    // vec4( 0., -1.,  1. / 1.41421356237, 1.),
    // vec4( 0.,  1.,  1. / 1.41421356237, 1.),
    vec4(-0.5, -0.5, 0., 1.),
    vec4(-0.5,  0.5, 0., 1.),
    vec4( 0.5,  0.5, 0., 1.),
    vec4( 0.5, -0.5, 0., 1.),
};

const vec4 tetra_color[4] = {
    vec4(1., 0., 0., 1.),
    vec4(0., 1., 0., 1.),
    vec4(0., 0., 1., 1.),
    vec4(1., 1., 1., 1.),
};

const vec4 tetra_offsets[4] = {
    vec4(-1., -1., 0., 0.),
    vec4(-1.,  1., 0., 0.),
    vec4( 1., -1., 0., 0.),
    vec4( 1.,  1., 0., 0.),
};

const uint tetra_index[] = {
    0, 1, 2,
    0, 2, 3,
};

const mat4 uProj = {
    vec4(1., 0., 0., 0.),
    vec4(0., 1., 0., 0.),
    vec4(0., 0., 1., 0.),
    vec4(0., 0., 0., 1.)
};

const mat4 uView = {
    vec4(1., 0., 0., 0.),
    vec4(0., 1., 0., 0.),
    vec4(0., 0., 1., 0.),
    vec4(0., 0., 0., 1.)
};

layout(location = 0) out vec4 vColor;

void main() {
    vec4 pos = tetra_pos[tetra_index[gl_VertexID % 6]];
    pos += (vec4(0.25) * tetra_offsets[gl_VertexID / 6]);
    vColor = tetra_color[tetra_index[gl_VertexID % 6]];
    gl_Position = uProj * uView * pos;
}
