in vec4 gl_FragCoord; 
out vec4 frag_colour;

in vec2 window;

void main() {
    vec4 topleft = vec4(1.0, 0.0, 0.0, 1.0);
    vec4 botright = vec4(0.0, 0.0, 1.0, 1.0);

    frag_colour = mix(topleft, botright, window.x * window.y);
}

// raymarching algorithm, its like raytracing. sdf for the scene