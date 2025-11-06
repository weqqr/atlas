use std::f32::consts::SQRT_2;

use glam::{Vec2, Vec3, Vec3Swizzles, Vec4, vec2, vec3, vec4};

use crate::{
    asset::Vertex,
    raster::{Image, Rgba},
};

pub fn render_node() -> Image {
    let mut image = Image::new(16, 16);

    let a = Vertex {
        position: vec3(0.5, 0.5, 0.0),
        normal: Vec3::ZERO,
        texcoord: Vec2::ZERO,
    };

    let b = Vertex {
        position: vec3(0.5, -0.5, 0.0),
        normal: Vec3::ZERO,
        texcoord: Vec2::ZERO,
    };

    let c = Vertex {
        position: vec3(-0.5, 0.0, 0.0),
        normal: Vec3::ZERO,
        texcoord: Vec2::ZERO,
    };

    draw_triangle(&mut image, a, b, c);

    image
}

fn cartesian_to_barycentric(p: Vec2, a: Vec2, b: Vec2, c: Vec2) -> Vec3 {
    let u = vec3(c.x - a.x, b.x - a.x, a.x - p.x);
    let v = vec3(c.y - a.y, b.y - a.y, a.y - p.y);
    let w = u.cross(v);

    vec3(1.0 - (w.x + w.y) / w.z, w.y / w.z, w.x / w.z)
}

fn sample_triangle(p: Vec2, a: Vec2, b: Vec2, c: Vec2) -> (bool, Vec3) {
    let sample_point_offset = vec2(0.5, 0.5);
    let barycentric = cartesian_to_barycentric(p + sample_point_offset, a, b, c);

    let in_triangle = barycentric.x > 0.0 && barycentric.y > 0.0 && barycentric.z > 0.0;

    (in_triangle, barycentric)
}

fn sample_texture(texture: &Image, texcoord: Vec2) -> Vec4 {
    assert!(texcoord.x >= 0.0);
    assert!(texcoord.y >= 0.0);

    let x = (texcoord.x * texture.width() as f32) as usize;
    let y = (texcoord.y * texture.height() as f32) as usize;

    let color = texture.get_pixel(x, y);

    let r = color.r as f32 / 255.0;
    let g = color.g as f32 / 255.0;
    let b = color.b as f32 / 255.0;
    let a = color.a as f32 / 255.0;

    vec4(r, g, b, a)
}

fn draw_triangle(target: &mut Image, mut a: Vertex, mut b: Vertex, mut c: Vertex) {
    let origin = vec2(target.width() as f32 / 2.0, target.height() as f32 / 2.0);

    // a.position = projection * a.position;
    // let c = projection * b;
    // let c = projection * c;

    let flip = vec2(1.0, -1.0);
    let base_resolution = 16;
    let scale = base_resolution as f32 * SQRT_2 / 2.0;

    let screen_space_a = a.position.xy() * flip * scale + origin;
    let screen_space_b = b.position.xy() * flip * scale + origin;
    let screen_space_c = c.position.xy() * flip * scale + origin;

    let bbox_min = screen_space_a.min(screen_space_b).min(screen_space_c);
    let bbox_max = screen_space_a.max(screen_space_b).max(screen_space_c);

    for y in bbox_min.x as usize..bbox_max.x as usize + 1 {
        for x in bbox_min.x as usize..bbox_max.x as usize + 1 {
            let p = vec2(x as f32, y as f32);
            let (is_inside_triangle, barycentric) =
                sample_triangle(p, screen_space_a, screen_space_b, screen_space_c);

            if is_inside_triangle {
                target.set_pixel(x, y, Rgba::new(255, 0, 0, 255));
            }
        }
    }
}
