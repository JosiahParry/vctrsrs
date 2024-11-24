
#[derive(Debug, Clone)]
pub struct Point {
    x: f64,
    y: f64,
}

// #[extendr]
fn make_points(x: Doubles, y: Doubles) -> Vctr<Vec<Option<Point>>> {
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
    Vctr::from(pnts)
}

// #[extendr]
fn centroid(x: Vctr<Vec<Option<Point>>>) -> Robj {
    let mut n = 0f64;

    let centroid = x
        .try_into_inner()
        .expect("points vector")
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
    Vctr::from(res).as_vctr()
}
