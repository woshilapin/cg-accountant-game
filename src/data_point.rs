use point::Point;

pub struct DataPoint {
    pub id: usize,
    pub pos: Point,
    pub is_taken: bool,
}

impl DataPoint {
    pub fn new(id: usize, x: isize, y: isize) -> DataPoint {
        DataPoint {
            id: id,
            pos: Point::new(x, y),
            is_taken: false,
        }
    }
    pub fn take(&mut self) {
        self.is_taken = true;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_take() {
        let mut dp1 = DataPoint::new(1, 0, 0);

        assert_eq!(dp1.is_taken, false);
        dp1.take();
        assert_eq!(dp1.is_taken, true);
    }
}
