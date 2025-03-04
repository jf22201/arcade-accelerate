use pyo3::prelude::*;

#[pyfunction]
pub fn are_polygons_intersecting(poly_a: Vec<(f32, f32)>, poly_b: Vec<(f32, f32)>) -> bool {
    let polygons = [poly_a, poly_b];
    for polygon in &polygons {
        for i1 in 0..polygon.len() {
            let i2 = (i1 + 1) % polygon.len();
            let projection_1 = polygon[i1];
            let projection_2 = polygon[i2];

            let normal = (
                projection_2.1 - projection_1.1,
                projection_1.0 - projection_2.0,
            );

            let mut min_a: Option<f32> = None;
            let mut max_a: Option<f32> = None;
            let mut min_b: Option<f32> = None;
            let mut max_b: Option<f32> = None;

            for point in &polygons[0] {
                let projected = normal.0 * point.0 + normal.1 * point.1;
                match min_a {
                    Some(x) if projected < x => min_a = Some(projected),
                    Some(_x) => {}
                    None => min_a = Some(projected),
                }
                match max_a {
                    Some(x) if projected > x => max_a = Some(projected),
                    Some(_x) => {}
                    None => max_a = Some(projected),
                }
            }

            for point in &polygons[1] {
                let projected = normal.0 * point.0 + normal.1 * point.1;
                match min_b {
                    Some(x) if projected < x => min_b = Some(projected),
                    Some(_x) => {}
                    None => min_b = Some(projected),
                }
                match max_b {
                    Some(x) if projected > x => max_b = Some(projected),
                    Some(_x) => {}
                    None => max_b = Some(projected),
                }
            }

            if max_a <= min_b || max_b <= min_a {
                return false;
            }
        }
    }
    true
}

#[pyfunction]
pub fn is_point_in_box(p: (f32, f32), q: (f32, f32), r: (f32, f32)) -> bool {
    (q.0 <= p.0.max(r.0)) && (q.0 >= p.0.min(r.0)) && (q.1 <= p.1.max(r.1)) && (q.1 >= p.1.min(r.1))
}

#[pyfunction]
pub fn get_triangle_orientation(p: (f32, f32), q: (f32, f32), r: (f32, f32)) -> i32 {
    let val: f32 = ((q.1 - p.1) * (r.0 - q.0)) - ((q.0 - p.0) * (r.1 - q.1));
    if val == 0.0 {
        0 //collinear
    } else if val > 0.0 {
        1 // clockwise
    } else {
        2 // counter clockwise
    }
}

#[pyfunction]
pub fn are_lines_intersecting(
    p1: (f32, f32),
    q1: (f32, f32),
    p2: (f32, f32),
    q2: (f32, f32),
) -> bool {
    let o1 = get_triangle_orientation(p1, q1, p2);
    let o2 = get_triangle_orientation(p1, q1, q2);
    let o3 = get_triangle_orientation(p1, q1, q2);
    let o4 = get_triangle_orientation(p2, q2, q1);
    // General case
    ((o1 != o2) && (o3 != o4))
    // p1, q1 and p2 are collinear and p2 lies on segment p1q1
    || ((o1 == 0) && is_point_in_box(p1, p2, q1))
    // p1, q1 and p2 are collinear and q2 lies on segment p1q1
    || ((o2 == 0) && is_point_in_box(p1, q2, q1))
    // p2, q2 and p1 are collinear and p1 lies on segment p2q2
    || ((o3 == 0) && is_point_in_box(p2, p1, q2))
    // p2, q2 and q1 are collinear and q1 lies on segment p2q2
    || ((o4 == 0) && is_point_in_box(p2, q1, q2))
}
