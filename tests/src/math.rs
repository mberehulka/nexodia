use cgmath::{Matrix4, SquareMatrix, Vector4, Rotation3, Deg, Rad, Euler, Matrix3, Decomposed, VectorSpace, InnerSpace};
use math::{Mat4x4, Vec4, Quaternion, Transform, Vec3};

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

    let q1 = cgmath::Quaternion::from(Euler::new(Rad(1.), Rad(2.), Rad(1.5)));
    let q2 = Quaternion::from_euler(1., 2., 1.5);
    assert!(compare_quat(q1, q2));
    assert!(compare(q1.into(), q2.into()));
    assert!(compare_quat(q1 * q1, q2 * q2));

    let mut pq1 = cgmath::Quaternion::from(Matrix3 { x: proj_1.x.truncate(), y: proj_1.y.truncate(), z: proj_1.z.truncate() });
    let mut pq2 = Quaternion::from(proj_2);
    assert!(compare_quat(pq1, pq2));
    assert!(compare_quat(pq1.normalize(), pq2.normalised()));

    assert!(compare_quat(cgmath::Quaternion::from_angle_x(Rad(1.2)), Quaternion::from_angle_x(1.2)));
    assert!(compare_quat(cgmath::Quaternion::from_angle_y(Rad(1.2)), Quaternion::from_angle_y(1.2)));
    assert!(compare_quat(cgmath::Quaternion::from_angle_z(Rad(1.2)), Quaternion::from_angle_z(1.2)));

    let q1 = q1.nlerp(q1*2., 0.5);
    let q2 = q2.nlerp(q2*2., 0.5);
    assert!(compare_quat(q1, q2));

    assert!(compare(
        cgmath::Matrix4::from(
            cgmath::Quaternion::from_angle_x(Rad(0.1)) *
            cgmath::Quaternion::from_angle_y(Rad(1.1)) *
            cgmath::Quaternion::from_angle_z(Rad(0.543))
        ),
        Mat4x4::from(
            Quaternion::from_angle_x(0.1) *
            Quaternion::from_angle_y(1.1) *
            Quaternion::from_angle_z(0.543)
        )
    ));

    assert!(compare(
        cgmath::Matrix4::from_translation(cgmath::Vector3::new(0.1, 1.3, 2.23)) *
        cgmath::Matrix4::from(
            cgmath::Quaternion::from_angle_x(Rad(0.1)) *
            cgmath::Quaternion::from_angle_y(Rad(1.1)) *
            cgmath::Quaternion::from_angle_z(Rad(0.543))
        ) *
        cgmath::Matrix4::from_nonuniform_scale(1.32, 12.1, 6.4),
        Transform::new(
            Vec3::new(0.1, 1.3, 2.23),
            Quaternion::from_angle_x(0.1) *
            Quaternion::from_angle_y(1.1) *
            Quaternion::from_angle_z(0.543),
            Vec3::new(1.32, 12.1, 6.4)
        ).into()
    ));
}

fn compare_v(a: Vector4<f32>, b: Vec4) -> bool {
    let a: [f32;4] = a.into();
    let b: [f32;4] = b.into();
    if a != b { println!("\n{:?}\n{:?}\n", a, b) }
    a == b
}
fn compare_quat(a: cgmath::Quaternion<f32>, b: Quaternion) -> bool {
    let a: [f32;4] = [a.v.x, a.v.y, a.v.z, a.s];
    let b: [f32;4] = b.into();
    if a != b { println!("\n{:?}\n{:?}\n", a, b) }
    a == b
}
fn compare(a: Matrix4<f32>, b: Mat4x4) -> bool {
    let a: [[f32;4];4] = a.into();
    let b: [[f32;4];4] = b.into();
    if a != b {
        println!("\n{:?}\n{:?}\n{:?}\n{:?}\n\n{:?}\n{:?}\n{:?}\n{:?}\n", a[0], a[1], a[2], a[3], b[0], b[1], b[2], b[3]);
    }
    a == b
}