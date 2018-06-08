#[derive(Debug)]
pub struct Point(i32, i32);

impl Point {
    // fn new(x: i32, y: i32) -> Self {
    //     Point(x, y)
    // }

    pub fn origin() -> Self {
        Point(0, 0)
    }

    pub fn x(&self) -> i32 {
        self.0
    }

    pub fn y(&self) -> i32 {
        self.1
    }

    pub fn on_axis(&self) -> bool {
        self.0 == 0 || self.1 == 0
    }

    pub fn reflection(&self) -> Vec<Self> {
        match *self {
            Point(0, 0) => vec![Point(0, 0)],
            Point(x, 0) => vec![Point(x, 0), Point(0, x), Point(-x, 0), Point(0, -x), Point(x, -x), Point(-x, x)],
            Point(x, y) => {
                let sum = x + y;
                vec![
                    Point(x, y),
                    Point(-x, -y),
                    Point(y, x),
                    Point(-y, -x),
                    Point(-x, sum),
                    Point(-y, sum),
                    Point(x, -sum),
                    Point(y, -sum),
                    Point(sum, -x),
                    Point(sum, -y),
                    Point(-sum, x),
                    Point(-sum, y)
                ]
            }
        }
    }

    pub fn neighbours(&self) -> Vec<Point> {
        vec![
            Point(self.0 + 1, self.1), // right
            Point(self.0 - 1, self.1), // left
            Point(self.0, self.1 + 1), // top
            Point(self.0, self.1 - 1), // bottom
            Point(self.0 - 1, self.1 + 1), // top right
            Point(self.0 + 1, self.1 - 1), // bottom left
        ]
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Point) -> bool {
        self.0 == other.0 && self.1 == other.1
    }
}

impl Eq for Point {}
