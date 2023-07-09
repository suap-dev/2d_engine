pub const VERTEX: &str = r#"
#version 150

in vec2 position;

void main() {
    gl_Position = vec4(position, 0.0, 1.0);
}
"#;

pub const FRAGMENT: &str = r#"
#version 150

uniform vec4 u_color;

out vec4 color;

void main() {
    color = u_color;
}
"#;
