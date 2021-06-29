pub mod vector;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_vector_output() {
        let a1 = vector::Vec2d::new(5.7456f64, 1f64);
        let b1 = vector::Vec3d::new(1f64, 2f64, -0.5f64);

        // pub fn to_string(&self) -> String
        assert_eq!(a1.to_string(), String::from("5.7456\t1"));
        assert_eq!(b1.to_string(), String::from("1\t2\t-0.5"));
    }

    #[test]
    fn test_vector_algebra() {
        let acc = 0.000001f64;
        let s = 2.0f64;

        let mut a_ = vector::Vec2d::new(3f64, 1f64);
        let mut a0 = vector::Vec2d::new(3f64, 1f64);
        let a1 = vector::Vec2d::new(3f64, 1f64);
        let a2 = vector::Vec2d::new(1f64, 2f64);
        let a_add = vector::Vec2d::new(4f64, 3f64);
        let a_sub = vector::Vec2d::new(2f64, -1f64);
        let a_mul = vector::Vec2d::new(6f64, 2f64);

        let mut b_ = vector::Vec3d::new(1f64, 2f64, 1f64);
        let mut b0 = vector::Vec3d::new(1f64, 2f64, 1f64);
        let b1 = vector::Vec3d::new(1f64, 2f64, 1f64);
        let b2 = vector::Vec3d::new(2f64, 1f64, 3f64);
        let b_add = vector::Vec3d::new(3f64, 3f64, 4f64);
        let b_sub = vector::Vec3d::new(-1f64, 1f64, -2f64);
        let b_mul = vector::Vec3d::new(2f64, 4f64, 2f64);
        let b_vep = vector::Vec3d::new(7f64, 5f64, 5f64);

        // pub fn add(&self, other: &Vec2d) -> Vec2d
        let a3 = a1.add(&a2);
        assert_eq!(a3.equal(&a_add, acc), true);

        // pub fn add_eq(&mut self, other: &Vec2d)
        a0.add_eq(&a2);
        assert_eq!(a0.equal(&a_add, acc), true);

        // pub fn equal(&self, other: &Vec2d, accuracy: f64) -> bool
        assert_eq!(a1.equal(&a0, acc), true);
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
        assert_eq!(a_sub.equal(&a0, acc), true);

        // pub fn to_polar(&self) -> Vec2dPolar
        let ap = a1.to_polar();
        assert_eq!(ap.alpha - 0.3217505543966422f64 < acc, true);
        assert_eq!(ap.r - 3.1622776601683795f64 < acc, true);
        
        // pub fn add(&self, other: &Vec3d) -> Vec3d
        let b3 = b1.add(&b2);
        assert_eq!(b3.equal(&b_add, acc), true);

        // pub fn add_eq(&mut self, other: &Vec3d)
        b0.add_eq(&b2);
        assert_eq!(b_add.equal(&b0, acc), true);

        // pub fn equal(&self, other: &Vec3d, accuracy: f64) -> bool
        assert_eq!(b1.equal(&b0, acc), true);
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
        assert_eq!(b_sub.equal(&b0, acc), true);

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
}
