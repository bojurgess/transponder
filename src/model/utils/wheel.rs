#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct WheelArray<T> {
    pub rear_left: T,
    pub rear_right: T,
    pub front_left: T,
    pub front_right: T,
}

impl<T: Copy> WheelArray<T> {
    pub fn from_slice(slice: &[T; 4]) -> Self {
        Self {
            rear_left: slice[0],
            rear_right: slice[1],
            front_left: slice[2],
            front_right: slice[3],
        }
    }

    pub fn to_array(self) -> [T; 4] {
        [
            self.rear_left,
            self.rear_right,
            self.front_left,
            self.front_right,
        ]
    }
}
