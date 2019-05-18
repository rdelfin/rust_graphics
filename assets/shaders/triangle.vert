#version 320 es

precision mediump float;

layout (location = 0) in vec3 Position;
layout (location = 1) in vec3 Color;

uniform mat4 projection;
uniform mat4 view;

out VS_OUTPUT {
    vec3 Color;
} OUT;

void main()
{
    gl_Position = projection * view * vec4(Position, 1.0);
    OUT.Color = Color;
}