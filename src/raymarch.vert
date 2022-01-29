layout (location = 0) in vec3 in_pos;

out vec2 window;

void main() {
    gl_Position = vec4(in_pos, 1.0);
    window = in_pos.xy + 1.0 / 2;
}