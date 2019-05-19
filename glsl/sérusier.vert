#version 460

/// Triangular Pyriamid
/// See: https://en.wikipedia.org/wiki/Tetrahedron
const vec4 tetra_pos[4] = {
    vec4(-1., 0., -1. / 1.41421356237, 1.),
    vec4(1., 0., -1. / 1.41421356237, 1.),
    vec4(-1., 0., 1. / 1.41421356237, 1.),
    vec4(1., 0., 1. / 1.41421356237, 1.),
};

void main() {
    vec4 pos = tetra_pos[gl_VertexID % 4];
    gl_Position = pos;
}
