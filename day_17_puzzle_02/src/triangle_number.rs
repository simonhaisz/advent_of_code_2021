pub trait TriangleNumber {
    fn triangle_number(&self) -> i32;
}

impl TriangleNumber for i32 {
    
    fn triangle_number(&self) -> i32 {
        self * (self.abs() + 1) / 2
    }
}