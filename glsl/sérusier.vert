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

    vec3(7.),
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
    47.0,
    44.8,
    46.6,
    33.0,
    34.3,
    46.2,
    40.2,
    42.7,
    43.3,
    34.7,
};

/// ==== Shape Data - Scene 9 Testing
const vec3 shape_offsets_9[65] = {
    // Skybox - Don't move it
    vec3( 0.0,  0.0,  0.0),

    // Everything else - 64 of them
    vec3(-2, -2, -2),
    vec3(-2, -2, -1),
    vec3(-2, -2, 0),
    vec3(-2, -2, 1),
    vec3(-2, -1, -2),
    vec3(-2, -1, -1),
    vec3(-2, -1, 0),
    vec3(-2, -1, 1),
    vec3(-2, 0, -2),
    vec3(-2, 0, -1),
    vec3(-2, 0, 0),
    vec3(-2, 0, 1),
    vec3(-2, 1, -2),
    vec3(-2, 1, -1),
    vec3(-2, 1, 0),
    vec3(-2, 1, 1),
    vec3(-1, -2, -2),
    vec3(-1, -2, -1),
    vec3(-1, -2, 0),
    vec3(-1, -2, 1),
    vec3(-1, -1, -2),
    vec3(-1, -1, -1),
    vec3(-1, -1, 0),
    vec3(-1, -1, 1),
    vec3(-1, 0, -2),
    vec3(-1, 0, -1),
    vec3(-1, 0, 0),
    vec3(-1, 0, 1),
    vec3(-1, 1, -2),
    vec3(-1, 1, -1),
    vec3(-1, 1, 0),
    vec3(-1, 1, 1),
    vec3(0, -2, -2),
    vec3(0, -2, -1),
    vec3(0, -2, 0),
    vec3(0, -2, 1),
    vec3(0, -1, -2),
    vec3(0, -1, -1),
    vec3(0, -1, 0),
    vec3(0, -1, 1),
    vec3(0, 0, -2),
    vec3(0, 0, -1),
    vec3(0, 0, 0),
    vec3(0, 0, 1),
    vec3(0, 1, -2),
    vec3(0, 1, -1),
    vec3(0, 1, 0),
    vec3(0, 1, 1),
    vec3(1, -2, -2),
    vec3(1, -2, -1),
    vec3(1, -2, 0),
    vec3(1, -2, 1),
    vec3(1, -1, -2),
    vec3(1, -1, -1),
    vec3(1, -1, 0),
    vec3(1, -1, 1),
    vec3(1, 0, -2),
    vec3(1, 0, -1),
    vec3(1, 0, 0),
    vec3(1, 0, 1),
    vec3(1, 1, -2),
    vec3(1, 1, -1),
    vec3(1, 1, 0),
    vec3(1, 1, 1),

    // "Random"
    // vec3(0.7191511282382121, 0.47436989500908855, 0.7451361058713958),
    // vec3(0.47436989500908855, 0.7451361058713958, 0.703488576674201),
    // vec3(0.7451361058713958, 0.703488576674201, 0.9481789980549938),
    // vec3(0.703488576674201, 0.9481789980549938, 0.07306080627040201),
    // vec3(0.9481789980549938, 0.07306080627040201, 0.7130252583857399),
    // vec3(0.07306080627040201, 0.7130252583857399, 0.9789645187388992),
    // vec3(0.7130252583857399, 0.9789645187388992, 0.33678550685338693),
    // vec3(0.9789645187388992, 0.33678550685338693, 0.7330370312211354),
    // vec3(0.33678550685338693, 0.7330370312211354, 0.004795889352936622),
    // vec3(0.7330370312211354, 0.004795889352936622, 0.09874319434048984),
    // vec3(0.004795889352936622, 0.09874319434048984, 0.6221792473995474),
    // vec3(0.09874319434048984, 0.6221792473995474, 0.057902147895985134),
    // vec3(0.6221792473995474, 0.057902147895985134, 0.15117443822623622),
    // vec3(0.057902147895985134, 0.15117443822623622, 0.31705633247036347),
    // vec3(0.15117443822623622, 0.31705633247036347, 0.25297818838113084),
    // vec3(0.31705633247036347, 0.25297818838113084, 0.6803181987928584),
    // vec3(0.25297818838113084, 0.6803181987928584, 0.4192351951480452),
    // vec3(0.6803181987928584, 0.4192351951480452, 0.5643979334671377),
    // vec3(0.4192351951480452, 0.5643979334671377, 0.7173832720660689),
    // vec3(0.5643979334671377, 0.7173832720660689, 0.6599772751105593),
    // vec3(0.7173832720660689, 0.6599772751105593, 0.5505616681095739),
    // vec3(0.6599772751105593, 0.5505616681095739, 0.536083844967051),
    // vec3(0.5505616681095739, 0.536083844967051, 0.3643038196137729),
    // vec3(0.536083844967051, 0.3643038196137729, 0.9522690282729414),
    // vec3(0.3643038196137729, 0.9522690282729414, 0.9046828186054915),
    // vec3(0.9522690282729414, 0.9046828186054915, 0.6751780829678651),
    // vec3(0.9046828186054915, 0.6751780829678651, 0.0804896629429136),
    // vec3(0.6751780829678651, 0.0804896629429136, 0.13754989349424818),
    // vec3(0.0804896629429136, 0.13754989349424818, 0.05185458382453101),
    // vec3(0.13754989349424818, 0.05185458382453101, 0.305751453821437),
    // vec3(0.05185458382453101, 0.305751453821437, 0.32705020489644),
    // vec3(0.305751453821437, 0.32705020489644, 0.10086888299005448),
    // vec3(0.32705020489644, 0.10086888299005448, 0.5584740403551578),
    // vec3(0.10086888299005448, 0.5584740403551578, 0.25045909571817715),
    // vec3(0.5584740403551578, 0.25045909571817715, 0.9405549001711998),
    // vec3(0.25045909571817715, 0.9405549001711998, 0.8087762611328108),
    // vec3(0.9405549001711998, 0.8087762611328108, 0.48986473949779075),
    // vec3(0.8087762611328108, 0.48986473949779075, 0.6638507772346451),
    // vec3(0.48986473949779075, 0.6638507772346451, 0.9895486648026341),
    // vec3(0.6638507772346451, 0.9895486648026341, 0.934725066004673),
    // vec3(0.9895486648026341, 0.934725066004673, 0.9692061073213116),
    // vec3(0.934725066004673, 0.9692061073213116, 0.6466875450611609),
};

const vec3 shape_scales_9[65] = {
    vec3(30.),

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


bool is_cube() {
    // scene 9
    if (0) {
        return (gl_VertexID / c_index.length()) % 3 == 0;
    }
    // scene 0
    if (1) {
        return (gl_VertexID / c_index.length()) != 1;
    }
}

void main() {
    uint VERTS_PER_SHAPE = c_index.length();
    uint shape_id = gl_VertexID / VERTS_PER_SHAPE;

    uint index;
    if (is_cube()) {
        index    = c_index[gl_VertexID % VERTS_PER_SHAPE];
    } else {
        index    = t_index[gl_VertexID % VERTS_PER_SHAPE];
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
    if (0) {
        rot = make_rot(vec3(1., 1., 0.), shape_rots_9[shape_id]);
    } else if (1) {
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
    }

    // Per vertex, regardless of shape
    if (shape_id == 1) {
        vec4 rot2 = make_rot(vec3(0., 0., 1.), -0.05 * shape_rots_0[shape_id] * uTime);
        rot = rotate_q(rot, rot2);
    }
    vRot = rot;

    vec3 v_pos;
    if (is_cube()) {
        v_pos = c_pos[index];
    } else {
        v_pos = t_pos[index];
    }
    vec4 pos = vec4(rotate_v(v_pos, rot), 1.);
    gl_Position = uProjView * model * pos;
    vWorldPos = (model * pos).xyz;

    // Per face - each face is defined with 4 index values, 6 faces makes a shape
    vec3 normal = vec3(1., 0., 1.);
    if (is_cube()) {
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
