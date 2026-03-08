use bevy::prelude::*;
use std::{collections::HashMap};


pub struct RandomPath<'a> {
    count: u32,
    map_dim: Vec3,
    predefined: Option<&'a Vec<Vec3>>,
    accuracy: f32
}

impl <'a>RandomPath<'a> {
    pub fn new(count: u32, map_dim: Vec3) -> Self {
        Self {
            count,
            map_dim,
            predefined: None,
            accuracy: 0.001
       }
    }

    // ---
    #[allow(dead_code)]
    pub fn from_predefined(predefined: &'a Vec<Vec3>) -> Self{
        Self {
            count: 0,
            map_dim: Vec3::ZERO,
            predefined: Some(predefined),
            accuracy: 0.001
       }
    }

    // ---

    #[allow(dead_code)]
    fn with_accuracy(mut self, accuracy: f32) -> Self {
        self.accuracy = accuracy;
        self
    }

    // ---

    #[allow(dead_code)]
    pub fn generate_convex_hull(&self) -> Vec<Vec3> {
        let path = match self.predefined {
            Some(path)=> path.clone(),
            _ => (0 .. self.count)
                .map(| _ |  vec3(
                        (fastrand::f32() - 0.5) * self.map_dim.x * 2.,
                        (fastrand::f32() - 0.5) * self.map_dim.y * 2.,
                        (fastrand::f32() - 0.5) * self.map_dim.z * 2.
                    )
                )
                .collect::<Vec<_>>()

        };


        let min_z = path.iter().min_by( | a, b |  a.z.total_cmp(&b.z)).unwrap();
        let min_x = path.iter().filter( | e |  e.z == min_z.z).min_by( | a, b | a.x.total_cmp(&b.x)).unwrap();
        let p0 = path.iter().filter( | e |  e.z == min_z.z && e.x == min_x.x).min_by( | a, b | a.y.total_cmp(&b.y)).unwrap();
        // println!("{:?}", p0);
        let mut dedup:HashMap<i32, Vec3> = HashMap::new();

        path.iter()
            .filter(| p | **p != *p0)
            .map( | p |  {
                let d = (p - p0).normalize().dot(Vec3::X);
                ((d  / self.accuracy) as i32, p)
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

        // println!("{:?}", dedup);

        let mut keys:Vec<_> = dedup.keys().collect();
        keys.sort();
        let p1 = dedup[keys.pop().unwrap()];

        // println!("{:?}", keys);
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

    // pub fn vary(convex_hull: &Vec<Vec3>) -> Vec<Vec3> {
    //     let mut convex_hull2: Vec<Vec3> = vec![convex_hull[0]];
    //     for i in 1 .. convex_hull.len() {
    //         let tang = (convex_hull[i] - convex_hull[i - 1]).normalize();
    //         let half = (convex_hull[i] + convex_hull[i - 1]) * 0.5;
    //         let div = tang.cross(Vec3::Y).normalize();
    //         // let sign = (fastrand::f32() - 0.5).signum();
    //         let sign = if i % 2 == 0 { 1. } else { -1. };
    //         let half_cross = half - sign * div *  fastrand::f32() * 20. ;
    //         convex_hull2.push(half_cross);
    //         convex_hull2.push(convex_hull[i]);
    //     }

    //     convex_hull2
    // }

    pub fn vary(path: &mut Vec<Vec3>) {
        let mut i = 1;
        let mut odd_even = 0 ;
        while let Some(current) = path.get(i) {
            println!("{:?}", i);
            let segment = current - path[i - 1];
            let half = (current + path[i - 1]) * 0.5;
            let div = segment.normalize().cross(Vec3::Y).normalize();
            // let sign = (fastrand::f32() - 0.5).signum();
            let sign = if odd_even % 2 == 0 { 1. } else { -1. };
            let half_cross = half - sign * div *  fastrand::f32() * segment.length() * 0.5 ;
            // let half_cross = half - sign * div * 2.  ;
            path.insert(i, half_cross);
            i += 2;
            odd_even += 1 ;
        }
    }



    // ---

    pub fn smooth_out(path: &mut Vec<Vec3>, min_angle: f32, min_segment_length: f32) {
        let max_dot = min_angle.cos();
        // println!("{} {}", min_angle, max_dot);
        for _j in 0 .. 1000 {
            let mut updated = false;
            for i in 1 .. path.len() - 1 {
                let vec1 = (path[i - 1] - path[i]).normalize();
                let vec2 = (path[i + 1] - path[i]).normalize();
                if vec1.dot(vec2) > max_dot {
                    updated = true;
                    path[i] += (vec1 + vec2).normalize() * 0.5
                }
            }
            if !updated {
                break;
            }
        }
        for _l in 0 .. path.len() {
            for i in 1 .. path.len() {
                if (path[i] - path[i -1 ]).length() < min_segment_length  {
                    path.remove(i);
                    break;
                }
            }
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
