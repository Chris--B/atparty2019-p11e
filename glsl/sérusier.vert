#version 460

/// Triangular Pyriamid
/// See: https://en.wikipedia.org/wiki/Tetrahedron
const vec3 tetra_pos[4] = {
    // vec4(-1.,  0., -1. / 1.41421356237, 1.),
    // vec4( 1.,  0., -1. / 1.41421356237, 1.),
    // vec4( 0., -1.,  1. / 1.41421356237, 1.),
    // vec4( 0.,  1.,  1. / 1.41421356237, 1.),
    vec3(-0.5, -0.5, 0.),
    vec3(-0.5,  0.5, 0.),
    vec3( 0.5,  0.5, 0.),
    vec3( 0.5, -0.5, 0.),
};

const vec4 shape_colors[] = {
    vec4(0., 0., 1., 1.),
    vec4(1., 0., 0., 1.),
    vec4(0., 1., 0., 1.),
    vec4(1., 1., 1., 1.),
    vec4(0x18, 0x27, 0x90, 255.) / 255.,
    vec4(0x2b, 0x78, 0xb8, 255.) / 255.,
    vec4(0x36, 0x7b, 0xa0, 255.) / 255.,
    vec4(0x53, 0x49, 0x9e, 255.) / 255.,
    vec4(0xef, 0x17, 0x4e, 255.) / 255.,
};

const vec3 shape_offets[4] = {
    vec3(-1., -1.,  2.0),
    vec3(-1.,  1.,  1.0),
    vec3( 1., -1., -1.0),
    vec3( 1.,  1., -2.0),
};

const uint tetra_index[] = {
    0, 1, 2,
    0, 2, 3,
};

// layout(location = 0) uniform mat4 uProj;

layout(location = 0) out vec4 vColor;

// https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluLookAt.xml
mat4 look_at(vec3 eye, vec3 center, vec3 up) {
    vec3 forward = normalize(center - eye);
    vec3 side    = normalize(cross(forward, up));
    vec3 u       = normalize(cross(side, forward));

    mat4 map = mat4(
        vec4(side,     0.),
        vec4(u,        0.),
        vec4(-forward, 0.),
        vec4(vec3(0.), 1.)
    );
    mat4 eye_shift = mat4(
        vec4(1., 0., 0., eye.x),
        vec4(0., 1., 0., eye.y),
        vec4(0., 0., 1., eye.z),
        vec4(0., 0., 0., 1.)
    );

    return eye_shift * map;
}

// https://www.khronos.org/registry/OpenGL-Refpages/gl2.1/xhtml/gluPerspective.xml
mat4 perspective(float fovy, float aspect, float zNear, float zFar) {
    float f = 1. / tan(0.5 * fovy);
    return mat4(
        vec4(f / aspect, 0.,                              0.,                                   0.),
        vec4(        0., f,                               0.,                                   0.),
        vec4(        0., 0., (zFar + zNear) / (zNear - zFar), (2. * zFar * zNear) / (zNear - zFar)),
        vec4(        0., 0.,                             -1.,                                   0.)
    );
}

void main() {
    // Per vertex, regardless of shape
    const uint vert_id  = tetra_index[gl_VertexID % 6];

    // Per shape, shared by all vertices
    const uint shape_id = gl_VertexID / 6;

    mat4 proj = perspective(
        90., // fov
        1.0, // aspect
        0.1, // zNear
        20.  // zFar
    );

    // mat4 view = look_at(
    //     vec3(10., 10., 10.),
    //     vec3( 0.,  0.,  0.),
    //     vec3( 0.,  0.,  1.)
    // );

    mat4 view = look_at(
        vec3( 0.,  0.,  1.), // eye
        vec3( 0.,  0.,  0.), // lookat center
        vec3( 0.,  1.,  0.)  // up
    );

    vec3 offset = shape_offets[shape_id];
    offset   *= 0.5;
    offset.z *= 0.5;
    mat4 model_offset = mat4(
        vec4(1., 0., 0., 0.),
        vec4(0., 1., 0., 0.),
        vec4(0., 0., 1., 0.),
        vec4(offset,     1.)
    );

    float scale = 1. / 2.;
    mat4 model_scale = mat4(
        vec4(scale, 0.,    0.,   0.),
        vec4(0.,    scale, 0.,   0.),
        vec4(0.,    0., scale,   0.),
        vec4(0.,    0.,    0.,   1.)
    );

    const mat4 model = mat4(1.)
        * model_offset
        * model_scale
        ;

    const vec4 pos = vec4(tetra_pos[vert_id], 1.0);
    // gl_Position = model * view * proj * pos;
    gl_Position = view * model * pos;

    // Per shape
    vColor = shape_colors[shape_id];
}
