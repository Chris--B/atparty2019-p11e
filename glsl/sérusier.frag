#version 460

layout(location = 0) in  vec4 vRot;
layout(location = 1) in  vec3 vWorldPos;
layout(location = 2) in  vec3 vNormal;

layout(location = 0) out vec4 fragColor;

layout(location = 1) uniform float uTime;

const float PI = 3.1415926535897932384626433832795;

vec4 qmul(vec4 q1, vec4 q2) {
    return vec4(
        (q2.xyz * q1.w) + (q1.xyz * q2.w) + cross(q1.xyz, q2.xyz),
        (q1.w   * q2.w)                   - dot(q1.xyz, q2.xyz)
    );
}

vec3 rotate_vector(vec3 v, vec4 r) {
    vec4 r_c = r * vec4(-1, -1, -1, 1);
    return qmul(r, qmul(vec4(v, 0), r_c)).xyz;
}

vec2 circle_5(int fifth) {
    float t = uTime - fifth * (2.*PI) /5;
    return 10. * vec2(sin(t), cos(t));
}

vec3 rgb(int r, int g, int b) {
    return vec3(r, g, b) / 255.;
}

vec3 color_z2(float z) {
    return vec3(0.9);
}

vec3 color_z(float z) {
    // Normalize into [0, 1].
    float zz = (z + 10.) / 20.;
    vec3 palette[] = {
        rgb(  2,  86, 208), // Dark blue
        rgb(  0, 140,  81), // Blue-green
        rgb(  0, 140,  54), // Dark green
        rgb(255, 208,  10), // Lighter orange
        rgb(255, 160,  15), // Darkish orange
        rgb(255, 70,   15), // Red orange
    };

    // Use this one for awesome corruption (on Raven)
    // float i_f  = zz * palette.length();
    float i_f  = zz * (palette.length() - 1);
    int   i_lo = clamp(int(i_f),       0, palette.length());
    int   i_hi = clamp(int(i_f + 0.5), 0, palette.length());

    return mix(palette[i_lo], palette[i_hi], fract(i_f));
}

void main() {
    int  light_cap   = int(15. * uTime);
    int  light_count = 0;
    vec3 diffuse     = vec3(0.);

    for (int i = 0; i < 5; i += 1) {
        if (i % 2 == 1) {
            // continue;
        }

        for (float zz = -10.; zz < 11.; zz += 2.) {
            if (light_count > light_cap) {
                break;
            }

            vec3 l_pos = vec3(circle_5(i), zz);
            vec3 world_pos = vWorldPos;
            vec3 normal = rotate_vector(vNormal, vRot);
            if (gl_PrimitiveID <= 12) {
                normal    = -normalize(world_pos);
                world_pos = 10. * -normal;
            }
            vec3 delta = l_pos - world_pos;
            if (gl_PrimitiveID > 12) {
                delta.z *= 2; // warp space time
            }
            float dist = dot(delta, delta);

            float a = 1. / (1. + dist);
            a *= mix(0., 10., clamp(0., 1., (15. * uTime) - light_count));

            vec3 l = normalize(l_pos - world_pos);
            vec3 n = normalize(normal);
            float discord = dot(n, l);
            diffuse += a * color_z(zz) * max(discord, 0.0);
            light_count += 1;
        }
    }

    fragColor = vec4(diffuse, 1.);
}
