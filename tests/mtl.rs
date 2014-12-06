extern crate obj;

use std::io::{File, BufferedReader};

use obj::mtl::{parse, MtlSet, Material, Color};
use obj::mtl::Illumination::AmbientDiffuseSpecular;

#[test]
fn test_parse() {
    expect("cube.mtl").to_be(MtlSet {
        materials: vec!(
            Material {
                name: "Material".into_string(),
                specular_coefficient: 96.078431,
                color_ambient:  Color { r: 0.0,  g: 0.0,  b: 0.0  },
                color_diffuse:  Color { r: 0.64, g: 0.64, b: 0.64 },
                color_specular: Color { r: 0.5,  g: 0.5,  b: 0.5  },
                optical_density: Some(1.0),
                alpha: 1.0,
                illumination: AmbientDiffuseSpecular,
                uv_map: Some("cube-uv-num.png".into_string()),
            },
        )
    });

    expect("untitled.mtl").to_be(MtlSet {
        materials: vec!(
            Material {
                name: "Material".into_string(),
                specular_coefficient: 96.078431,
                color_ambient:  Color { r: 0.0,  g: 0.0,  b: 0.0  },
                color_diffuse:  Color { r: 0.64, g: 0.64, b: 0.64 },
                color_specular: Color { r: 0.5,  g: 0.5,  b: 0.5  },
                optical_density: Some(1.0),
                alpha: 1.0,
                illumination: AmbientDiffuseSpecular,
                uv_map: None,
            },
            Material {
                name: "None".into_string(),
                specular_coefficient: 0.0,
                color_ambient:  Color { r: 0.0, g: 0.0, b: 0.0 },
                color_diffuse:  Color { r: 0.8, g: 0.8, b: 0.8 },
                color_specular: Color { r: 0.8, g: 0.8, b: 0.8 },
                optical_density: None,
                alpha: 1.0,
                illumination: AmbientDiffuseSpecular,
                uv_map: None,
            },
        )
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
    let mtl_set = parse(content).unwrap();
    Actual { actual: mtl_set }
}

struct Actual {
    actual: MtlSet,
}

impl Actual {
    fn to_be(&self, expectation: MtlSet) {
        assert_eq!(self.actual, expectation);
    }
}
