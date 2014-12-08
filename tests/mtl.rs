extern crate obj;
extern crate bear;

use obj::mtl::{parse, MtlSet, Material, Color};
use obj::mtl::Illumination::AmbientDiffuseSpecular;

#[test]
fn test_parse() {
    fn f(filename: &str) -> MtlSet {
        obj::mtl::parse(bear::fixture(filename)).unwrap()
    }

    assert_eq!(f("cube.mtl"), MtlSet {
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

    assert_eq!(f("untitled.mtl"), MtlSet {
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
