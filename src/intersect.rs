use fast_floats::Fast;
#[derive(Copy, Clone, Debug, StructOfArray)]
pub struct Sphere {
    pub x: Fast<f32>,
    pub y: Fast<f32>,
    pub z: Fast<f32>,
    pub r: Fast<f32>,
}

pub struct Spheres {
    pub x: Vec<Fast<f32>>,
    pub y: Vec<Fast<f32>>,
    pub z: Vec<Fast<f32>>,
    pub r: Vec<Fast<f32>>,
}

pub fn cull_with_soa(target: Sphere, spheres: Spheres) -> bool {
    let mut intersects = false;
    let len = spheres.x.len();

    assert!(spheres.x.len() >= len);
    assert!(spheres.y.len() >= len);
    assert!(spheres.z.len() >= len);
    assert!(spheres.r.len() >= len);

    for i in 0..len {
        let mut x = target.x - spheres.x[i];
        let mut y = target.y - spheres.y[i];
        let mut z = target.z - spheres.z[i];
        let mut r = target.r + spheres.r[i];

        x *= x;
        y *= y;
        z *= z;
        r *= r;

        if x + y + z < r {
            intersects = true;
        }
    }
    intersects
}

impl<'a> SphereRef<'a> {
    #[inline]
    pub fn intersect(self, other: Self) -> bool {
        let mut dx = *self.x - *other.x;
        let mut dy = *self.y - *other.y;
        let mut dz = *self.z - *other.z;
        let mut dr = *self.r + *other.r;

        dx *= dx;
        dy *= dy;
        dz *= dz;
        dr *= dr;

        dx + dy + dz < dr
    }
}

impl Sphere {
    #[inline]
    pub fn intersect(self, other: Self) -> bool {
        let mut dx = self.x - other.x;
        let mut dy = self.y - other.y;
        let mut dz = self.z - other.z;
        let mut dr = self.r + other.r;

        dx *= dx;
        dy *= dy;
        dz *= dz;
        dr *= dr;

        dx + dy + dz < dr
    }
}

pub fn cull_aos(target: Sphere, spheres: &[Sphere]) -> bool {
    spheres
        .iter()
        .any(|&sphere| Sphere::intersect(target, sphere))
}

pub fn cull_soa3(target: SphereRef, spheres: &SphereVec) -> bool {
    let mut intersects = false;
    for i in 0..spheres.len() {
        let sphere = unsafe {
            SphereRef {
                x: spheres.x.get_unchecked(i),
                y: spheres.x.get_unchecked(i),
                z: spheres.x.get_unchecked(i),
                r: spheres.x.get_unchecked(i),
            }
        };
        if SphereRef::intersect(target, sphere) {
            intersects = true;
        }
    }
    intersects
}
pub fn cull_soa(target: SphereRef, spheres: &SphereVec) -> bool {
    spheres
        .iter()
        .any(|sphere| SphereRef::intersect(target, sphere))
}

pub fn cull_soa2(target: SphereRef, spheres: &SphereVec) -> bool {
    let mut intersects = false;
    spheres.iter().for_each(|sphere| {
        if SphereRef::intersect(target, sphere) {
            intersects = true;
        }
    });
    intersects
}
