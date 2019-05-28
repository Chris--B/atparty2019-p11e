#version 460

layout(location = 0) in  vec3 vColor;
layout(location = 1) in  vec3 vWorldPos;
layout(location = 2) in  vec3 vNormal;

layout(location = 0) out vec4 fragColor;

layout(location = 1) uniform float uTime;

const float PI = 3.1415926535897932384626433832795;
const float PI_2 = 1.57079632679489661923;
const float PI_4 = 0.785398163397448309616;

struct Light {
    vec3 pos;
    vec3 color;
};

vec3 circle_pos(float t) {
    return -vec3(3. * sin(3. * t), 3. * cos(3. * t), 0.);
}

void main() {

    Light lights[] = {
        Light(circle_pos(uTime - 0*PI_4), vec3( 1.,  0.,  0.)),
        Light(circle_pos(uTime - 1*PI_4), vec3( 0.,  1.,  0.)),
        Light(circle_pos(uTime - 2*PI_4), vec3( 0.,  0.,  1.)),
        Light(circle_pos(uTime - 3*PI_4), vec3(0.7, 0.7, 0.7)),
    };

    vec3 diffuse = vec3(0.);
    for (int i = 0; i < 4; i += 1) {
        float a = 30. / (1. + dot(lights[i].pos - vWorldPos, lights[i].pos - vWorldPos));
        vec3 l = normalize(lights[i].pos - vWorldPos);
        vec3 n = normalize(vNormal);
        float discord = dot(n, l);
        diffuse += a * lights[i].color * max(discord, 0.0);
    }

    fragColor = vec4(vColor * diffuse, 1.);
}
