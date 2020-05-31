#version 300 es

precision mediump float;

in vec2 vertex_pos;
in vec4 vertex_color;

out vec2 fragment_pos;
out vec4 fragment_color;

uniform mat4 view;
uniform mat4 projection;

void main()
{
	gl_Position.xy = (projection * view * vec4(vertex_pos, 0.0, 1.0)).xy;
	gl_Position.z = 0.0;
	gl_Position.w = 1.0;
	
	fragment_color = vertex_color;
}
