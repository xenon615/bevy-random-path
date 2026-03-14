# Generate Random Loop  
Let's say we need a race track, and we need to get a random set  of points that describe it.  
Please refer to [/examples/basic.rs](/examples/basic.rs) 

Lets create convex hull around random points  

```rust
    let mut rpath = RandomLoop::generate(12, vec3(100., 0., 100.));
```
![image1](img/img1.png)
Add extra points

```rust
    RandomLoop::vary(&mut rpath, 50.);
```
![image2](img/img2.png)

Smooth it  
```rust
    RandomLoop::smooth_out(&mut rpath, 120f32.to_radians(), 20.);
```
![image3](img/img3.png)

```rust
        let cr = CubicBSpline::new(rpath).to_curve_cyclic().unwrap();
        let spline = cr.iter_positions(120).collect::<Vec<_>>();
```
![image4](img/img4.png)

## Version Compatibility
| bevy | bevy_random_loop |
| ---- | ------------------------- |
| 0.18 | 0.1                       |
