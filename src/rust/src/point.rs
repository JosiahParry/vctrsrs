use extendr_api::prelude::*;
use extendr_vctrs::{extendr_vctr, Rvctr, Vctr};


#[derive(Debug, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

#[extendr_vctr("points")]
#[derive(Debug, Clone, Rvctr)]
pub struct Points(Vec<Option<Point>>);



/// @export
#[extendr]
fn make_points(x: Doubles, y: Doubles) -> Points {
    let pnts = x.into_iter().zip(y.into_iter())
    .map(|(x, y)| {
        if x.is_na() || y.is_na() {
            None
        } else {
            let pnt = Point {
                x: x.inner(),
                y: y.inner()
            };
            Some(pnt)
        }
    }).collect::<Vec<_>>();
    Points(pnts)
}

/// @export
#[extendr]
fn centroid(x: Points) -> Result<Points> {
    let mut n = 0f64;

    let centroid = x
        .0
        .iter()
        .fold((0f64, 0f64), |mut acc, next| {
            if let Some(pnt) = next {
                n += 1.0;
                acc.0 += pnt.x;
                acc.1 += pnt.y;
            }
            acc
        });

    let res = vec![
        Some(Point { x: centroid.0 / n, y: centroid.1 / n})
    ];
    Ok(Points(res))
}

extendr_module! {
    mod point;
    // fn show_points;
    fn centroid;
    fn make_points;
}

