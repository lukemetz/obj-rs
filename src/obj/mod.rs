//! A parser for Wavefront's `.obj` file format for storing 3D meshes.
use std::iter;
use std::mem;
use std::num::Float;
use std::result::{Result,Ok,Err};

use lex::{ParseError,Lexer};

#[cfg(test)]
mod tests;

/// A set of objects, as listed in an `.obj` file.
#[deriving(Clone, Show, PartialEq)]
pub struct ObjSet {
    /// Which material library to use.
    pub material_library: String,
    /// The set of objects.
    pub objects: Vec<Object>,
}

/// A mesh object.
#[deriving(Clone, Show, PartialEq)]
pub struct Object {
    /// A human-readable name for this object. This can be set in blender.
    pub name: String,
    /// The set of vertices this object is composed of. These are referenced
    /// by index in `faces`.
    pub vertices: Vec<Vertex>,
    /// The set of texture vertices referenced by this object. The actual
    /// vertices are indexed by the second element in a `VTIndex`.
    pub tex_vertices: Vec<TVertex>,
    /// The set of normals referenced by this object. This are are referenced
    /// by the third element in a `VTIndex`.
    pub normals: Vec<Normal>,
    /// A set of shapes (with materials applied to them) of which this object is
    /// composed.
    pub geometry: Vec<Geometry>,
}

/// A set of shapes, all using the given material.
#[deriving(Clone, Show, PartialEq)]
pub struct Geometry {
    /// A reference to the material to apply to this geometry.
    pub material_name: Option<String>,
    /// Should we use smooth shading when rendering this?
    pub smooth_shading_group: uint,
    /// The shapes of which this geometry is composed.
    pub shapes: Vec<Shape>,
}

/// The various shapes supported by this library.
///
/// Convex polygons more complicated than a triangle are automatically
/// converted into triangles.
#[deriving(Clone, Show, Hash, PartialEq)]
pub enum Shape {
    /// A point specified by its position.
    Point(VTIndex),
    /// A line specified by its endpoints.
    Line(VTIndex, VTIndex),
    /// A triangle specified by its three vertices.
    Triangle(VTIndex, VTIndex, VTIndex),
}

/// A single 3-dimensional point on the corner of an object.
#[allow(missing_docs)]
#[deriving(Clone, Copy, Show)]
pub struct Vertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// A single 3-dimensional normal
pub type Normal = Vertex;

/// A single 2-dimensional point on a texture. "Texure Vertex".
#[allow(missing_docs)]
#[deriving(Clone, Copy, Show)]
pub struct TVertex {
    pub x: f64,
    pub y: f64,
}

fn fuzzy_cmp(a: f64, b: f64, delta: f64) -> Ordering {
    if (a - b).abs() <= delta {
        Equal
    } else if a < b {
        Less
    } else {
        Greater
    }
}

// TODO(cgaebel): Can we implement Eq here?
impl PartialEq for Vertex {
    fn eq(&self, other: &Vertex) -> bool {
        self.partial_cmp(other).unwrap() == Equal
    }
}

impl PartialOrd for Vertex {
    fn partial_cmp(&self, other: &Vertex) -> Option<Ordering> {
        Some(fuzzy_cmp(self.x, other.x, 0.00001)
            .cmp(&fuzzy_cmp(self.y, other.y, 0.00001))
            .cmp(&fuzzy_cmp(self.z, other.z, 0.00001)))
    }
}

impl PartialEq for TVertex {
    fn eq(&self, other: &TVertex) -> bool {
        self.partial_cmp(other).unwrap() == Equal
    }
}

impl PartialOrd for TVertex {
    fn partial_cmp(&self, other: &TVertex) -> Option<Ordering> {
        Some(fuzzy_cmp(self.x, other.x, 0.00001)
            .cmp(&fuzzy_cmp(self.y, other.y, 0.00001)))
    }
}

/// An index into the `vertices` array of an object, representing a vertex in
/// the mesh. After parsing, this is guaranteed to be a valid index into the
/// array, so unchecked indexing may be used.
pub type VertexIndex = uint;

/// An index into the `texture vertex` array of an object.
///
/// Unchecked indexing may be used, because the values are guaranteed to be in
/// range by the parser.
pub type TextureIndex = uint;

/// An index into the `normals` array of an object.
///
/// Unchecked indexing may be used, because the values are guaranteed to be in
/// range by the parser.
pub type NormalIndex = uint;

/// An index into the vertex array, with an optional index into the texture
/// array. This is used to define the corners of shapes which may or may not
/// be textured.
pub type VTIndex = (VertexIndex, Option<TextureIndex>, Option<NormalIndex>);

/// Slices the underlying string in an option.
fn sliced<'a>(s: &'a Option<String>) -> Option<&'a str> {
    match *s {
        None => None,
        Some(ref s) => Some(s.as_slice()),
    }
}

/// Blender exports shapes as a list of the vertices representing their corners.
/// This function turns that into a set of OpenGL-usable shapes - i.e. points,
/// lines, or triangles.
fn to_triangles(xs: &[VTIndex]) -> Vec<Shape> {
    match xs.len() {
        0 => return vec!(),
        1 => return vec!(Shape::Point(xs[0])),
        2 => return vec!(Shape::Line(xs[0], xs[1])),
        _ => {},
    }

    let last_elem = *xs.last().unwrap();

    xs.slice_to(xs.len()-1)
        .iter()
        .zip(xs.slice(1, xs.len()-1).iter())
        .map(|(&x, &y)| Shape::Triangle(last_elem, x, y))
        .collect()
}

struct Parser<'a> {
    line_number: uint,
    lexer: iter::Peekable<String, Lexer<'a>>,
}

impl<'a> Parser<'a> {
    fn new<'a>(input: &'a str) -> Parser<'a> {
        Parser {
            line_number: 1,
            lexer: Lexer::new(input).peekable(),
        }
    }

    fn error<A>(&self, msg: String) -> Result<A, ParseError> {
        Err(ParseError {
            line_number: self.line_number,
            message:     msg,
        })
    }

    fn next(&mut self) -> Option<String> {
        // TODO(cgaebel): This has a lot of useless allocations. Techincally we can
        // just be using slices into the underlying buffer instead of allocating a
        // new string for every single token. Unfortunately, I'm not sure how to
        // structure this to appease the borrow checker.
        let ret = self.lexer.next();

        match ret {
            None => {},
            Some(ref s) =>
                if s.as_slice() == "\n" {
                    self.line_number += 1;
                },
        }

        ret
    }

    fn advance(&mut self) {
        self.next();
    }

    fn peek(&mut self) -> Option<String> {
        // TODO(cgaebel): See the comment in `next`.
        self.lexer.peek().map(|s| s.clone())
    }

    /// Possibly skips over some newlines.
    fn zero_or_more_newlines(&mut self) {
        loop {
            match sliced(&self.peek()) {
                None       => break,
                Some("\n") => {},
                Some(_)    => break,
            }
            self.advance();
        }
    }

    /// Parse just a constant string.
    fn parse_tag(&mut self, tag: &str) -> Result<(), ParseError> {
        match sliced(&self.next()) {
            None =>
                return self.error(format!("Expected `{}` but got end of input.", tag)),
            Some(s) => {
                if s != tag {
                    return self.error(format!("Expected `{}` but got {}.", tag, s));
                }
            }
        }

        return Ok(())
    }

    /// Skips over some newlines, failing if it didn't manage to skip any.
    fn one_or_more_newlines(&mut self) -> Result<(), ParseError> {
        try!(self.parse_tag("\n"));
        self.zero_or_more_newlines();
        Ok(())
    }

    fn parse_str(&mut self) -> Result<String, ParseError> {
        match self.next() {
            None =>
                self.error(format!("Expected string but got end of input.")),
            Some(got) => {
                if got.as_slice() == "\n" {
                    self.error(format!("Expected string but got `end of line`."))
                } else {
                    Ok(got)
                }
            }
        }
    }

    fn parse_material_library<'a>(&mut self) -> Result<String, ParseError> {
        try!(self.parse_tag("mtllib"));
        self.parse_str()
    }

    fn parse_object_name(&mut self) -> Result<String, ParseError> {
        try!(self.parse_tag("o"));
        self.parse_str()
    }

    // TODO(cgaebel): Should this be returning `num::rational::BigRational` instead?
    // I can't think of a good reason to do this except to make testing easier.
    fn parse_double(&mut self) -> Result<f64, ParseError> {
        let s = try!(self.parse_str());

        match from_str::<f64>(s.as_slice()) {
            None =>
                self.error(format!("Expected f64 but got {}.", s)),
            Some(ret) =>
                Ok(ret)
        }
    }

    fn parse_vertex(&mut self) -> Result<Vertex, ParseError> {
        try!(self.parse_tag("v"));

        let x = try!(self.parse_double());
        let y = try!(self.parse_double());
        let z = try!(self.parse_double());

        Ok(Vertex { x: x, y: y, z: z })
    }

    /// BUG: Also munches trailing whitespace.
    fn parse_vertices(&mut self) -> Result<Vec<Vertex>, ParseError> {
        let mut result = Vec::new();

        loop {
            match sliced(&self.peek()) {
                Some("v") => {
                    let v = try!(self.parse_vertex());
                    result.push(v);
                },
                _ => break,
            }

            try!(self.one_or_more_newlines());
        }

        Ok(result)
    }

    fn parse_tex_vertex(&mut self) -> Result<TVertex, ParseError> {
        try!(self.parse_tag("vt"));

        let x = try!(self.parse_double());
        let y = try!(self.parse_double());

        Ok(TVertex { x: x, y: y })
    }

    /// BUG: Also munches trailing whitespace.
    fn parse_tex_vertices(&mut self) -> Result<Vec<TVertex>, ParseError> {
        let mut result = Vec::new();

        loop {
            match sliced(&self.peek()) {
                Some("vt") => {
                    let v = try!(self.parse_tex_vertex());
                    result.push(v);
                },
                _ => break,
            }

            try!(self.one_or_more_newlines());
        }

        Ok(result)
    }

    fn parse_normal(&mut self) -> Result<Normal, ParseError> {
        try!(self.parse_tag("vn"));

        let x = try!(self.parse_double());
        let y = try!(self.parse_double());
        let z = try!(self.parse_double());

        Ok(Normal { x: x, y: y, z: z })
    }

    /// BUG: Also munches trailing whitespace.
    fn parse_normals(&mut self) -> Result<Vec<Vertex>, ParseError> {
        let mut result = Vec::new();

        loop {
            match sliced(&self.peek()) {
                Some("vn") => {
                    let vn = try!(self.parse_normal());
                    result.push(vn);
                },
                _ => break,
            }

            try!(self.one_or_more_newlines());
        }

        Ok(result)
    }

    fn parse_usemtl(&mut self) -> Result<String, ParseError> {
        try!(self.parse_tag("usemtl"));
        self.parse_str()
    }

    fn parse_smooth_shading(&mut self) -> Result<uint, ParseError> {
        try!(self.parse_tag("s"));

        match try!(self.parse_str()).as_slice() {
            "off" => Ok(0),
            s     => match from_str::<uint>(s) {
                Some(ret) => Ok(ret),
                None => self.error(format!("Expected an unsigned int or `off` but got {}.", s)),
            }
        }
    }

    fn parse_int_from(&mut self, s: &str) -> Result<int, ParseError> {
        match from_str::<int>(s) {
            None =>
                return self.error(format!("Expected int but got {}.", s)),
            Some(ret) =>
                Ok(ret)
        }
    }

    fn parse_vtindex(&mut self, valid_vtx: (uint, uint), valid_tx: (uint, uint),
                                    valid_nx: (uint, uint) ) -> Result<VTIndex, ParseError> {
        match sliced(&self.next()) {
            None =>
                return self.error("Expected vertex index but got end of input.".into_string()),
            Some(s) => {
                let splits: Vec<&str> = s.split('/').collect();
                assert!(splits.len() != 0);

                match splits.len() {
                    1 => {
                        let v_idx = try!(self.parse_int_from(splits[0]));
                        let v_idx = try!(self.check_valid_index(valid_vtx, v_idx));
                        Ok((v_idx, None, None))
                    },
                    2 => {
                        let v_idx = try!(self.parse_int_from(splits[0]));
                        let v_idx = try!(self.check_valid_index(valid_vtx, v_idx));
                        let t_idx = try!(self.parse_int_from(splits[1]));
                        let t_idx = try!(self.check_valid_index(valid_tx, t_idx));
                        Ok((v_idx, Some(t_idx), None))
                    },
                    3 => {
                        let v_idx = try!(self.parse_int_from(splits[0]));
                        let v_idx = try!(self.check_valid_index(valid_vtx, v_idx));
                        let t_idx_opt = if splits[1].len() == 0 {
                            None
                        } else {
                            let t_idx = try!(self.parse_int_from(splits[1]));
                            let t_idx = try!(self.check_valid_index(valid_tx, t_idx));
                            Some(t_idx)
                        };
                        let n_idx = try!(self.parse_int_from(splits[2]));
                        let n_idx = try!(self.check_valid_index(valid_nx, n_idx));
                        Ok((v_idx, t_idx_opt, Some(n_idx)))
                    },
                    n =>
                        self.error(format!("Expected at most 2 vertex indexes but got {}.", n)),
                }
            }
        }
    }

    /// `valid_values` is a range of valid bounds for the actual value.
    fn check_valid_index(&self, valid_values: (uint, uint), actual_value: int) -> Result<uint, ParseError> {
        let (min, max) = valid_values;

        let mut x = actual_value;

        // Handle negative vertex indexes.
        if x < 0 {
            x = max as int - x;
        }

        if x >= min as int && x < max as int {
            assert!(x > 0);
            Ok((x - min as int) as uint)
        } else {
            self.error(format!("Expected index in the range [{}, {}), but got {}.", min, max, actual_value))
        }
    }

    fn parse_face(&mut self, valid_vtx: (uint, uint), valid_tx: (uint, uint),
                              valid_nx: (uint, uint)) -> Result<Vec<Shape>, ParseError> {
        match sliced(&self.next()) {
            Some("f") => {},
            Some("l") => {},
            None      => return self.error("Expected `f` or `l` but got end of input.".into_string()),
            Some(s)   => return self.error(format!("Expected `f` or `l` but got {}.", s)),
        }

        let mut corner_list = Vec::new();

        corner_list.push(try!(self.parse_vtindex(valid_vtx, valid_tx, valid_nx)));

        loop {
            match sliced(&self.peek()) {
                None       => break,
                Some("\n") => break,
                Some( _  ) => corner_list.push(try!(self.parse_vtindex(valid_vtx, valid_tx, valid_nx))),
            }
        }

        Ok(to_triangles(corner_list.as_slice()))
    }

    fn parse_geometries(&mut self, valid_vtx: (uint, uint), valid_tx: (uint, uint),
                                          valid_nx: (uint, uint)) -> Result<Vec<Geometry>, ParseError> {
        let mut result = Vec::new();
        let mut shapes = Vec::new();

        let mut current_material   = None;
        let mut smooth_shading_group = 0;

        loop {
            match sliced(&self.peek()) {
                Some("usemtl") => {
                    let old_material =
                        mem::replace(
                            &mut current_material,
                            Some(try!(self.parse_usemtl())));

                    result.push(Geometry {
                        material_name:        old_material,
                        smooth_shading_group: smooth_shading_group,
                        shapes:               mem::replace(&mut shapes, Vec::new()),
                    });
                },
                Some("s") => {
                    let old_smooth_shading =
                        mem::replace(
                            &mut smooth_shading_group,
                            try!(self.parse_smooth_shading()));

                    result.push(Geometry {
                        material_name:        current_material.clone(),
                        smooth_shading_group: old_smooth_shading,
                        shapes:               mem::replace(&mut shapes, Vec::new()),
                    })
                },
                Some("f") | Some("l") => {
                    shapes.push_all(try!(self.parse_face(valid_vtx, valid_tx,
                                                                                              valid_nx)).as_slice());
                },
                _ => break,
            }

            try!(self.one_or_more_newlines());
        }

        result.push(Geometry {
            material_name:      current_material,
            smooth_shading_group: smooth_shading_group,
            shapes:             shapes,
        });

        Ok(result.into_iter().filter(|ref x| !x.shapes.is_empty()).collect())
    }

    fn parse_object(&mut self,
            min_vertex_index: &mut uint,
            max_vertex_index: &mut uint,
            min_tex_index:    &mut uint,
            max_tex_index:    &mut uint,
            min_normal_index: &mut uint,
            max_normal_index: &mut uint
            ) -> Result<Object, ParseError> {
        let name = try!(self.parse_object_name());
        try!(self.one_or_more_newlines());

        let vertices     = try!(self.parse_vertices());
        let tex_vertices = try!(self.parse_tex_vertices());
        let normals      = try!(self.parse_normals());

        *max_vertex_index += vertices.len();
        *max_tex_index    += tex_vertices.len();
        *max_normal_index += normals.len();

        let geometry =
            try!(self.parse_geometries(
                (*min_vertex_index, *max_vertex_index),
                (*min_tex_index, *max_tex_index),
                (*min_normal_index, *max_normal_index)));

        *min_vertex_index += vertices.len();
        *min_tex_index    += tex_vertices.len();
        *min_normal_index += normals.len();

        Ok(Object {
            name:          name,
            vertices:     vertices,
            tex_vertices: tex_vertices,
            normals:      normals,
            geometry:      geometry,
        })
    }

    fn parse_objects(&mut self) -> Result<Vec<Object>, ParseError> {
        let mut result = Vec::new();

        let mut min_vertex_index = 1;
        let mut max_vertex_index = 1;
        let mut min_tex_index    = 1;
        let mut max_tex_index    = 1;
        let mut min_normal_index = 1;
        let mut max_normal_index = 1;

        loop {
            match sliced(&self.peek()) {
                Some("o") => result.push(try!(self.parse_object(
                                            &mut min_vertex_index,
                                            &mut max_vertex_index,
                                            &mut min_tex_index,
                                            &mut max_tex_index,
                                            &mut min_normal_index,
                                            &mut max_normal_index))),
                _         => break,
            }
        }

        Ok(result)
    }

    fn parse_objset(&mut self) -> Result<ObjSet, ParseError> {
        self.zero_or_more_newlines();

        let material_library = try!(self.parse_material_library());
        try!(self.one_or_more_newlines());
        let objects          = try!(self.parse_objects());

        self.zero_or_more_newlines();

        match self.peek() {
            None =>
                {},
            Some(s) =>
                return self.error(format!("Expected end of input but got {}.", s)),
        }

        Ok(ObjSet {
            material_library: material_library,
            objects:          objects,
        })
    }
}


/// Parses a wavefront `.obj` file, returning either the successfully parsed
/// file, or an error. Support in this parser for the full file format is
/// best-effort and realistically I will only end up supporting the subset
/// of the file format which falls under the "things I see exported from blender"
/// category.
pub fn parse(mut input: String) -> Result<ObjSet, ParseError> {
    // Unfortunately, the parser requires a trailing newline. This is the easiest
    // way I could find to allow non-trailing newlines.
    input.push_str("\n");
    Parser::new(input.as_slice()).parse_objset()
}
