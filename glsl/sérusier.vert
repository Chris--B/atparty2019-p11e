#version 460

const float PI = 3.1415926535897932384626433832795;

/// ==== Vertex Data

/// Triangular Pyriamid
/// See: https://en.wikipedia.org/wiki/Tetrahedron
const vec3 positions[] = {
    // vec4(-1.,  0., -1. / 1.41421356237, 1.),
    // vec4( 1.,  0., -1. / 1.41421356237, 1.),
    // vec4( 0., -1.,  1. / 1.41421356237, 1.),
    // vec4( 0.,  1.,  1. / 1.41421356237, 1.),

    // Front
    vec3(-1.0,  1.0, -1.0),
    vec3( 1.0,  1.0, -1.0),
    vec3( 1.0,  1.0,  1.0),
    vec3(-1.0,  1.0,  1.0),

    // Right
    vec3( 1.0,  1.0,  1.0),
    vec3( 1.0, -1.0,  1.0),
    vec3( 1.0, -1.0, -1.0),
    vec3( 1.0,  1.0, -1.0),

    // Back
    vec3(-1.0, -1.0, -1.0),
    vec3( 1.0, -1.0, -1.0),
    vec3( 1.0, -1.0,  1.0),
    vec3(-1.0, -1.0,  1.0),

    // Left
    vec3(-1.0, -1.0, -1.0),
    vec3(-1.0,  1.0, -1.0),
    vec3(-1.0,  1.0,  1.0),
    vec3(-1.0, -1.0,  1.0),

    // Top
    vec3( 1.0,  1.0,  1.0),
    vec3(-1.0,  1.0,  1.0),
    vec3(-1.0, -1.0,  1.0),
    vec3( 1.0, -1.0,  1.0),

    // Bottom
    vec3(-1.0, -1.0, -1.0),
    vec3( 1.0, -1.0, -1.0),
    vec3( 1.0,  1.0, -1.0),
    vec3(-1.0,  1.0, -1.0),
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

const uint vert_indices[] = {
     0,  1,  2, /**/  0,  2,  3, // Front
     4,  5,  6, /**/  4,  6,  7, // Right
     8,  9, 10, /**/  8, 10, 11, // Back
    12, 13, 14, /**/ 12, 14, 15, // Left
    16, 17, 18, /**/ 16, 18, 19, // Top
    20, 21, 22, /**/ 20, 22, 23, // Bottom
};

/// ==== Shape Data

const vec3 shape_offets[] = {
    vec3( 0.0,  0.0,  0.0), // Skybox
    // vec3(-0.5, -0.5,  0.5),
    // vec3(-0.5,  0.5, -0.5),
    // vec3( 0.5, -0.5,  0.5),
    // vec3( 0.5,  0.5, -0.5),

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

const float shape_time_scales[] = {
    0., // Skybox
    3. / 2.,
    5. / 2.,
    7. / 2.,
    9. / 2.,
};

layout(location = 0) uniform mat4  uProjView;
layout(location = 1) uniform float uTime;

layout(location = 0) out vec3 vColor;
layout(location = 1) out vec3 vWorldPos;
layout(location = 2) out vec3 vNormal;

void main() {
    const uint VERTS_PER_SHAPE = vert_indices.length();
    const uint index  = vert_indices[gl_VertexID % VERTS_PER_SHAPE];
    const uint shape_id = gl_VertexID / VERTS_PER_SHAPE;

    vColor = vec3(0.8, 0.8, 0.8);

    float t      = shape_time_scales[shape_id % shape_time_scales.length()] * uTime;
    vec3  offset = shape_offets[shape_id];
    vec3  scale  = vec3(0.8, 0.8, 0.8);

    if (shape_id == 0) {
        offset = vec3(0);
        scale = vec3(15.);
    } else {
        offset *= 4.;
    }
    const mat4 model_offset = mat4(
        vec4(1., 0., 0., 0.),
        vec4(0., 1., 0., 0.),
        vec4(0., 0., 1., 0.),
        vec4(offset,     1.)
    );

    const mat4 model_scale = mat4(
        vec4(scale.x,      0.,         0.,  0.),
        vec4(0.,      scale.y,         0.,  0.),
        vec4(0.,           0.,    scale.z,  0.),
        vec4(0.,           0.,         0.,  1.)
    );

    const mat4 model = model_offset * model_scale;

    // Per vertex, regardless of shape
    const vec4 pos = vec4(positions[index], 1.0);
    gl_Position = uProjView * model * pos;
    vWorldPos = (model * pos).xyz;

    // Per face - each face is defined with 4 index values, 6 faces makes a shape
    vec3 normal = (model * vec4(vert_normals[(index / 4) % 6], 0.)).xyz;
    if (shape_id == 0) {
        normal = -normal;
    }
    vNormal = normal;

    // Per shape
}
