#version 450

layout(location = 0) in vec4 v_Position;
layout(location = 1) in vec2 v_Uv;
layout(location = 2) in vec3 v_color;

layout(location = 0) out vec4 o_Target;

layout(set = 2, binding = 0) uniform ShaderInputs_time {
    float u_time;
};
layout(set = 2, binding = 1) uniform ShaderInputs_resolution {
    vec2 u_resolution;
};

// 2D Random
float random(in vec2 st) {
    return fract(sin(dot(st.xy, vec2(12.9898, 78.233))) * 43758.5453123);
}

// 2D Noise based on Morgan McGuire @morgan3d
// https://www.shadertoy.com/view/4dS3Wd
float noise(in vec2 st) {
    vec2 i = floor(st);
    vec2 f = fract(st);

    // Four corners in 2D of a tile
    float a = random(i);
    float b = random(i + vec2(1.0, 0.0));
    float c = random(i + vec2(0.0, 1.0));
    float d = random(i + vec2(1.0, 1.0));

    // Smooth Interpolation

    // Cubic Hermine Curve.  Same as SmoothStep()
    vec2 u = f * f * (3.0 - 2.0 * f);
    u = smoothstep(0.,1.,f);

    // Mix 4 corners percentages
    return mix(a, b, u.x) +
        (c - a) * u.y * (1.0 - u.x) +
        (d - b) * u.x * u.y;
}

void main() {
    vec2 uv = gl_FragCoord.xy/u_resolution;
    o_Target = vec4(vec3(noise(uv * 20 * sin(u_time))), 1.0);
}