use cgmath::{Matrix4, SquareMatrix, Vector4};
use math::{Mat4x4, Vec4};

#[test]
fn math() {
    let position = [0., 0., -2.];
    let target = [0., 0., 0.];
    
    let view_1 = cgmath::Matrix4::look_at_rh(position.into(), target.into(), [0., 1., 0.].into());
    let view_2 = Mat4x4::look_at(position.into(), target.into());
    assert!(compare(view_1, view_2));

    let aspect = 800. / 600.;
    let proj_1 = cgmath::perspective(cgmath::Deg(90.), aspect, 0.01, 100.);
    let proj_2 = Mat4x4::perspective(math::deg_to_rad(90.), aspect, 0.01, 100.);
    assert!(compare(proj_1, proj_2));

    assert!(compare(view_1 * proj_1, view_2 * proj_2));

    assert!(Mat4x4::from(<[[f32;4];4]>::from(proj_2)) == proj_2);

    assert!(compare(proj_1.invert().unwrap(), proj_2.inverted().unwrap()));

    let v = [1., 0.5, 2., 3.12312];
    let v1 = Vector4::from(v);
    let v2 = Vec4::from(v);
    assert!(compare_v(proj_1 * v1, proj_2 * v2));
}

fn compare_v(a: Vector4<f32>, b: Vec4) -> bool {
    let a: [f32;4] = a.into();
    let b: [f32;4] = b.into();
    if a != b { println!("\n{:?}\n{:?}\n", a, b) }
    a == b
}
fn compare(a: Matrix4<f32>, b: Mat4x4) -> bool {
    let a: [[f32;4];4] = a.into();
    let b: [[f32;4];4] = b.into();
    if a != b {
        println!();
        println!("{:?}", a[0]);
        println!("{:?}", a[1]);
        println!("{:?}", a[2]);
        println!("{:?}", a[3]);
        println!();
        println!("{:?}", b[0]);
        println!("{:?}", b[1]);
        println!("{:?}", b[2]);
        println!("{:?}", b[3]);
        println!();
    }
    a == b
}