use super::to_triangles;
use super::Shape::{Line, Point, Triangle};

#[test]
fn test_to_triangles() {
    assert_eq!(to_triangles(&[]), vec!());

    assert_eq!(to_triangles(&[(3,None, None)]), vec!(Point((3,None, None))));

    assert_eq!(
        to_triangles(&[
            (1, None, None),
            (2, None, None),
        ]),
        vec!(
            Line(
                (1, None, None),
                (2, None, None),
            ),
        )
    );

    assert_eq!(
        to_triangles(&[
            (1, None, None),
            (2, None, None),
            (3, None, None),
        ]),
        vec!(
            Triangle(
                (3, None, None),
                (1, None, None),
                (2, None, None),
            ),
        )
    );

    assert_eq!(
        to_triangles(&[
            (1, None, None),
            (2, None, None),
            (3, None, None),
            (4, None, None),
        ]),
        vec!(
            Triangle(
                (4, None, None),
                (1, None, None),
                (2, None, None),
            ),
            Triangle(
                (4, None, None),
                (2, None, None),
                (3, None, None),
            ),
        )
    );

    assert_eq!(
        to_triangles(&[
            (1, None, None),
            (2, None, None),
            (3, None, None),
            (4, None, None),
            (5, None, None),
        ]),
        vec!(
            Triangle(
                (5, None, None),
                (1, None, None),
                (2, None, None),
            ),
            Triangle(
                (5, None, None),
                (2, None, None),
                (3, None, None),
            ),
            Triangle(
                (5, None, None),
                (3, None, None),
                (4, None, None),
            ),
        )
    );
}
