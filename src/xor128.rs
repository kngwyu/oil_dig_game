pub struct Xor128 {
    x: usize,
    y: usize,
    z: usize,
    w: usize,
    t: usize,
}
impl Xor128 {
    pub fn new() -> Xor128 {
        Xor128 {
            x: 123456789,
            y: 362436069,
            z: 521288629,
            w: 88675123,
            t: 1,
        }
    }
    pub fn rand(&mut self) -> i32 {
        self.t = self.x ^ (self.x << 11);
        self.x = self.y;
        self.y = self.z;
        self.z = self.w;
        self.w = (self.w ^ (self.w >> 19)) ^ (self.t ^ (self.t >> 8));
        (self.w & 0x7fffffff) as i32
    }
}
