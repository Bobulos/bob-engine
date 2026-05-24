pub use crate::float2::Float2;

/// Circle vs Circle
pub fn circle_circle(
    pos_a: Float2,
    ra: f32,
    pos_b: Float2,
    rb: f32,
) -> Option<(Float2, f32, Float2)> {
    let d = pos_b - pos_a;
    let dist_sq = d.length_sq();
    let sum_r = ra + rb;
    if dist_sq >= sum_r * sum_r {
        return None;
    }
    let dist = dist_sq.sqrt();
    let normal = if dist < 1e-10 {
        Float2::new(1.0, 0.0) // degenerate: push in arbitrary direction
    } else {
        d * (1.0 / dist)
    };
    let depth = sum_r - dist;
    let contact = pos_a + normal * ra;
    Some((normal, depth, contact))
}

/// Project polygon vertices onto axis, return (min, max).
pub fn project_polygon(verts: &[Float2], axis: Float2) -> (f32, f32) {
    let mut min = f32::MAX;
    let mut max = f32::MIN;
    for &v in verts {
        let p = v.dot(axis);
        min = min.min(p);
        max = max.max(p);
    }
    (min, max)
}

/// Project circle onto axis, return (min, max).
pub fn project_circle(center: Float2, radius: f32, axis: Float2) -> (f32, f32) {
    let p = center.dot(axis);
    (p - radius, p + radius)
}

/// SAT overlap on one axis — returns penetration (positive = overlap).
pub fn sat_overlap(min_a: f32, max_a: f32, min_b: f32, max_b: f32) -> Option<f32> {
    let overlap = max_a.min(max_b) - min_a.max(min_b);
    if overlap < 0.0 { None } else { Some(overlap) }
}

/// Rectangle vs Rectangle (SAT)
pub fn rect_rect(
    verts_a: &[Float2],
    verts_b: &[Float2],
    pos_a: Float2,
    pos_b: Float2,
) -> Option<(Float2, f32, Float2)> {
    let mut min_depth = f32::MAX;
    let mut best_normal = Float2::ZERO;

    let axes: Vec<Float2> = verts_a
        .windows(2)
        .chain(std::iter::once(
            [verts_a[verts_a.len() - 1], verts_a[0]].as_slice(),
        ))
        .chain(verts_b.windows(2))
        .chain(std::iter::once(
            [verts_b[verts_b.len() - 1], verts_b[0]].as_slice(),
        ))
        .map(|e| (e[1] - e[0]).perp().normalize())
        .collect();

    for axis in &axes {
        let (min_a, max_a) = project_polygon(verts_a, *axis);
        let (min_b, max_b) = project_polygon(verts_b, *axis);
        let overlap = sat_overlap(min_a, max_a, min_b, max_b)?;
        if overlap < min_depth {
            min_depth = overlap;
            best_normal = *axis;
        }
    }

    // Ensure normal points from A to B.
    if (pos_b - pos_a).dot(best_normal) < 0.0 {
        best_normal = -best_normal;
    }

    // Approximate contact as midpoint of overlapping edges (simple version).
    let contact = pos_a + best_normal * (min_depth * 0.5);
    Some((best_normal, min_depth, contact))
}

/// Circle vs Rectangle (SAT with circle-specific axes)
pub fn circle_rect(
    circle_pos: Float2,
    radius: f32,
    rect_verts: &[Float2],
    rect_pos: Float2,
) -> Option<(Float2, f32, Float2)> {
    let mut min_depth = f32::MAX;
    let mut best_normal = Float2::ZERO;

    // Axes from rectangle edges.
    let mut axes: Vec<Float2> = rect_verts
        .windows(2)
        .chain(std::iter::once(
            [rect_verts[rect_verts.len() - 1], rect_verts[0]].as_slice(),
        ))
        .map(|e| (e[1] - e[0]).perp().normalize())
        .collect();

    // Axis from closest vertex to circle center.
    let closest = rect_verts
        .iter()
        .copied()
        .min_by(|&a, &b| {
            let da = (a - circle_pos).length_sq();
            let db = (b - circle_pos).length_sq();
            da.partial_cmp(&db).unwrap()
        })
        .unwrap();
    let vertex_axis = (circle_pos - closest).normalize();
    if vertex_axis.length_sq() > 1e-10 {
        axes.push(vertex_axis);
    }

    for axis in &axes {
        let (min_a, max_a) = project_circle(circle_pos, radius, *axis);
        let (min_b, max_b) = project_polygon(rect_verts, *axis);
        let overlap = sat_overlap(min_a, max_a, min_b, max_b)?;
        if overlap < min_depth {
            min_depth = overlap;
            best_normal = *axis;
        }
    }

    // Ensure normal points from rect → circle (A = rect, B = circle).
    if (circle_pos - rect_pos).dot(best_normal) < 0.0 {
        best_normal = -best_normal;
    }

    let contact = circle_pos - best_normal * radius;
    Some((best_normal, min_depth, contact))
}
