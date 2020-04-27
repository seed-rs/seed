precision mediump float;

attribute vec2 a_vertex;
attribute vec2 a_position;

varying vec2 v_uv;

uniform mat4 u_size;
uniform mat4 u_camera;

void main() {
    mat4 transform = mat4(1.0);

    //https://www.geeks3d.com/20141114/glsl-4x4-matrix-mat4-fields/
    transform[3] = vec4(a_position, 0.0, 1.0);

    mat4 modelViewProjection = u_camera * transform; 

    gl_Position = modelViewProjection * (u_size * vec4(a_vertex,0,1));
    v_uv = a_vertex;
}
