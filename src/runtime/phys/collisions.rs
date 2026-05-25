pub use crate::runtime::math::Float2;
use crate::runtime::phys::ContactManifold;
/// Circle vs Circle
pub fn circle_circle(
    pos_a: Float2,
    ra: f32,
    pos_b: Float2,
    rb: f32,
) -> Option<(Float2, f32, ContactManifold)> {
    let d = pos_b - pos_a;
    let dist_sq = d.length_sq();
    let sum_r = ra + rb;
    if dist_sq >= sum_r * sum_r {
        return None;
    }
    let dist = dist_sq.sqrt();
    let normal = if dist < 1e-10 {
        Float2::new(1.0, 0.0)
    } else {
        d * (1.0 / dist)
    };
    let depth = sum_r - dist;
    let mut manifold = ContactManifold::new();
    manifold.push(pos_a + normal * ra);
    Some((normal, depth, manifold))
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

/// Sutherland-Hodgman clip of `subject` (up to 8 pts) against one half-plane edge.
/// `edge_a`→`edge_b` defines the clip edge; inside is to the left (CCW winding).
/// Writes results into `output`, returns new count.
fn clip_against_edge(
    input: &[Float2],
    input_count: usize,
    edge_a: Float2,
    edge_b: Float2,
    output: &mut [Float2; 8],
) -> usize {
    let edge_dir = edge_b - edge_a;
    let edge_normal = Float2::new(-edge_dir.y, edge_dir.x); // inward normal (CCW)
    let mut output_count = 0usize;

    for k in 0..input_count {
        let current = input[k];
        let previous = input[(k + input_count - 1) % input_count];

        let d_curr = edge_normal.dot(current - edge_a);
        let d_prev = edge_normal.dot(previous - edge_a);

        if d_curr >= 0.0 {
            if d_prev < 0.0 {
                // Previous outside → current inside: emit intersection.
                let t = d_prev / (d_prev - d_curr);
                if output_count < 8 {
                    output[output_count] = previous + (current - previous) * t;
                    output_count += 1;
                }
            }
            if output_count < 8 {
                output[output_count] = current;
                output_count += 1;
            }
        } else if d_prev >= 0.0 {
            // Current outside, previous inside: emit intersection only.
            let t = d_prev / (d_prev - d_curr);
            if output_count < 8 {
                output[output_count] = previous + (current - previous) * t;
                output_count += 1;
            }
        }
    }

    output_count
}

/// Full Sutherland-Hodgman clip of `subject` against all edges of `clip` polygon.
/// Both buffers are fixed [Float2; 8]. Returns the clipped manifold.
fn sutherland_hodgman(
    subject: &[Float2],
    subject_count: usize,
    clip: &[Float2],
    clip_count: usize,
) -> ContactManifold {
    // Ping-pong between two fixed buffers to avoid allocation.
    let mut buf_a = [Float2::ZERO; 8];
    let mut buf_b = [Float2::ZERO; 8];

    // Seed buf_a with the subject.
    let copy_count = subject_count.min(8);
    buf_a[..copy_count].copy_from_slice(&subject[..copy_count]);
    let mut current_count = copy_count;

    for i in 0..clip_count {
        if current_count == 0 {
            break;
        }
        let edge_a = clip[i];
        let edge_b = clip[(i + 1) % clip_count];

        // Alternate source/dest so we never allocate.
        if i % 2 == 0 {
            current_count = clip_against_edge(&buf_a, current_count, edge_a, edge_b, &mut buf_b);
        } else {
            current_count = clip_against_edge(&buf_b, current_count, edge_a, edge_b, &mut buf_a);
        }
    }

    // Final result is in whichever buffer was last written.
    let final_buf: &[Float2; 8] = if clip_count % 2 == 0 { &buf_a } else { &buf_b };
    let mut manifold = ContactManifold::new();
    for i in 0..current_count {
        manifold.push(final_buf[i]);
    }
    manifold
}

/// Find the index of the face on `verts` whose normal is most anti-parallel to `normal`.
fn find_incident_face_index(verts: &[Float2], normal: Float2, count: usize) -> usize {
    let mut best_i = 0;
    let mut best_dot = f32::MAX;
    for i in 0..count {
        let edge = verts[(i + 1) % count] - verts[i];
        let face_normal = Float2::new(-edge.y, edge.x).normalize();
        let d = face_normal.dot(normal);
        if d < best_dot {
            best_dot = d;
            best_i = i;
        }
    }
    best_i
}

/// Find the index of the face on `verts` whose normal is most parallel to `normal`.
fn find_reference_face_index(verts: &[Float2], normal: Float2, count: usize) -> usize {
    let mut best_i = 0;
    let mut best_dot = f32::MIN;
    for i in 0..count {
        let edge = verts[(i + 1) % count] - verts[i];
        let face_normal = Float2::new(-edge.y, edge.x).normalize();
        let d = face_normal.dot(normal);
        if d > best_dot {
            best_dot = d;
            best_i = i;
        }
    }
    best_i
}

/// Rectangle vs Rectangle (SAT + Sutherland-Hodgman, up to 8 contacts, zero allocation).
pub fn rect_rect(
    verts_a: &[Float2],
    verts_b: &[Float2],
    pos_a: Float2,
    pos_b: Float2,
) -> Option<(Float2, f32, ContactManifold)> {
    let na = verts_a.len();
    let nb = verts_b.len();
    let mut min_depth = f32::MAX;
    let mut best_normal = Float2::ZERO;

    // SAT over all edge normals of both polygons.
    for pass in 0..2 {
        let verts = if pass == 0 { verts_a } else { verts_b };
        let n = if pass == 0 { na } else { nb };
        for i in 0..n {
            let edge = verts[(i + 1) % n] - verts[i];
            let axis = Float2::new(-edge.y, edge.x).normalize();
            let (min_a, max_a) = project_polygon(verts_a, axis);
            let (min_b, max_b) = project_polygon(verts_b, axis);
            let overlap = sat_overlap(min_a, max_a, min_b, max_b)?;
            if overlap < min_depth {
                min_depth = overlap;
                best_normal = axis;
            }
        }
    }

    // Ensure normal points A → B.
    if (pos_b - pos_a).dot(best_normal) < 0.0 {
        best_normal = -best_normal;
    }

    // Reference face on A, incident face on B.
    let ref_i = find_reference_face_index(verts_a, best_normal, na);
    let inc_i = find_incident_face_index(verts_b, best_normal, nb);

    // Incident face as a 2-point subject for clipping.
    let incident = [verts_b[inc_i], verts_b[(inc_i + 1) % nb]];

    // Clip incident face against all edges of verts_a.
    let clipped = sutherland_hodgman(&incident, 2, verts_a, na);

    // Depth-filter: keep only points behind the reference face plane.
    let ref_face_normal = {
        let e = verts_a[(ref_i + 1) % na] - verts_a[ref_i];
        Float2::new(-e.y, e.x).normalize()
    };
    let ref_d = ref_face_normal.dot(verts_a[ref_i]);

    let mut contacts = ContactManifold::new();
    for i in 0..clipped.count {
        let p = clipped.points[i];
        if ref_face_normal.dot(p) <= ref_d + 1e-4 {
            contacts.push(p);
        }
    }

    if contacts.count == 0 {
        return None;
    }

    Some((best_normal, min_depth, contacts))
}

/// Circle vs Rectangle (SAT, single contact, zero allocation).
pub fn circle_rect(
    circle_pos: Float2,
    radius: f32,
    rect_verts: &[Float2],
    rect_pos: Float2,
) -> Option<(Float2, f32, ContactManifold)> {
    let n = rect_verts.len();
    let mut min_depth = f32::MAX;
    let mut best_normal = Float2::ZERO;

    // Fixed buffer for axes: 4 edge normals + 1 vertex axis = 5 max for a rect.
    let mut axes = [Float2::ZERO; 5];
    let mut axis_count = 0usize;

    for i in 0..n {
        let edge = rect_verts[(i + 1) % n] - rect_verts[i];
        axes[axis_count] = edge.perp().normalize();
        axis_count += 1;
    }

    // Closest-vertex axis.
    let mut closest = rect_verts[0];
    let mut closest_dist_sq = (rect_verts[0] - circle_pos).length_sq();
    for &v in &rect_verts[1..] {
        let dsq = (v - circle_pos).length_sq();
        if dsq < closest_dist_sq {
            closest_dist_sq = dsq;
            closest = v;
        }
    }
    let vertex_axis = (circle_pos - closest).normalize();
    if vertex_axis.length_sq() > 1e-10 && axis_count < 5 {
        axes[axis_count] = vertex_axis;
        axis_count += 1;
    }

    for i in 0..axis_count {
        let axis = axes[i];
        let (min_a, max_a) = project_circle(circle_pos, radius, axis);
        let (min_b, max_b) = project_polygon(rect_verts, axis);
        let overlap = sat_overlap(min_a, max_a, min_b, max_b)?;
        if overlap < min_depth {
            min_depth = overlap;
            best_normal = axis;
        }
    }

    if (circle_pos - rect_pos).dot(best_normal) < 0.0 {
        best_normal = -best_normal;
    }

    let mut manifold = ContactManifold::new();
    manifold.push(circle_pos - best_normal * radius);
    Some((best_normal, min_depth, manifold))
}
