#version 460

const float PI  = 3.1415926535897932384626433832795;
const float D2R = PI / 180.0;

#define CIRCLE2(t, r) ((r) * vec2(cos(D2R * (t)), sin(D2R * (t))))

/// ==== Vertex Data
const vec3 c_pos[24] = {
    // +y       Front
    vec3(-1.,  1., -1.),
    vec3(-1.,  1.,  1.),
    vec3( 1.,  1.,  1.),
    vec3( 1.,  1., -1.),

    // +x       Right
    vec3( 1.,  1.,  1.),
    vec3( 1., -1.,  1.),
    vec3( 1., -1., -1.),
    vec3( 1.,  1., -1.),

    // -y       Back
    vec3(-1., -1., -1.),
    vec3( 1., -1., -1.),
    vec3( 1., -1.,  1.),
    vec3(-1., -1.,  1.),

    // -x       Left
    vec3(-1., -1., -1.),
    vec3(-1., -1.,  1.),
    vec3(-1.,  1.,  1.),
    vec3(-1.,  1., -1.),

    // +z       Top
    vec3( 1., -1.,  1.),
    vec3(-1., -1.,  1.),
    vec3(-1.,  1.,  1.),
    vec3( 1.,  1.,  1.),

    // -z       Bottom
    vec3(-1., -1., -1.),
    vec3(-1.,  1., -1.),
    vec3( 1.,  1., -1.),
    vec3( 1., -1., -1.),
};

const uint c_index[36] = {
     0,  1,  2, /**/  0,  2,  3, // Front
     4,  5,  6, /**/  4,  6,  7, // Right
     8,  9, 10, /**/  8, 10, 11, // Back
    12, 13, 14, /**/ 12, 14, 15, // Left
    16, 17, 18, /**/ 16, 18, 19, // Top
    20, 21, 22, /**/ 20, 22, 23, // Bottom
};

/// https://en.wikipedia.org/wiki/Tetrahedron
const vec3 t_pos[4] = {
    vec3( 1,  1,  1), // A
    vec3( 1, -1, -1), // B
    vec3(-1,  1, -1), // C
    vec3(-1, -1,  1), // D
};

const uint t_index[12] = {
    0, 2, 1,
    0, 2, 3,
    0, 3, 1,
    1, 3, 2,
};

/// ==== Shape Data - Scene 0 Chosen One
const vec3 shape_offsets_0[] = {
    vec3(0.),
    vec3(0., 0., 5.),

#define R 20.0
    vec3(CIRCLE2( 1 * 360./23., R), 3),
    vec3(CIRCLE2( 2 * 360./23., R), 3),
    vec3(CIRCLE2( 3 * 360./23., R), 3),
    vec3(CIRCLE2( 4 * 360./23., R), 3),
    vec3(CIRCLE2( 5 * 360./23., R), 3),
    vec3(CIRCLE2( 6 * 360./23., R), 3),
    vec3(CIRCLE2( 7 * 360./23., R), 3),
    vec3(CIRCLE2( 8 * 360./23., R), 3),
    vec3(CIRCLE2( 9 * 360./23., R), 3),
    vec3(CIRCLE2(10 * 360./23., R), 3),
    vec3(CIRCLE2(11 * 360./23., R), 3),
    vec3(CIRCLE2(12 * 360./23., R), 3),
    vec3(CIRCLE2(13 * 360./23., R), 3),
    vec3(CIRCLE2(14 * 360./23., R), 3),
    vec3(CIRCLE2(15 * 360./23., R), 3),
    vec3(CIRCLE2(16 * 360./23., R), 3),
    vec3(CIRCLE2(17 * 360./23., R), 3),
    vec3(CIRCLE2(18 * 360./23., R), 3),
    vec3(CIRCLE2(19 * 360./23., R), 3),
    vec3(CIRCLE2(20 * 360./23., R), 3),
    vec3(CIRCLE2(21 * 360./23., R), 3),
    vec3(CIRCLE2(22 * 360./23., R), 3),
    vec3(CIRCLE2(23 * 360./23., R), 3),
#undef R
};

const vec3 shape_scales_0[] = {
    vec3(30.),
    vec3(7),

    vec3(0.3),
    vec3(0.3),
    vec3(0.3),
    vec3(0.3),
    vec3(0.3),
    vec3(0.3),
    vec3(0.3),
    vec3(0.3),
    vec3(0.3),
    vec3(0.3),
    vec3(0.3),
    vec3(0.3),
    vec3(0.3),
    vec3(0.3),
    vec3(0.3),
    vec3(0.3),
    vec3(0.3),
    vec3(0.3),
    vec3(0.3),
    vec3(0.3),
    vec3(0.3),
    vec3(0.3),
    vec3(0.3),
};

const float shape_rots_0[] = {
    0.,
    90.,

    39.8,
    43.1,
    33.6,
    42.0,
    31.6,
    47.4,
    40.7,
    36.5,
    44.2,
    49.9,
    32.0,
    41.8,
    30.7,
    46.4,
    49.9,
    38.8,
    39.5,
    43.9,
    36.1,
    36.7,
    39.6,
    33.0,
};

/// ==== Shape Data - Scene 9 Testing
const vec3 shape_scales_9[] = {
    // vec3(30.),
    vec3(0.),

    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
    vec3(0.8),
};

const float shape_rots_9[65] = {
    0.,

     00.,
     10.,
     20.,
     30.,
     40.,
     50.,
     60.,
     70.,
     80.,
     90.,
    100.,
    110.,
    120.,
    130.,
    140.,
    150.,
    160.,
    170.,
    180.,
    190.,
    200.,
    210.,
    220.,
    230.,
    240.,
    250.,
    260.,
    270.,
    280.,
    290.,
    300.,
    310.,
    320.,
    330.,
    340.,
    350.,
    360.,
    370.,
    380.,
    390.,
    400.,
    410.,
    420.,
    430.,
    440.,
    450.,
    460.,
    470.,
    480.,
    490.,
    500.,
    510.,
    520.,
    530.,
    540.,
    550.,
    560.,
    570.,
    580.,
    590.,
    600.,
    610.,
    620.,
    630.,
};

layout(location = 0) uniform mat4  uProjView;
layout(location = 1) uniform float uTime;

layout(location = 0) out vec4 vRot;
layout(location = 1) out vec3 vWorldPos;
layout(location = 2) out vec3 vNormal;

#define CALC_NORMAL(OUTPUT, VERT_ID, POSITIONS, INDICES)                       \
    {                                                                          \
        const uint vi0 = (VERT_ID) - ((VERT_ID) % 3);                          \
        const vec3 v0  = POSITIONS[INDICES[(vi0 + 0) % INDICES.length()]];     \
        const vec3 v1  = POSITIONS[INDICES[(vi0 + 1) % INDICES.length()]];     \
        const vec3 v2  = POSITIONS[INDICES[(vi0 + 2) % INDICES.length()]];     \
                                                                               \
        OUTPUT = cross(v1 - v0, v2 - v1);                                      \
    }

vec4 make_rot(vec3 axis, float degrees) {
    const float radians = degrees * PI / 180.;
    const float s = sin(0.5 * radians);
    const float c = cos(0.5 * radians);
    return vec4(s * normalize(axis), c);
}

vec4 qmul(vec4 q1, vec4 q2) {
    return vec4(
        (q2.xyz * q1.w) + (q1.xyz * q2.w) + cross(q1.xyz, q2.xyz),
        (q1.w   * q2.w)                   - dot(q1.xyz, q2.xyz)
    );
}

vec3 rotate_v(vec3 v, vec4 r) {
    vec4 r_c = r * vec4(-1, -1, -1, 1);
    return qmul(r, qmul(vec4(v, 0), r_c)).xyz;
}

vec4 rotate_q(vec4 r0, vec4 r1) {
    vec4 r0_c = r0 * vec4(-1, -1, -1, 1);
    return qmul(r1, qmul(r1, r0_c));
}

bool is_cube_0() {
    return (gl_VertexID / c_index.length()) != 1;
}

bool is_cube_9() {
    return (gl_VertexID / c_index.length()) == 0;
}

void scene_0() {
    uint VERTS_PER_SHAPE = c_index.length();
    uint shape_id = gl_VertexID / VERTS_PER_SHAPE;

    uint index;
    if (is_cube_0()) {
        index = c_index[gl_VertexID % VERTS_PER_SHAPE];
    } else {
        index = t_index[gl_VertexID % VERTS_PER_SHAPE];
    }

    vec3 offset = shape_offsets_0[shape_id];
    vec3 scale  = shape_scales_0[shape_id];

    const mat4 model_offset = mat4(
        vec4(1.,  0.,  0.,  0.),
        vec4(0.,  1.,  0.,  0.),
        vec4(0.,  0.,  1., 0.),
        vec4(      offset,  1.)
    );

    const mat4 model_scale = mat4(
        vec4(scale.x,      0.,         0.,  0.),
        vec4(0.,      scale.y,         0.,  0.),
        vec4(0.,           0.,    scale.z,  0.),
        vec4(0.,           0.,         0.,  1.)
    );

    const mat4 model = model_offset * model_scale;

    vec4 rot;
    if (shape_id > 1) {
        // Rotate about the axis pointing towards the center
        vec4 p4 = (model * vec4(0., 0., 0., 1.));
        vec3 p = p4.xyz / p4.w;
        vec3 axis = normalize(p);
        rot = make_rot(axis, 13. * shape_rots_0[shape_id] * uTime);
    } else if (shape_id == 1) {
        rot = make_rot(vec3(1., 1., 0.), shape_rots_0[shape_id]);
    } else {
        rot = make_rot(vec3(1., 1., 0.), shape_rots_0[0]);
    }

    // Per vertex, regardless of shape
    if (shape_id == 1) {
        vec4 rot2 = make_rot(vec3(0., 0., 1.), -0.05 * shape_rots_0[shape_id] * uTime);
        rot = rotate_q(rot, rot2);
    }
    vRot = rot;

    vec3 v_pos;
    if (is_cube_0()) {
        v_pos = c_pos[index];
    } else {
        v_pos = t_pos[index];
    }
    vec4 pos = vec4(rotate_v(v_pos, rot), 1.);
    gl_Position = uProjView * model * pos;
    vWorldPos = (model * pos).xyz;

    // Per face - each face is defined with 4 index values, 6 faces makes a shape
    vec3 normal = vec3(1., 0., 1.);
    if (is_cube_0()) {
        CALC_NORMAL(normal, gl_VertexID, c_pos, c_index);
    } else {
        CALC_NORMAL(normal, gl_VertexID, t_pos, t_index);
    }
    normal = normalize(normal);
    if (shape_id == 0) {
        normal = -normal;
    }

    vNormal = normal;
}

void scene_9() {
    uint VERTS_PER_SHAPE = c_index.length();
    uint shape_id = gl_VertexID / VERTS_PER_SHAPE;

    uint index;
    if (is_cube_9()) {
        index = c_index[gl_VertexID % VERTS_PER_SHAPE];
    } else {
        index = t_index[gl_VertexID % VERTS_PER_SHAPE];
    }

    vec3 offset = vec3(0.);
    if (shape_id > 0) {
        uint x = (shape_id % 10) - 5;
        uint y = (shape_id / 10);
        float xx = 4. * x + 2 * (y % 2);
        float yy = 4. * y - 10.;
        offset = vec3(xx, 0., yy);
    }
    vec3 scale  = shape_scales_9[shape_id]; //reuse

    const mat4 model_offset = mat4(
        vec4(1.,  0.,  0.,  0.),
        vec4(0.,  1.,  0.,  0.),
        vec4(0.,  0.,  1., 0.),
        vec4(      offset,  1.)
    );

    const mat4 model_scale = mat4(
        vec4(scale.x,      0.,         0.,  0.),
        vec4(0.,      scale.y,         0.,  0.),
        vec4(0.,           0.,    scale.z,  0.),
        vec4(0.,           0.,         0.,  1.)
    );

    const mat4 model = model_offset * model_scale;

    vec4 rot = make_rot(vec3(1., 1., 0.), 0. * shape_rots_9[shape_id]);
    vRot = rot;

    vec3 v_pos;
    if (is_cube_9()) {
        v_pos = c_pos[index];
    } else {
        v_pos = t_pos[index];
    }
    vec4 pos = vec4(rotate_v(v_pos, rot), 1.);
    gl_Position = uProjView * model * pos;
    vWorldPos = (model * pos).xyz;

    // Per face - each face is defined with 4 index values, 6 faces makes a shape
    vec3 normal = vec3(1., 0., 1.);
    if (is_cube_9()) {
        CALC_NORMAL(normal, gl_VertexID, c_pos, c_index);
    } else {
        CALC_NORMAL(normal, gl_VertexID, t_pos, t_index);
    }
    normal = normalize(normal);
    if (shape_id == 0) {
        normal = -normal;
    }

    vNormal = normal;
}

void main() {
    if (uTime < 8.) {
        scene_0();
    } else {
        scene_9();
    }
}
