#version 450
#extension GL_ARB_separate_shader_objects : enable

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
// layout(set = 2, binding = 2) uniform ShaderInputs_mouse {
//     vec2 u_mouse;
// };

// 2D Random
highp float random(vec2 co) {
    highp float a = 12.9898;
    highp float b = 78.233;
    highp float c = 43758.5453;
    highp float dt = dot(co.xy, vec2(a, b));
    highp float sn = mod(dt, 3.14);
    return fract(sin(sn) * c);
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
    //u = smoothstep(0.,1.,f);

    // Mix 4 corners percentages
    return mix(a, b, u.x) +
        (c - a) * u.y * (1.0 - u.x) +
        (d - b) * u.x * u.y;
}

void main() {
    float aspect = u_resolution.x / u_resolution.y;
    vec2 st = gl_FragCoord.xy / u_resolution.xy;

    float n = noise(gl_FragCoord.xy);
    vec3 col = mix(vec3(0.005 + 0.01 * st.x, 0.01, 0.01), vec3(0.0), n);

    o_Target = vec4(col, 1.0);
}