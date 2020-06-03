#version 300 es

precision mediump float;

in vec2 vertex;

uniform mat4 model;
uniform mat4 view;
uniform mat4 projection;
uniform vec4 color;

void main()
{
	gl_Position.xy = (projection * view * model * vec4(vertex, 0.0, 1.0)).xy;
	gl_Position.z = 0.0;
	gl_Position.w = 1.0;
}
