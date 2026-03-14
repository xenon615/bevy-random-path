use bevy::prelude::*;
use std::{collections::HashMap};


pub struct RandomLoop;

impl RandomLoop  {
    const ACCURACY: f32 = 0.001;

    /// Returns a Vec<Vec3> representing convex hull around **points_count** random points with spread of **scale**
    #[allow(dead_code)]
    pub fn generate(points_count: usize, scale: Vec3 ) -> Vec<Vec3> {
        let path = (0 .. points_count)
            .map(| _ |  vec3(
                    (fastrand::f32() - 0.5) * scale.x * 2.,
                    (fastrand::f32() - 0.5) * scale.y * 2.,
                    (fastrand::f32() - 0.5) * scale.z * 2.
                )
            )
            .collect::<Vec<_>>();


        let min_z = path.iter().min_by( | a, b |  a.z.total_cmp(&b.z)).unwrap();
        let min_x = path.iter().filter( | e |  e.z == min_z.z).min_by( | a, b | a.x.total_cmp(&b.x)).unwrap();
        let p0 = path.iter().filter( | e |  e.z == min_z.z && e.x == min_x.x).min_by( | a, b | a.y.total_cmp(&b.y)).unwrap();

        let mut dedup:HashMap<i32, Vec3> = HashMap::new();

        path.iter()
            .filter(| p | **p != *p0)
            .map( | p |  {
                let d = (p - p0).normalize().dot(Vec3::X);
                ((d  / Self::ACCURACY) as i32, p)
            })
            .for_each(| p | {
                dedup.entry(p.0).and_modify(| e |  {
                    if (*e - p0).length() < (p.1 - p0).length() {
                        *e = *p.1;
                    }
                })
                .or_insert(*p.1)
                ;
            });

        let mut keys:Vec<_> = dedup.keys().collect();
        keys.sort();
        let p1 = dedup[keys.pop().unwrap()];

        let mut convex_hull = vec![*p0, p1];

        for key in keys.iter().rev() {
            while convex_hull.len() > 1 && ccw(convex_hull[convex_hull.len() - 2], convex_hull[convex_hull.len() - 1], dedup[key]) {
                convex_hull.pop();
            }
            convex_hull.push(dedup[key]);
        }

        let closing_point = *p0 - (*p0 - convex_hull[convex_hull.len() - 1]) * 0.25;
        convex_hull.push(closing_point);
        convex_hull
    }

    // ---

    /// Vary the path  by adding new points shifted by **variation**
    #[allow(dead_code)]
    pub fn vary(path: &mut Vec<Vec3>, variation: f32) {
        let mut i = 1;
        let mut odd_even = 0 ;
        while let Some(current) = path.get(i) {
            let prev = path[i - 1];
            let segment = current - prev;
            let segment_vec = segment.normalize();
            let bn = segment_vec.cross(Vec3::Y).normalize();
            let insert_count = (segment.length() / variation).floor() as usize;
            for j in 0 .. insert_count {
                odd_even += 1;
                let sign = if odd_even % 2 == 0 { 1. } else { -1. };
                let new_point = prev + variation * (segment_vec * j as f32 + sign * bn);
                path.insert(i, new_point);
                i += 1;
            }
            i += 1;
        }
    }

    // ---
    ///  Smooth the path let's limit the rotation angles not less than **min_angle**  and segment length not less than **min_segment_length**
    #[allow(dead_code)]
    pub fn smooth_out(path: &mut Vec<Vec3>, min_angle: f32, min_segment_length: f32) {
        for _l in 0 .. path.len() {
            for i in 1 .. path.len() {
                if (path[i] - path[i -1 ]).length() < min_segment_length  {
                    path.remove(i);
                    break;
                }
            }
        }

        let max_dot = min_angle.cos();
        let smooth_steps = 10;
        for _j in 0 .. smooth_steps {
            let mut angle_ok = true;
            for i in 1 .. path.len() - 1 {
                let vec1 = path[i - 1] - path[i];
                let vec2 = path[i + 1] - path[i];
                if vec1.normalize().dot(vec2.normalize()) > max_dot {
                    angle_ok = false;
                    path[i] += (vec1 + vec2) / smooth_steps as f32;
                }
            }
            if angle_ok {
                break;
            }
        }


        let last =  path.len() - 1;
        for _k in 0 .. smooth_steps {
            let vec1 = path[last - 1 ] - path[last];
            let vec2 = path[1] - path[0];

            if vec1.normalize().dot(vec2.normalize()) < max_dot {
                break;
            }

            let bisec_step = (vec1 + vec2) / smooth_steps as f32;
            path[0] += bisec_step;
            path[last] += bisec_step;
        }
    }
}

// ---

fn ccw (v1: Vec3, v2: Vec3, v3: Vec3) -> bool {
    let back = v2 - v1;
    let back_cross = -back.normalize().cross(Vec3::Y);
    let forward = v3 - v2;
    forward.dot(back_cross) >= 0.
}
