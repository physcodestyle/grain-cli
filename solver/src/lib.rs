pub mod math;
pub mod entity;

#[cfg(test)]
mod tests {

    #[test]
    fn test_vector_output() {
        use crate::math::vector::{Vec2d, Vec3d};

        let a1 = Vec2d::new(5.7456f64, 1f64);
        let b1 = Vec3d::new(1f64, 2f64, -0.5f64);

        // pub fn to_string(&self) -> String
        assert_eq!(a1.to_string(), String::from("5.7456\t1"));
        assert_eq!(b1.to_string(), String::from("1\t2\t-0.5"));
    }

    #[test]
    fn test_vector_algebra() {
        use crate::math::vector::{Vec2d, Vec3d};

        let acc = 0.000001f64;
        let s = 2.0f64;

        let mut a_ = Vec2d::new(3f64, 1f64);
        let mut a0 = Vec2d::new(3f64, 1f64);
        let a1 = Vec2d::new(3f64, 1f64);
        let a2 = Vec2d::new(1f64, 2f64);
        let a_add = Vec2d::new(4f64, 3f64);
        let a_sub = Vec2d::new(2f64, -1f64);
        let a_mul = Vec2d::new(6f64, 2f64);

        // pub fn add(&self, other: &Vec2d) -> Vec2d
        let a3 = a1.add(&a2);
        assert_eq!(a3.equal(&a_add, acc), true);

        // pub fn add_eq(&mut self, other: &Vec2d)
        a0.add_eq(&a2);
        assert_eq!(a0.equal(&a_add, acc), true);

        // pub fn equal(&self, other: &Vec2d, accuracy: f64) -> bool
        assert_eq!(a1.equal(&a2, acc), false);

        // pub fn length(&self) -> f64
        assert_eq!(a1.length() - 3.1622776601683795f64 < acc, true);
        assert_eq!(a2.length() - 2.23606797749979f64 < acc, true);
        assert_eq!(a3.length() - 5.0f64 < acc, true);

        // pub fn mul(&self, other: &Vec2d) -> Vec2d {
        let a3 = a1.mul(&s);
        assert_eq!(a3.equal(&a_mul, acc), true);

        // pub fn mul_eq(&mut self, other: &Vec2d)
        a_.mul_eq(&s);
        assert_eq!(a_mul.equal(&a_, acc), true);

        // pub fn scp(&self, other: &Vec2d) -> f64 {
        let scp = a1.scp(&a2);
        assert_eq!(scp - 5.0f64 < acc, true);

        // pub fn sub(&self, other: &Vec2d) -> Vec2d {
        let a3 = a1.sub(&a2);
        assert_eq!(a3.equal(&a_sub, acc), true);

        // pub fn sub_eq(&mut self, other: &Vec2d)
        a0.sub_eq(&a2);
        assert_eq!(a1.equal(&a0, acc), true);

        // pub fn to_polar(&self) -> Vec2dPolar
        let ap = a1.to_polar();
        assert_eq!(ap.alpha - 0.3217505543966422f64 < acc, true);
        assert_eq!(ap.r - 3.1622776601683795f64 < acc, true);

        let mut b_ = Vec3d::new(1f64, 2f64, 1f64);
        let mut b0 = Vec3d::new(1f64, 2f64, 1f64);
        let b1 = Vec3d::new(1f64, 2f64, 1f64);
        let b2 = Vec3d::new(2f64, 1f64, 3f64);
        let b_add = Vec3d::new(3f64, 3f64, 4f64);
        let b_sub = Vec3d::new(-1f64, 1f64, -2f64);
        let b_mul = Vec3d::new(2f64, 4f64, 2f64);
        let b_vep = Vec3d::new(7f64, 5f64, 5f64);
        
        // pub fn add(&self, other: &Vec3d) -> Vec3d
        let b3 = b1.add(&b2);
        assert_eq!(b3.equal(&b_add, acc), true);

        // pub fn add_eq(&mut self, other: &Vec3d)
        b0.add_eq(&b2);
        assert_eq!(b_add.equal(&b0, acc), true);

        // pub fn equal(&self, other: &Vec3d, accuracy: f64) -> bool
        assert_eq!(b1.equal(&b2, acc), false);

        // pub fn length(&self) -> f64
        assert_eq!(b1.length() - 2.449489742783178f64 < acc, true);
        assert_eq!(b2.length() - 3.7416573867739413 < acc, true);
        assert_eq!(b3.length() - 5.830951894845301 < acc, true);

        // pub fn mul(&self, other: &Vec3d) -> Vec3d {
        let b3 = b1.mul(&s);
        assert_eq!(b3.equal(&b_mul, acc), true);

        // pub fn mul_eq(&mut self, other: &Vec3d)
        b_.mul_eq(&s);
        assert_eq!(b_mul.equal(&b_, acc), true);

        // pub fn scp(&self, other: &Vec3d) -> f64 {
        let scp = b1.scp(&b2);
        assert_eq!(scp - 7.0f64 < acc, true);

        // pub fn sub(&self, other: &Vec3d) -> Vec3d
        let b3 = b1.sub(&b2);
        assert_eq!(b3.equal(&b_sub, acc), true);

        // pub fn sub_eq(&mut self, other: &Vec3d)
        b0.sub_eq(&b2);
        assert_eq!(b1.equal(&b0, acc), true);

        // pub fn to_polar(&self) -> Vec3dPolar
        let bp = b1.to_polar();
        assert_eq!(bp.alpha - 1.1071487177940906f64 < acc, true);
        assert_eq!(bp.r - 2.449489742783178f64 < acc, true);
        assert_eq!(bp.z - 1.0f64 < acc, true);

        // pub fn to_spherical(&self) -> Vec3dSpherical
        let bs = b1.to_spherical();
        assert_eq!(bs.alpha - 1.1071487177940906f64 < acc, true);
        assert_eq!(bs.beta - 0.4205343352839651f64 < acc, true);
        assert_eq!(bs.r - 2.449489742783178f64 < acc, true);

        // pub fn vep(&self, other: &Vec3d) -> Vec3d
        let b3 = b1.vep(&b2);
        assert_eq!(b3.equal(&b_vep, acc), true);
    }

    #[test]
    fn test_grain_output() {
        use crate::entity::grain::Grain;
        use crate::math::vector::{Vec2d, Vec3d};

        let el_origins = vec![
            (Vec2d::new(1f64, 2f64), Vec2d::new(-3f64, -4f64)),
            (Vec2d::new(-1f64, -2f64), Vec2d::new(3f64, 4f64))
        ];

        let electrons: Grain<Vec2d> = Grain::<Vec2d>::new(
            String::from("electron"),
            el_origins,
            Vec::new(),
            vec![9.10938356e-31f64],
        );

        assert_eq!(format!("{}", electrons), "electron\n1\t2\t|\t-3\t-4\n-1\t-2\t|\t3\t4");
        assert_eq!(format!("{:?}", electrons.get_units()), "[0.000000000000000000000000000000910938356]");

        let el_origins_3d = vec![
            (Vec3d::new(1f64, 2f64, 0f64), Vec3d::new(-3f64, -4f64, 0f64)),
            (Vec3d::new(-1f64, -2f64, 0f64), Vec3d::new(3f64, 4f64, 0f64))
        ];

        let electrons_3d: Grain<Vec3d> = Grain::<Vec3d>::new(
            String::from("electron"),
            el_origins_3d,
            Vec::new(),
            vec![9.10938356e-31f64],
        );

        assert_eq!(format!("{}", electrons_3d), "electron\n1\t2\t0\t|\t-3\t-4\t0\n-1\t-2\t0\t|\t3\t4\t0");
    }

    #[test]
    fn test_grain_calc() {
        use crate::entity::grain::{Grain, Calc};
        use crate::math::vector::{Vec2d};

        let mut electrons: Grain<Vec2d> = Grain::<Vec2d>::new(
            String::from("electron"),
            vec![
                (Vec2d::new(1f64, 2f64), Vec2d::new(-3f64, -4f64)),
                (Vec2d::new(-1f64, -2f64), Vec2d::new(3f64, 4f64)),
            ],
            vec![
                vec![1.60217662e-19f64],
                vec![1.60217662e-19f64],
            ],
            vec![9.10938356e-31f64],
        );

        impl Calc<Vec2d> for Grain<Vec2d> {
            fn distribute(&self, coords: Vec2d) -> Vec<f64> {
                vec![coords.x]
            }
            
            fn migrate(&mut self, delta: &f64) {
                self.change_origin(0, Vec2d::new(0f64, *delta), Vec2d::new(*delta, 0f64));
                self.change_origin(1, Vec2d::new(0f64, *delta), Vec2d::new(*delta, 0f64));
            }
        }

        assert_eq!(format!("{}", electrons), "electron\n1\t2\t|\t-3\t-4\n-1\t-2\t|\t3\t4");
        electrons.distribute(Vec2d::new(0f64, 0f64));
        assert_eq!(format!("{}", electrons), "electron\n1\t2\t|\t-3\t-4\n-1\t-2\t|\t3\t4");
        let dt = 0f64;
        electrons.migrate(&dt);
        assert_eq!(format!("{}", electrons), "electron\n0\t0\t|\t0\t0\n0\t0\t|\t0\t0");
    }

    #[test]
    fn test_geo_output() {
        use crate::math::vector::{Vec2d, Vec3d};
        use crate::math::geo::{Point, Triangle};

        let t2d = Triangle::<Vec2d>::new((
            Point::<Vec2d>::new(Vec2d::new(0.0f64, 0.0f64)),
            Point::<Vec2d>::new(Vec2d::new(1.0f64, 0.0f64)),
            Point::<Vec2d>::new(Vec2d::new(0.0f64, 1.0f64)),
        ));
        let t3d = Triangle::<Vec3d>::new((
            Point::<Vec3d>::new(Vec3d::new(0.0f64, 0.0f64, 0.5f64)),
            Point::<Vec3d>::new(Vec3d::new(1.0f64, 0.0f64, 5.0f64)),
            Point::<Vec3d>::new(Vec3d::new(0.0f64, 1.0f64, 0.5f64)),
        ));

        assert_eq!(format!("{}", t2d), "0\t0\t1\t0\t0\t1");
        assert_eq!(format!("{}", t3d), "0\t0\t0.5\t1\t0\t5\t0\t1\t0.5");
    }

    #[test]
    fn test_geo_plain() {
        use crate::math::vector::{Vec2d};
        use crate::math::geo::{Plain, Point, Triangle};

        let t2d = Triangle::<Vec2d>::new((
            Point::<Vec2d>::new(Vec2d::new(0.0f64, 0.0f64)),
            Point::<Vec2d>::new(Vec2d::new(1.0f64, 0.0f64)),
            Point::<Vec2d>::new(Vec2d::new(0.0f64, 1.0f64)),
        ));
        let acc = 0.00001f64;

        assert_eq!(t2d.is_inside(&Point::<Vec2d>::new(Vec2d::new(0.0f64, 0.0f64)), &acc), true);
        assert_eq!(t2d.is_inside(&Point::<Vec2d>::new(Vec2d::new(-1.0f64, 0.5f64)), &acc), false);
        assert_eq!(t2d.square(), 0.49999999999999983f64);
    }

    #[test]
    fn test_geo_ops() {
        use crate::math::vector::{Vec3d};
        use crate::math::geo::{Point, Space, Triangle};

        let t3d = Triangle::<Vec3d>::new((
            Point::<Vec3d>::new(Vec3d::new(0.0f64, 0.0f64, 0.5f64)),
            Point::<Vec3d>::new(Vec3d::new(1.0f64, 0.0f64, 5.0f64)),
            Point::<Vec3d>::new(Vec3d::new(0.0f64, 1.0f64, 0.5f64)),
        ));
        let acc = 0.00001f64;

        assert_eq!(t3d.is_inside(&Point::<Vec3d>::new(Vec3d::new(0.0f64, 0.0f64, 0.5f64)), &acc), true);
        assert_eq!(t3d.is_inside(&Point::<Vec3d>::new(Vec3d::new(0.0f64, 0.0f64, 0.1f64)), &acc), false);
        assert_eq!(t3d.square(), 2.304886114323224f64);
        assert_eq!(format!("{}", t3d.normal()), "0.9761870601839527\t0\t0.21693045781865616");
    }
}
