use jandering_engine::types::Vec2;

pub struct Polygon {
    pub points: Vec<Vec2>,
}

//https://www.geeksforgeeks.org/how-to-check-if-a-given-point-lies-inside-a-polygon/
impl Polygon {
    pub fn point_inside(&self, point: Vec2) -> bool {
        let x = point.x;
        let y = point.y;

        self.points
            .iter()
            .enumerate()
            .skip(1)
            .fold(false, |mut acc, (i, p2)| {
                let p1 = self.points[i - 1];
                if y > p1.y.min(p2.y) && y <= p1.y.max(p2.y) {
                    let x_intersection = (y - p1.y) * (p2.x - p1.x) / (p2.y - p1.y) + p1.x;

                    if p1.x == p2.x || x < x_intersection {
                        acc = !acc;
                    }
                }
                acc
            })
    }
}
