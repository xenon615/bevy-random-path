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
    pub fn generate(&self) -> Vec<Vec3> {
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

        let far_point = - self.map_dim * 1.5;

        let p0 = path.iter().min_by(|a, b |  {
            far_point.distance_squared(**a).total_cmp(&(far_point.distance_squared(**b)))
        })
        .cloned().unwrap()
        ;

        let mut dedup:HashMap<i32, Vec3> = HashMap::new();

        path.iter()
            .filter(| p | **p != p0)
            .map( | p |  {
                let d = (p - p0).normalize().dot(Vec3::X);
                ((d * 1000.) as i32, p)
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
        let mut convex_hull = vec![p0, p1];

        for key in keys.iter().rev() {
            while convex_hull.len() > 1 && ccw(convex_hull[convex_hull.len() - 2], convex_hull[convex_hull.len() - 1], dedup[key]) {
                convex_hull.pop();
            }
            convex_hull.push(dedup[key]);
        }
        convex_hull.push(p0);
        modify_convex_hull(&convex_hull)
    }

}

// ---

fn modify_convex_hull(convex_hull: &Vec<Vec3>) -> Vec<Vec3> {
    let mut convex_hull2: Vec<Vec3> = vec![convex_hull[0]];
    for i in 1 .. convex_hull.len() {
        let half = (convex_hull[i] +  convex_hull[i - 1]) * 0.5;
        let half_vec = half - convex_hull[i - 1];
        // let sign = (fastrand::f32() - 0.5).signum();
        let half_cross = half -  half_vec.normalize().cross(Vec3::Y)  * half_vec.length() * 0.3;
        convex_hull2.push(half_cross);
        convex_hull2.push(convex_hull[i]);
    }
    convex_hull2
}

// ---

fn ccw (v1: Vec3, v2: Vec3, v3: Vec3) -> bool {
    let back = v2 - v1;
    let back_cross = -back.normalize().cross(Vec3::Y);
    let forward = v3 - v2;
    forward.dot(back_cross) >= 0.
}
