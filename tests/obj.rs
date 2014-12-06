extern crate obj;

use obj::obj::{Object, ObjSet, Vertex, TVertex, Normal, Geometry, parse};
use obj::obj::Shape::{Line, Triangle};

#[test]
fn test_parse() {
    let test_case =
r#"
# Blender v2.69 (sub 0) OBJ File: ''
# www.blender.org
mtllib untitled.mtl
o Cube.001
v -1.000000 -1.000000 1.000000
v -1.000000 -1.000000 -1.000000
v 1.000000 -1.000000 -1.000000
v 1.000000 -1.000000 1.000000
v -1.000000 1.000000 1.000000
v -1.000000 1.000000 -1.000000
v 1.000000 1.000000 -1.000000
v 1.000000 1.000000 1.000000
usemtl None
s off
f 5 6 2 1
f 6 7 3 2
f 7 8 4 3
f 8 5 1 4
f 1 2 3 4
f 8 7 6 5
o Circle
v 0.000000 0.000000 -1.000000
v -0.195090 0.000000 -0.980785
v -0.382683 0.000000 -0.923880
v -0.555570 0.000000 -0.831470
v -0.707107 0.000000 -0.707107
v -0.831470 0.000000 -0.555570
v -0.923880 0.000000 -0.382683
v -0.980785 0.000000 -0.195090
v -1.000000 0.000000 -0.000000
v -0.980785 0.000000 0.195090
v -0.923880 0.000000 0.382683
v -0.831470 0.000000 0.555570
v -0.707107 0.000000 0.707107
v -0.555570 0.000000 0.831470
v -0.382683 0.000000 0.923880
v -0.195090 0.000000 0.980785
v 0.000000 0.000000 1.000000
v 0.195091 0.000000 0.980785
v 0.382684 0.000000 0.923879
v 0.555571 0.000000 0.831469
v 0.707107 0.000000 0.707106
v 0.831470 0.000000 0.555570
v 0.923880 0.000000 0.382683
v 0.980785 0.000000 0.195089
v 1.000000 0.000000 -0.000001
v 0.980785 0.000000 -0.195091
v 0.923879 0.000000 -0.382684
v 0.831469 0.000000 -0.555571
v 0.707106 0.000000 -0.707108
v 0.555569 0.000000 -0.831470
v 0.382682 0.000000 -0.923880
v 0.195089 0.000000 -0.980786
l 10 9
l 11 10
l 12 11
l 13 12
l 14 13
l 15 14
l 16 15
l 17 16
l 18 17
l 19 18
l 20 19
l 21 20
l 22 21
l 23 22
l 24 23
l 25 24
l 26 25
l 27 26
l 28 27
l 29 28
l 30 29
l 31 30
l 32 31
l 33 32
l 34 33
l 35 34
l 36 35
l 37 36
l 38 37
l 39 38
l 40 39
l 9 40
o Cube
v 1.000000 -1.000000 -1.000000
v 1.000000 -1.000000 1.000000
v -1.000000 -1.000000 1.000000
v -1.000000 -1.000000 -1.000000
v 1.000000 1.000000 -0.999999
v 0.999999 1.000000 1.000001
v -1.000000 1.000000 1.000000
v -1.000000 1.000000 -1.000000
usemtl Material
s off
f 41 42 43 44
f 45 48 47 46
f 41 45 46 42
f 42 46 47 43
f 43 47 48 44
f 45 41 44 48
"#;

    let expected = ObjSet {
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
    };

    assert_eq!(parse(test_case.into_string()).unwrap(), expected);
}

#[test]
fn test_cube() {
    let test_case =
r#"
# Blender v2.71 (sub 0) OBJ File: 'cube.blend'
# www.blender.org
mtllib cube.mtl
o Cube
v 1.000000 -1.000000 -1.000000
v 1.000000 -1.000000 1.000000
v -1.000000 -1.000000 1.000000
v -1.000000 -1.000000 -1.000000
v 1.000000 1.000000 -0.999999
v 0.999999 1.000000 1.000001
v -1.000000 1.000000 1.000000
v -1.000000 1.000000 -1.000000
vt 1.004952 0.498633
vt 0.754996 0.498236
vt 0.755393 0.248279
vt 1.005349 0.248677
vt 0.255083 0.497442
vt 0.255480 0.247485
vt 0.505437 0.247882
vt 0.505039 0.497839
vt 0.754598 0.748193
vt 0.504642 0.747795
vt 0.505834 -0.002074
vt 0.755790 -0.001677
vt 0.005127 0.497044
vt 0.005524 0.247088
usemtl Material
s off
f 1/1 2/2 3/3 4/4
f 5/5 8/6 7/7 6/8
f 1/9 5/10 6/8 2/2
f 2/2 6/8 7/7 3/3
f 3/3 7/7 8/11 4/12
f 5/5 1/13 4/14 8/6
"#;

    let expected = ObjSet {
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
    };

    assert_eq!(parse(test_case.into_string()).unwrap(), expected);
}

#[test]
fn test_normals_no_tex() {
    let test_case =
r#"
# Blender v2.70 (sub 4) OBJ File: ''
# www.blender.org
mtllib normal-cone.mtl
o Cone
v 0.000000  -1.000000 -1.000000
v 0.000000   1.000000  0.000000
v 0.195090  -1.000000 -0.980785
v 0.382683  -1.000000 -0.923880
v 0.555570  -1.000000 -0.831470
v 0.707107  -1.000000 -0.707107
v 0.831470  -1.000000 -0.555570
v 0.923880  -1.000000 -0.382683
v 0.980785  -1.000000 -0.195090
v 1.000000  -1.000000 -0.000000
v 0.980785  -1.000000  0.195090
v 0.923880  -1.000000  0.382683
v 0.831470  -1.000000  0.555570
v 0.707107  -1.000000  0.707107
v 0.555570  -1.000000  0.831470
v 0.382683  -1.000000  0.923880
v 0.195090  -1.000000  0.980785
v -0.000000 -1.000000  1.000000
v -0.195091 -1.000000  0.980785
v -0.382684 -1.000000  0.923879
v -0.555571 -1.000000  0.831469
v -0.707107 -1.000000  0.707106
v -0.831470 -1.000000  0.555570
v -0.923880 -1.000000  0.382683
v -0.980785 -1.000000  0.195089
v -1.000000 -1.000000 -0.000001
v -0.980785 -1.000000 -0.195091
v -0.923879 -1.000000 -0.382684
v -0.831469 -1.000000 -0.555571
v -0.707106 -1.000000 -0.707108
v -0.555569 -1.000000 -0.831470
v -0.382682 -1.000000 -0.923880
v -0.195089 -1.000000 -0.980786
vn -0.259887 0.445488 -0.856737
vn 0.087754 0.445488 -0.890977
vn -0.422035 0.445488 -0.789574
vn -0.567964 0.445488 -0.692068
vn -0.692066 0.445488 -0.567966
vn -0.789573 0.445488 -0.422037
vn -0.856737 0.445488 -0.259889
vn -0.890977 0.445488 -0.087754
vn -0.890977 0.445488 0.087753
vn -0.856737 0.445488 0.259887
vn -0.789574 0.445488 0.422035
vn -0.692067 0.445488 0.567964
vn -0.567965 0.445488 0.692066
vn -0.422036 0.445488 0.789573
vn -0.259889 0.445488 0.856737
vn -0.087754 0.445488 0.890977
vn 0.087753 0.445488 0.890977
vn 0.259888 0.445488 0.856737
vn 0.422036 0.445488 0.789573
vn 0.567965 0.445488 0.692067
vn 0.692067 0.445488 0.567965
vn 0.789573 0.445488 0.422035
vn 0.856737 0.445488 0.259888
vn 0.890977 0.445488 0.087753
vn 0.890977 0.445488 -0.087754
vn 0.856737 0.445488 -0.259888
vn 0.789573 0.445488 -0.422036
vn 0.692067 0.445488 -0.567965
vn 0.567965 0.445488 -0.692067
vn 0.422036 0.445488 -0.789573
vn -0.087753 0.445488 -0.890977
vn 0.259888 0.445488 -0.856737
vn 0.000000 -1.000000 -0.000000
usemtl Material.002
s off
f 32//1 2//1 33//1
f 1//2 2//2 3//2
f 31//3 2//3 32//3
f 30//4 2//4 31//4
f 29//5 2//5 30//5
f 28//6 2//6 29//6
f 27//7 2//7 28//7
f 26//8 2//8 27//8
f 25//9 2//9 26//9
f 24//10 2//10 25//10
f 23//11 2//11 24//11
f 22//12 2//12 23//12
f 21//13 2//13 22//13
f 20//14 2//14 21//14
f 19//15 2//15 20//15
f 18//16 2//16 19//16
f 17//17 2//17 18//17
f 16//18 2//18 17//18
f 15//19 2//19 16//19
f 14//20 2//20 15//20
f 13//21 2//21 14//21
f 12//22 2//22 13//22
f 11//23 2//23 12//23
f 10//24 2//24 11//24
f 9//25 2//25 10//25
f 8//26 2//26 9//26
f 7//27 2//27 8//27
f 6//28 2//28 7//28
f 5//29 2//29 6//29
f 4//30 2//30 5//30
f 33//31 2//31 1//31
f 3//32 2//32 4//32
"#;

    let expected = ObjSet {
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
    };

    assert_eq!( parse(test_case.into_string()).unwrap(), expected);
}


#[test]
fn test_smooth_shading_groups() {
    let test_case =
r#"
# Blender v2.72 (sub 0) OBJ File: 'dome.blend'
# www.blender.org
mtllib dome.mtl
o Dome
v -0.382683 0.923880 0.000000
v -0.707107 0.707107 0.000000
v -0.923880 0.382683 0.000000
v -1.000000 -0.000000 0.000000
v -0.270598 0.923880 -0.270598
v -0.500000 0.707107 -0.500000
v -0.653282 0.382683 -0.653281
v -0.707107 -0.000000 -0.707107
v -0.000000 0.923880 -0.382683
v -0.000000 0.707107 -0.707107
v -0.000000 0.382683 -0.923879
v -0.000000 -0.000000 -1.000000
v -0.000000 1.000000 0.000000
v 0.270598 0.923880 -0.270598
v 0.500000 0.707107 -0.500000
v 0.653281 0.382683 -0.653281
v 0.707106 -0.000000 -0.707107
v 0.382683 0.923880 -0.000000
v 0.707106 0.707107 -0.000000
v 0.923879 0.382683 -0.000000
v 1.000000 -0.000000 -0.000000
v 0.270598 0.923880 0.270598
v 0.500000 0.707107 0.500000
v 0.653281 0.382683 0.653281
v 0.707106 -0.000000 0.707107
v -0.000000 0.923880 0.382683
v -0.000000 0.707107 0.707107
v -0.000000 0.382683 0.923879
v -0.000000 -0.000000 1.000000
v -0.270598 0.923880 0.270598
v -0.500000 0.707107 0.500000
v -0.653281 0.382683 0.653281
v -0.707107 -0.000000 0.707107
usemtl None
s 1
f 4 3 7
f 3 2 6
f 1 5 6
f 7 11 12
f 6 10 11
f 5 9 10
f 11 16 17
f 11 10 15
f 10 9 14
f 16 20 21
f 15 19 20
f 14 18 19
f 20 24 25
f 20 19 23
f 18 22 23
f 24 28 29
f 24 23 27
f 23 22 26
f 28 32 33
f 28 27 31
f 27 26 30
f 1 13 5
f 5 13 9
f 9 13 14
f 14 13 18
f 18 13 22
f 22 13 26
f 26 13 30
f 32 3 4
f 31 2 3
f 30 1 2
f 30 13 1
f 8 4 7
f 7 3 6
f 2 1 6
f 8 7 12
f 7 6 11
f 6 5 10
f 12 11 17
f 16 11 15
f 15 10 14
f 17 16 21
f 16 15 20
f 15 14 19
f 21 20 25
f 24 20 23
f 19 18 23
f 25 24 29
f 28 24 27
f 27 23 26
f 29 28 33
f 32 28 31
f 31 27 30
f 33 32 4
f 32 31 3
f 31 30 2
s 2
f 33 4 8
f 29 33 25
f 12 17 21
f 12 33 8
f 33 21 25
f 21 33 12
"#;

    let expected = ObjSet {
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
    };

    assert_eq!(parse(test_case.into_string()).unwrap(), expected);
}
