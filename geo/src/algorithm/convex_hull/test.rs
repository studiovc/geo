use super::*;
use crate::geometry::*;
use crate::{coord, line_string, polygon};

#[test]
fn convex_hull_multipoint_test() {
    let v = vec![
        Point::new(0, 10),
        Point::new(1, 1),
        Point::new(10, 0),
        Point::new(1, -1),
        Point::new(0, -10),
        Point::new(-1, -1),
        Point::new(-10, 0),
        Point::new(-1, 1),
        Point::new(0, 10),
    ];
    let mp = MultiPoint::new(v);
    let correct = vec![
        Coord::from((0, -10)),
        Coord::from((10, 0)),
        Coord::from((0, 10)),
        Coord::from((-10, 0)),
        Coord::from((0, -10)),
    ];
    let res = mp.convex_hull();
    assert_eq!(res.exterior().0, correct);
}
#[test]
fn convex_hull_linestring_test() {
    let mp = line_string![
        (x: 0.0, y: 10.0),
        (x: 1.0, y: 1.0),
        (x: 10.0, y: 0.0),
        (x: 1.0, y: -1.0),
        (x: 0.0, y: -10.0),
        (x: -1.0, y: -1.0),
        (x: -10.0, y: 0.0),
        (x: -1.0, y: 1.0),
        (x: 0.0, y: 10.0),
    ];
    let correct = vec![
        Coord::from((0.0, -10.0)),
        Coord::from((10.0, 0.0)),
        Coord::from((0.0, 10.0)),
        Coord::from((-10.0, 0.0)),
        Coord::from((0.0, -10.0)),
    ];
    let res = mp.convex_hull();
    assert_eq!(res.exterior().0, correct);
}
#[test]
fn convex_hull_multilinestring_test() {
    let v1 = line_string![(x: 0.0, y: 0.0), (x: 1.0, y: 10.0)];
    let v2 = line_string![(x: 1.0, y: 10.0), (x: 2.0, y: 0.0), (x: 3.0, y: 1.0)];
    let mls = MultiLineString::new(vec![v1, v2]);
    let correct = vec![
        Coord::from((2.0, 0.0)),
        Coord::from((3.0, 1.0)),
        Coord::from((1.0, 10.0)),
        Coord::from((0.0, 0.0)),
        Coord::from((2.0, 0.0)),
    ];
    let res = mls.convex_hull();
    assert_eq!(res.exterior().0, correct);
}
#[test]
fn convex_hull_multipolygon_test() {
    let p1 = polygon![(x: 0.0, y: 0.0), (x: 1.0, y: 10.0), (x: 2.0, y: 0.0), (x: 0.0, y: 0.0)];
    let p2 = polygon![(x: 3.0, y: 0.0), (x: 4.0, y: 10.0), (x: 5.0, y: 0.0), (x: 3.0, y: 0.0)];
    let mp = MultiPolygon::new(vec![p1, p2]);
    let correct = vec![
        Coord::from((5.0, 0.0)),
        Coord::from((4.0, 10.0)),
        Coord::from((1.0, 10.0)),
        Coord::from((0.0, 0.0)),
        Coord::from((5.0, 0.0)),
    ];
    let res = mp.convex_hull();
    assert_eq!(res.exterior().0, correct);
}

#[test]
fn collection() {
    let collection = GeometryCollection(vec![
        Point::new(0.0, 0.0).into(),
        Triangle::new(
            coord! { x: 1.0, y: 0.0},
            coord! { x: 4.0, y: 0.0},
            coord! { x: 4.0, y: 4.0 },
        )
        .into(),
    ]);

    let convex_hull = collection.convex_hull();
    assert_eq!(
        convex_hull,
        polygon![
            coord! { x: 4.0, y: 0.0 },
            coord! { x: 4.0, y: 4.0 },
            coord! { x: 0.0, y: 0.0 }
        ]
    );
}
