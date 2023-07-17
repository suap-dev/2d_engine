pub const VERTEX: &str = r#"
#version 150

in vec2 position;
in vec4 color;

out vec4 v_color;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
    v_color = color;
}
"#;

pub const FRAGMENT: &str = r#"
#version 150

uniform vec4 u_color;

in vec4 v_color;
out vec4 color;

void main() {
    color = v_color;
}
"#;
