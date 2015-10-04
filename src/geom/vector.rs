
pub struct Vector3{
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl Vector3  {
    /// create a new `Vector3` with supplied values
    pub fn new(x: f32, y: f32, z: f32) -> Vector3  {
        Vector3 {
            x: x, y: y, z: z
        }
    }

    pub fn clone(& self) -> Vector3  {
        Vector3::new(self.x, self.y, self.z)
    }

    pub fn negated(&self) -> Vector3 {
        Vector3::new(-self.x, -self.y, -self.z)
    }

    pub fn plus(&self, a:&Vector3) -> Vector3 {
        Vector3::new(self.x + a.x, self.y + a.y, self.z + a.z)
    }

    pub fn minus(&self, a:&Vector3) -> Vector3 {
        Vector3::new(self.x - a.x, self.y - a.y, self.z - a.z)
    }

    pub fn times(&self, a:f32)  -> Vector3 {
        Vector3::new(self.x * a, self.y * a, self.z * a)
    }

    pub fn divided_by(&self, a: f32) ->Vector3 {
        Vector3::new(self.x / a, self.y / a, self.z / a)
    }

    pub fn dot(&self, a:&Vector3) ->f32 {
        self.x * a.x + self.y * a.y + self.z * a.z
    }

    pub fn lerp(&self, a:&Vector3, t:f32) -> Vector3 {
        let v = a.minus(self);
         self.plus(&v.times(t))
    }

    pub fn length(&self) ->f32 {
        let sq = self.dot(self);
        sq * sq
    }

    pub fn unit(&self) ->Vector3 {
        self.divided_by(self.length())
    }

    pub fn cross(&self, a:&Vector3) ->Vector3 {
        Vector3::new(
            self.y * a.z - self.z * a.y,
            self.z * a.x - self.x * a.z,
            self.x * a.y - self.y * a.x
        )
    }
}


#[test]
fn test_Vector3_new() {
    let pt :Vector3 = Vector3::new(22f32,2f32,2f32);
    assert!(pt.x == 22f32);
}

#[test]
fn test_Vector3_negated() {
    let pt :Vector3 = Vector3::new(22f32,2f32,2f32);
    let pt2 = pt.negated();
    assert!(pt2.x == -22f32);
}

#[test]
fn test_Vector3_plus() {
    let v1 :Vector3 = Vector3::new(22f32,2f32,2f32);
    let v2 :Vector3 = Vector3::new(23f32,1f32,1f32);
    let v3 :Vector3 = v1.plus(&v2);
    assert!(v3.x == 45f32);
    assert!(v3.y == 3f32);
    assert!(v3.z == 3f32);
}

#[test]
fn test_Vector3_minus() {
    let v1 :Vector3 = Vector3::new(22f32,2f32,2f32);
    let v2 :Vector3 = Vector3::new(23f32,1f32,1f32);
    let v3 :Vector3 = v1.minus(&v2);
    assert!(v3.x == -1f32);
    assert!(v3.y == 1f32);
    assert!(v3.z == 1f32);
}

#[test]
fn test_Vector3_times() {
    let v1 :Vector3 = Vector3::new(22f32,2f32,3f32);
    let v3 :Vector3 = v1.times(4f32);
    assert!(v3.x == 88f32);
    assert!(v3.y == 8f32);
    assert!(v3.z == 12f32);
}

#[test]
fn test_Vector3_dividedBy() {
    let v1 :Vector3 = Vector3::new(28f32,24f32,12f32);
    let v3 :Vector3 = v1.divided_by(4f32);
    assert!(v3.x == 7f32);
    assert!(v3.y == 6f32);
    assert!(v3.z == 3f32);
}

#[test]
fn test_Vector3_dot() {
    let v1 :Vector3 = Vector3::new(1f32,0f32,0f32);
    let v2 :Vector3 = Vector3::new(0f32,0f32,1f32);
    let dp = v1.dot(&v2);
    assert!(dp == 0f32);
}

#[test]
fn test_Vector3_cross() {
    let v1 :Vector3 = Vector3::new(1f32,0f32,0f32);
    let v2 :Vector3 = Vector3::new(0f32,0f32,1f32);
    let v3: Vector3 = v1.cross(&v2);
    assert!(v3.x == 0f32);
    assert!(v3.y == -1f32);
    assert!(v3.z == 0f32);
}

