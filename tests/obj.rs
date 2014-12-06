extern crate obj;

use std::io::{File, BufferedReader};

use obj::obj::{parse, ObjSet, Object, Vertex, TVertex, Normal, Geometry};
use obj::obj::Shape::{Line, Triangle};

#[test]
fn test_cube() {
    expect("cube.obj").to_be(ObjSet {
        material_library: "cube.mtl".into_string(),
        objects: vec![
            Object {
                name: "Cube".into_string(),
                vertices: vec![
                    Vertex { x:  1.0, y: -1.0, z: -1.0 },
                    Vertex { x:  1.0, y: -1.0, z:  1.0 },
                    Vertex { x: -1.0, y: -1.0, z:  1.0 },
                    Vertex { x: -1.0, y: -1.0, z: -1.0 },
                    Vertex { x:  1.0, y:  1.0, z: -1.0 },
                    Vertex { x:  1.0, y:  1.0, z:  1.0 },
                    Vertex { x: -1.0, y:  1.0, z:  1.0 },
                    Vertex { x: -1.0, y:  1.0, z: -1.0 }
                ],
                tex_vertices: vec![
                    TVertex { x: 1.004952, y: 0.498633 },
                    TVertex { x: 0.754996, y: 0.498236 },
                    TVertex { x: 0.755393, y: 0.248279 },
                    TVertex { x: 1.005349, y: 0.248677 },
                    TVertex { x: 0.255083, y: 0.497442 },
                    TVertex { x: 0.25548, y: 0.247485 },
                    TVertex { x: 0.505437, y: 0.247882 },
                    TVertex { x: 0.505039, y: 0.497839 },
                    TVertex { x: 0.754598, y: 0.748193 },
                    TVertex { x: 0.504642, y: 0.747795 },
                    TVertex { x: 0.505834, y: -0.002074 },
                    TVertex { x: 0.75579, y: -0.001677 },
                    TVertex { x: 0.005127, y: 0.497044 },
                    TVertex { x: 0.005524, y: 0.247088 },
                ],
                normals : vec![],
                geometry: vec![
                    Geometry {
                        material_name: Some("Material".into_string()),
                        smooth_shading_group: 0,
                        shapes: vec![
                            Triangle((3, Some(3), None),  (0, Some(0), None), (1, Some(1), None)),
                            Triangle((3, Some(3), None),  (1, Some(1), None), (2, Some(2), None)),
                            Triangle((5, Some(7), None),  (4, Some(4), None), (7, Some(5), None)),
                            Triangle((5, Some(7), None),  (7, Some(5), None), (6, Some(6), None)),
                            Triangle((1, Some(1), None),  (0, Some(8), None), (4, Some(9), None)),
                            Triangle((1, Some(1), None),  (4, Some(9), None), (5, Some(7), None)),
                            Triangle((2, Some(2), None),  (1, Some(1), None), (5, Some(7), None)),
                            Triangle((2, Some(2), None),  (5, Some(7), None), (6, Some(6), None)),
                            Triangle((3, Some(11), None), (2, Some(2), None), (6, Some(6), None)),
                            Triangle((3, Some(11), None), (6, Some(6), None), (7, Some(10), None)),
                            Triangle((7, Some(5), None),  (4, Some(4), None), (0, Some(12), None)),
                            Triangle((7, Some(5), None),  (0, Some(12), None), (3, Some(13), None)),
                        ]
                    }
                ]
            }
        ]
    });

    expect("untitled.obj").to_be(ObjSet {
        material_library: "untitled.mtl".into_string(),
        objects: vec!(
            Object {
                name: "Cube.001".into_string(),
                vertices: vec!(
                    Vertex { x: -1.0, y: -1.0, z: 1.0 },
                    Vertex { x: -1.0, y: -1.0, z: -1.0 },
                    Vertex { x: 1.0, y: -1.0, z: -1.0 },
                    Vertex { x: 1.0, y: -1.0, z: 1.0 },
                    Vertex { x: -1.0, y: 1.0, z: 1.0 },
                    Vertex { x: -1.0, y: 1.0, z: -1.0 },
                    Vertex { x: 1.0, y: 1.0, z: -1.0 },
                    Vertex { x: 1.0, y: 1.0, z: 1.0 },
                ),
                tex_vertices: vec!(),
                normals: vec!(),
                geometry: vec!(
                    Geometry {
                        material_name: Some("None".into_string()),
                        smooth_shading_group: 0,
                        shapes: vec!(
                            Triangle((0, None, None), (4, None, None), (5, None, None)),
                            Triangle((0, None, None), (5, None, None), (1, None, None)),
                            Triangle((1, None, None), (5, None, None), (6, None, None)),
                            Triangle((1, None, None), (6, None, None), (2, None, None)),
                            Triangle((2, None, None), (6, None, None), (7, None, None)),
                            Triangle((2, None, None), (7, None, None), (3, None, None)),
                            Triangle((3, None, None), (7, None, None), (4, None, None)),
                            Triangle((3, None, None), (4, None, None), (0, None, None)),
                            Triangle((3, None, None), (0, None, None), (1, None, None)),
                            Triangle((3, None, None), (1, None, None), (2, None, None)),
                            Triangle((4, None, None), (7, None, None), (6, None, None)),
                            Triangle((4, None, None), (6, None, None), (5, None, None)),
                        )
                    }
                )
            },
            Object {
                name: "Circle".into_string(),
                vertices: vec!(
                    Vertex { x: 0.0, y: 0.0, z: -1.0 },
                    Vertex { x: -0.19509, y: 0.0, z: -0.980785 },
                    Vertex { x: -0.382683, y: 0.0, z: -0.92388 },
                    Vertex { x: -0.55557, y: 0.0, z: -0.83147 },
                    Vertex { x: -0.707107, y: 0.0, z: -0.707107 },
                    Vertex { x: -0.83147, y: 0.0, z: -0.55557 },
                    Vertex { x: -0.92388, y: 0.0, z: -0.382683 },
                    Vertex { x: -0.980785, y: 0.0, z: -0.19509 },
                    Vertex { x: -1.0, y: 0.0, z: 0.0 },
                    Vertex { x: -0.980785, y: 0.0, z: 0.19509 },
                    Vertex { x: -0.92388, y: 0.0, z: 0.382683 },
                    Vertex { x: -0.83147, y: 0.0, z: 0.55557 },
                    Vertex { x: -0.707107, y: 0.0, z: 0.707107 },
                    Vertex { x: -0.55557, y: 0.0, z: 0.83147 },
                    Vertex { x: -0.382683, y: 0.0, z: 0.92388 },
                    Vertex { x: -0.19509, y: 0.0, z: 0.980785 },
                    Vertex { x: 0.0, y: 0.0, z: 1.0 },
                    Vertex { x: 0.195091, y: 0.0, z: 0.980785 },
                    Vertex { x: 0.382684, y: 0.0, z: 0.923879 },
                    Vertex { x: 0.555571, y: 0.0, z: 0.831469 },
                    Vertex { x: 0.707107, y: 0.0, z: 0.707106 },
                    Vertex { x: 0.83147, y: 0.0, z: 0.55557 },
                    Vertex { x: 0.92388, y: 0.0, z: 0.382683 },
                    Vertex { x: 0.980785, y: 0.0, z: 0.195089 },
                    Vertex { x: 1.0, y: 0.0, z: -0.000001 },
                    Vertex { x: 0.980785, y: 0.0, z: -0.195091 },
                    Vertex { x: 0.923879, y: 0.0, z: -0.382684 },
                    Vertex { x: 0.831469, y: 0.0, z: -0.555571 },
                    Vertex { x: 0.707106, y: 0.0, z: -0.707108 },
                    Vertex { x: 0.555569, y: 0.0, z: -0.83147 },
                    Vertex { x: 0.382682, y: 0.0, z: -0.92388 },
                    Vertex { x: 0.195089, y: 0.0, z: -0.980786 },
                ),
                tex_vertices: vec!(),
                normals: vec!(),
                geometry: vec!(
                    Geometry {
                        material_name: None,
                        smooth_shading_group: 0,
                        shapes: vec!(
                            Line((1, None, None), (0, None, None)),
                            Line((2, None, None), (1, None, None)),
                            Line((3, None, None), (2, None, None)),
                            Line((4, None, None), (3, None, None)),
                            Line((5, None, None), (4, None, None)),
                            Line((6, None, None), (5, None, None)),
                            Line((7, None, None), (6, None, None)),
                            Line((8, None, None), (7, None, None)),
                            Line((9, None, None), (8, None, None)),
                            Line((10, None, None), (9, None, None)),
                            Line((11, None, None), (10, None, None)),
                            Line((12, None, None), (11, None, None)),
                            Line((13, None, None), (12, None, None)),
                            Line((14, None, None), (13, None, None)),
                            Line((15, None, None), (14, None, None)),
                            Line((16, None, None), (15, None, None)),
                            Line((17, None, None), (16, None, None)),
                            Line((18, None, None), (17, None, None)),
                            Line((19, None, None), (18, None, None)),
                            Line((20, None, None), (19, None, None)),
                            Line((21, None, None), (20, None, None)),
                            Line((22, None, None), (21, None, None)),
                            Line((23, None, None), (22, None, None)),
                            Line((24, None, None), (23, None, None)),
                            Line((25, None, None), (24, None, None)),
                            Line((26, None, None), (25, None, None)),
                            Line((27, None, None), (26, None, None)),
                            Line((28, None, None), (27, None, None)),
                            Line((29, None, None), (28, None, None)),
                            Line((30, None, None), (29, None, None)),
                            Line((31, None, None), (30, None, None)),
                            Line((0, None, None), (31, None, None)),
                        )
                    }
                )
            },
            Object {
                name: "Cube".into_string(),
                vertices: vec!(
                    Vertex { x: 1.0, y: -1.0, z: -1.0 },
                    Vertex { x: 1.0, y: -1.0, z: 1.0 },
                    Vertex { x: -1.0, y: -1.0, z: 1.0 },
                    Vertex { x: -1.0, y: -1.0, z: -1.0 },
                    Vertex { x: 1.0, y: 1.0, z: -0.999999 },
                    Vertex { x: 0.999999, y: 1.0, z: 1.000001 },
                    Vertex { x: -1.0, y: 1.0, z: 1.0 },
                    Vertex { x: -1.0, y: 1.0, z: -1.0 },
                ),
                tex_vertices: vec!(),
                normals: vec!(),
                geometry: vec!(
                    Geometry {
                        material_name: Some("Material".into_string()),
                        smooth_shading_group: 0,
                        shapes: vec!(
                            Triangle((3, None, None), (0, None, None), (1, None, None)),
                            Triangle((3, None, None), (1, None, None), (2, None, None)),
                            Triangle((5, None, None), (4, None, None), (7, None, None)),
                            Triangle((5, None, None), (7, None, None), (6, None, None)),
                            Triangle((1, None, None), (0, None, None), (4, None, None)),
                            Triangle((1, None, None), (4, None, None), (5, None, None)),
                            Triangle((2, None, None), (1, None, None), (5, None, None)),
                            Triangle((2, None, None), (5, None, None), (6, None, None)),
                            Triangle((3, None, None), (2, None, None), (6, None, None)),
                            Triangle((3, None, None), (6, None, None), (7, None, None)),
                            Triangle((7, None, None), (4, None, None), (0, None, None)),
                            Triangle((7, None, None), (0, None, None), (3, None, None)),
                        )
                    }
                )
            }
        )
    });

    expect("normal-cone.obj").to_be(ObjSet {
        material_library: "normal-cone.mtl".into_string(),
        objects: vec![
            Object {
                name: "Cone".into_string(),
                vertices: vec![
                    Vertex { x: 0.000000  , y: -1.000000 , z: -1.000000 },
                    Vertex { x: 0.000000  , y:  1.000000 , z: 0.000000 },
                    Vertex { x: 0.195090  , y: -1.000000 , z: -0.980785 },
                    Vertex { x: 0.382683  , y: -1.000000 , z: -0.923880 },
                    Vertex { x: 0.555570  , y: -1.000000 , z: -0.831470 },
                    Vertex { x: 0.707107  , y: -1.000000 , z: -0.707107 },
                    Vertex { x: 0.831470  , y: -1.000000 , z: -0.555570 },
                    Vertex { x: 0.923880  , y: -1.000000 , z: -0.382683 },
                    Vertex { x: 0.980785  , y: -1.000000 , z: -0.195090 },
                    Vertex { x: 1.000000  , y: -1.000000 , z: -0.000000 },
                    Vertex { x: 0.980785  , y: -1.000000 , z: 0.195090 },
                    Vertex { x: 0.923880  , y: -1.000000 , z: 0.382683 },
                    Vertex { x: 0.831470  , y: -1.000000 , z: 0.555570 },
                    Vertex { x: 0.707107  , y: -1.000000 , z: 0.707107 },
                    Vertex { x: 0.555570  , y: -1.000000 , z: 0.831470 },
                    Vertex { x: 0.382683  , y: -1.000000 , z: 0.923880 },
                    Vertex { x: 0.195090  , y: -1.000000 , z: 0.980785 },
                    Vertex { x: -0.000000 , y: -1.000000 , z: 1.000000 },
                    Vertex { x: -0.195091 , y: -1.000000 , z: 0.980785 },
                    Vertex { x: -0.382684 , y: -1.000000 , z: 0.923879 },
                    Vertex { x: -0.555571 , y: -1.000000 , z: 0.831469 },
                    Vertex { x: -0.707107 , y: -1.000000 , z: 0.707106 },
                    Vertex { x: -0.831470 , y: -1.000000 , z: 0.555570 },
                    Vertex { x: -0.923880 , y: -1.000000 , z: 0.382683 },
                    Vertex { x: -0.980785 , y: -1.000000 , z: 0.195089 },
                    Vertex { x: -1.000000 , y: -1.000000 , z: -0.000001 },
                    Vertex { x: -0.980785 , y: -1.000000 , z: -0.195091 },
                    Vertex { x: -0.923879 , y: -1.000000 , z: -0.382684 },
                    Vertex { x: -0.831469 , y: -1.000000 , z: -0.555571 },
                    Vertex { x: -0.707106 , y: -1.000000 , z: -0.707108 },
                    Vertex { x: -0.555569 , y: -1.000000 , z: -0.831470 },
                    Vertex { x: -0.382682 , y: -1.000000 , z: -0.923880 },
                    Vertex { x: -0.195089 , y: -1.000000 , z: -0.980786 },
                ],
                tex_vertices: vec!(),
                normals : vec![
                    Normal { x: -0.259887 , y: 0.445488 , z: -0.856737 },
                    Normal { x: 0.087754 , y: 0.445488 , z: -0.890977 },
                    Normal { x: -0.422035 , y: 0.445488 , z: -0.789574 },
                    Normal { x: -0.567964 , y: 0.445488 , z: -0.692068 },
                    Normal { x: -0.692066 , y: 0.445488 , z: -0.567966 },
                    Normal { x: -0.789573 , y: 0.445488 , z: -0.422037 },
                    Normal { x: -0.856737 , y: 0.445488 , z: -0.259889 },
                    Normal { x: -0.890977 , y: 0.445488 , z: -0.087754 },
                    Normal { x: -0.890977 , y: 0.445488 , z: 0.087753 },
                    Normal { x: -0.856737 , y: 0.445488 , z: 0.259887 },
                    Normal { x: -0.789574 , y: 0.445488 , z: 0.422035 },
                    Normal { x: -0.692067 , y: 0.445488 , z: 0.567964 },
                    Normal { x: -0.567965 , y: 0.445488 , z: 0.692066 },
                    Normal { x: -0.422036 , y: 0.445488 , z: 0.789573 },
                    Normal { x: -0.259889 , y: 0.445488 , z: 0.856737 },
                    Normal { x: -0.087754 , y: 0.445488 , z: 0.890977 },
                    Normal { x: 0.087753 , y: 0.445488 , z: 0.890977 },
                    Normal { x: 0.259888 , y: 0.445488 , z: 0.856737 },
                    Normal { x: 0.422036 , y: 0.445488 , z: 0.789573 },
                    Normal { x: 0.567965 , y: 0.445488 , z: 0.692067 },
                    Normal { x: 0.692067 , y: 0.445488 , z: 0.567965 },
                    Normal { x: 0.789573 , y: 0.445488 , z: 0.422035 },
                    Normal { x: 0.856737 , y: 0.445488 , z: 0.259888 },
                    Normal { x: 0.890977 , y: 0.445488 , z: 0.087753 },
                    Normal { x: 0.890977 , y: 0.445488 , z: -0.087754 },
                    Normal { x: 0.856737 , y: 0.445488 , z: -0.259888 },
                    Normal { x: 0.789573 , y: 0.445488 , z: -0.422036 },
                    Normal { x: 0.692067 , y: 0.445488 , z: -0.567965 },
                    Normal { x: 0.567965 , y: 0.445488 , z: -0.692067 },
                    Normal { x: 0.422036 , y: 0.445488 , z: -0.789573 },
                    Normal { x: -0.087753 , y: 0.445488 , z: -0.890977 },
                    Normal { x: 0.259888 , y: 0.445488 , z: -0.856737 },
                    Normal { x: 0.000000 , y: -1.000000 , z: -0.000000 },
                ],
                geometry: vec![
                    Geometry {
                        material_name: Some("Material.002".into_string()),
                        smooth_shading_group: 0,
                        shapes: vec![
                            Triangle( (32, None, Some(0))  , (31, None, Some(0))  ,  (1, None, Some(0))  ),
                            Triangle( (2, None, Some(1))   , (0, None, Some(1))   ,  (1, None, Some(1))  ),
                            Triangle( (31, None, Some(2))  , (30, None, Some(2))  ,  (1, None, Some(2))  ),
                            Triangle( (30, None, Some(3))  , (29, None, Some(3))  ,  (1, None, Some(3))  ),
                            Triangle( (29, None, Some(4))  , (28, None, Some(4))  ,  (1, None, Some(4))  ),
                            Triangle( (28, None, Some(5))  , (27, None, Some(5))  ,  (1, None, Some(5))  ),
                            Triangle( (27, None, Some(6))  , (26, None, Some(6))  ,  (1, None, Some(6))  ),
                            Triangle( (26, None, Some(7))  , (25, None, Some(7))  ,  (1, None, Some(7))  ),
                            Triangle( (25, None, Some(8))  , (24, None, Some(8))  ,  (1, None, Some(8))  ),
                            Triangle( (24, None, Some(9))  , (23, None, Some(9))  ,  (1, None, Some(9))  ),
                            Triangle( (23, None, Some(10)) , (22, None, Some(10)) ,  (1, None, Some(10)) ),
                            Triangle( (22, None, Some(11)) , (21, None, Some(11)) ,  (1, None, Some(11)) ),
                            Triangle( (21, None, Some(12)) , (20, None, Some(12)) ,  (1, None, Some(12)) ),
                            Triangle( (20, None, Some(13)) , (19, None, Some(13)) ,  (1, None, Some(13)) ),
                            Triangle( (19, None, Some(14)) , (18, None, Some(14)) ,  (1, None, Some(14)) ),
                            Triangle( (18, None, Some(15)) , (17, None, Some(15)) ,  (1, None, Some(15)) ),
                            Triangle( (17, None, Some(16)) , (16, None, Some(16)) ,  (1, None, Some(16)) ),
                            Triangle( (16, None, Some(17)) , (15, None, Some(17)) ,  (1, None, Some(17)) ),
                            Triangle( (15, None, Some(18)) , (14, None, Some(18)) ,  (1, None, Some(18)) ),
                            Triangle( (14, None, Some(19)) , (13, None, Some(19)) ,  (1, None, Some(19)) ),
                            Triangle( (13, None, Some(20)) , (12, None, Some(20)) ,  (1, None, Some(20)) ),
                            Triangle( (12, None, Some(21)) , (11, None, Some(21)) ,  (1, None, Some(21)) ),
                            Triangle( (11, None, Some(22)) , (10, None, Some(22)) ,  (1, None, Some(22)) ),
                            Triangle( (10, None, Some(23)) , (9, None, Some(23))  ,  (1, None, Some(23)) ),
                            Triangle( (9, None, Some(24))  , (8, None, Some(24))  ,  (1, None, Some(24)) ),
                            Triangle( (8, None, Some(25))  , (7, None, Some(25))  ,  (1, None, Some(25)) ),
                            Triangle( (7, None, Some(26))  , (6, None, Some(26))  ,  (1, None, Some(26)) ),
                            Triangle( (6, None, Some(27))  , (5, None, Some(27))  ,  (1, None, Some(27)) ),
                            Triangle( (5, None, Some(28))  , (4, None, Some(28))  ,  (1, None, Some(28)) ),
                            Triangle( (4, None, Some(29))  , (3, None, Some(29))  ,  (1, None, Some(29)) ),
                            Triangle( (0, None, Some(30))  , (32, None, Some(30)) ,  (1, None, Some(30)) ),
                            Triangle( (3, None, Some(31))  , (2, None, Some(31))  ,  (1, None, Some(31)) ),
                        ]
                    }
                ]
            }
        ]
    });

    expect("dome.obj").to_be(ObjSet {
        material_library: "dome.mtl".into_string(),
        objects: vec![
            Object {
                name: "Dome".into_string(),
                vertices: vec![
                    Vertex { x: -0.382683, y: 0.92388, z: 0.0 },
                    Vertex { x: -0.707107, y: 0.707107, z: 0.0 },
                    Vertex { x: -0.92388, y: 0.382683, z: 0.0 },
                    Vertex { x: -1.0, y: 0.0, z: 0.0 },
                    Vertex { x: -0.270598, y: 0.92388, z: -0.270598 },
                    Vertex { x: -0.5, y: 0.707107, z: -0.5 },
                    Vertex { x: -0.653282, y: 0.382683, z: -0.653281 },
                    Vertex { x: -0.707107, y: 0.0, z: -0.707107 },
                    Vertex { x: 0.0, y: 0.92388, z: -0.382683 },
                    Vertex { x: 0.0, y: 0.707107, z: -0.707107 },
                    Vertex { x: 0.0, y: 0.382683, z: -0.923879 },
                    Vertex { x: 0.0, y: 0.0, z: -1.0 },
                    Vertex { x: 0.0, y: 1.0, z: 0.0 },
                    Vertex { x: 0.270598, y: 0.92388, z: -0.270598 },
                    Vertex { x: 0.5, y: 0.707107, z: -0.5 },
                    Vertex { x: 0.653281, y: 0.382683, z: -0.653281 },
                    Vertex { x: 0.707106, y: 0.0, z: -0.707107 },
                    Vertex { x: 0.382683, y: 0.92388, z: 0.0 },
                    Vertex { x: 0.707106, y: 0.707107, z: 0.0 },
                    Vertex { x: 0.923879, y: 0.382683, z: 0.0 },
                    Vertex { x: 1.0, y: 0.0, z: 0.0 },
                    Vertex { x: 0.270598, y: 0.92388, z: 0.270598 },
                    Vertex { x: 0.5, y: 0.707107, z: 0.5 },
                    Vertex { x: 0.653281, y: 0.382683, z: 0.653281 },
                    Vertex { x: 0.707106, y: 0.0, z: 0.707107 },
                    Vertex { x: 0.0, y: 0.92388, z: 0.382683 },
                    Vertex { x: 0.0, y: 0.707107, z: 0.707107 },
                    Vertex { x: 0.0, y: 0.382683, z: 0.923879 },
                    Vertex { x: 0.0, y: 0.0, z: 1.0 },
                    Vertex { x: -0.270598, y: 0.92388, z: 0.270598 },
                    Vertex { x: -0.5, y: 0.707107, z: 0.5 },
                    Vertex { x: -0.653281, y: 0.382683, z: 0.653281 },
                    Vertex { x: -0.707107, y: 0.0, z: 0.707107 },
                ],
                tex_vertices: vec![],
                normals: vec![],
                geometry: vec![
                    Geometry {
                        material_name: Some("None".into_string()),
                        smooth_shading_group: 1,
                        shapes: vec![
                            Triangle((6, None, None), (3, None, None), (2, None, None)),
                            Triangle((5, None, None), (2, None, None), (1, None, None)),
                            Triangle((5, None, None), (0, None, None), (4, None, None)),
                            Triangle((11, None, None), (6, None, None), (10, None, None)),
                            Triangle((10, None, None), (5, None, None), (9, None, None)),
                            Triangle((9, None, None), (4, None, None), (8, None, None)),
                            Triangle((16, None, None), (10, None, None), (15, None, None)),
                            Triangle((14, None, None), (10, None, None), (9, None, None)),
                            Triangle((13, None, None), (9, None, None), (8, None, None)),
                            Triangle((20, None, None), (15, None, None), (19, None, None)),
                            Triangle((19, None, None), (14, None, None), (18, None, None)),
                            Triangle((18, None, None), (13, None, None), (17, None, None)),
                            Triangle((24, None, None), (19, None, None), (23, None, None)),
                            Triangle((22, None, None), (19, None, None), (18, None, None)),
                            Triangle((22, None, None), (17, None, None), (21, None, None)),
                            Triangle((28, None, None), (23, None, None), (27, None, None)),
                            Triangle((26, None, None), (23, None, None), (22, None, None)),
                            Triangle((25, None, None), (22, None, None), (21, None, None)),
                            Triangle((32, None, None), (27, None, None), (31, None, None)),
                            Triangle((30, None, None), (27, None, None), (26, None, None)),
                            Triangle((29, None, None), (26, None, None), (25, None, None)),
                            Triangle((4, None, None), (0, None, None), (12, None, None)),
                            Triangle((8, None, None), (4, None, None), (12, None, None)),
                            Triangle((13, None, None), (8, None, None), (12, None, None)),
                            Triangle((17, None, None), (13, None, None), (12, None, None)),
                            Triangle((21, None, None), (17, None, None), (12, None, None)),
                            Triangle((25, None, None), (21, None, None), (12, None, None)),
                            Triangle((29, None, None), (25, None, None), (12, None, None)),
                            Triangle((3, None, None), (31, None, None), (2, None, None)),
                            Triangle((2, None, None), (30, None, None), (1, None, None)),
                            Triangle((1, None, None), (29, None, None), (0, None, None)),
                            Triangle((0, None, None), (29, None, None), (12, None, None)),
                            Triangle((6, None, None), (7, None, None), (3, None, None)),
                            Triangle((5, None, None), (6, None, None), (2, None, None)),
                            Triangle((5, None, None), (1, None, None), (0, None, None)),
                            Triangle((11, None, None), (7, None, None), (6, None, None)),
                            Triangle((10, None, None), (6, None, None), (5, None, None)),
                            Triangle((9, None, None), (5, None, None), (4, None, None)),
                            Triangle((16, None, None), (11, None, None), (10, None, None)),
                            Triangle((14, None, None), (15, None, None), (10, None, None)),
                            Triangle((13, None, None), (14, None, None), (9, None, None)),
                            Triangle((20, None, None), (16, None, None), (15, None, None)),
                            Triangle((19, None, None), (15, None, None), (14, None, None)),
                            Triangle((18, None, None), (14, None, None), (13, None, None)),
                            Triangle((24, None, None), (20, None, None), (19, None, None)),
                            Triangle((22, None, None), (23, None, None), (19, None, None)),
                            Triangle((22, None, None), (18, None, None), (17, None, None)),
                            Triangle((28, None, None), (24, None, None), (23, None, None)),
                            Triangle((26, None, None), (27, None, None), (23, None, None)),
                            Triangle((25, None, None), (26, None, None), (22, None, None)),
                            Triangle((32, None, None), (28, None, None), (27, None, None)),
                            Triangle((30, None, None), (31, None, None), (27, None, None)),
                            Triangle((29, None, None), (30, None, None), (26, None, None)),
                            Triangle((3, None, None), (32, None, None), (31, None, None)),
                            Triangle((2, None, None), (31, None, None), (30, None, None)),
                            Triangle((1, None, None), (30, None, None), (29, None, None)),
                        ]
                    },
                    Geometry {
                        material_name: Some("None".into_string()),
                        smooth_shading_group: 2,
                        shapes: vec![
                            Triangle((7, None, None), (32, None, None), (3, None, None)),
                            Triangle((24, None, None), (28, None, None), (32, None, None)),
                            Triangle((20, None, None), (11, None, None), (16, None, None)),
                            Triangle((7, None, None), (11, None, None), (32, None, None)),
                            Triangle((24, None, None), (32, None, None), (20, None, None)),
                            Triangle((11, None, None), (20, None, None), (32, None, None)),
                        ]
                    }
                ]
            }
        ]
    });
}


//
// Test helpers below
//
fn expect(filename: &str) -> Actual {
    let path = Path::new("tests").join("fixtures").join(filename);
    let file = File::open(&path).unwrap();
    let mut reader = BufferedReader::new(file);
    let buf = reader.read_to_end().unwrap();
    let content = String::from_utf8(buf).unwrap();
    let obj_set = parse(content).unwrap();
    Actual { actual: obj_set }
}

struct Actual {
    actual: ObjSet,
}

impl Actual {
    fn to_be(&self, expectation: ObjSet) {
        assert_eq!(self.actual, expectation);
    }
}
