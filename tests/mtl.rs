extern crate obj;

use obj::mtl::{MtlSet, Material, Color, parse};
use obj::mtl::Illumination::AmbientDiffuseSpecular;

#[test]
fn test_cube() {
    let input = r#"
# Blender MTL File: 'cube.blend'
# Material Count: 1

newmtl Material
Ns 96.078431
Ka 0.000000 0.000000 0.000000
Kd 0.640000 0.640000 0.640000
Ks 0.500000 0.500000 0.500000
Ni 1.000000
d 1.000000
illum 2
map_Kd cube-uv-num.png
"#;

    let expected = MtlSet{
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
            }
        )
    };

    assert_eq!(parse(input.into_string()).unwrap(), expected);
}

#[test]
fn test_parse() {
    let input = r#"
# Blender MTL File: 'None'
# Material Count: 2

# name
newmtl Material
# Phong specular coefficient
Ns 96.078431
# ambient color (weighted)
Ka 0.000000 0.000000 0.000000
# diffuse color (weighted)
Kd 0.640000 0.640000 0.640000
# dissolve factor (weighted)
Ks 0.500000 0.500000 0.500000
# optical density (refraction)
Ni 1.000000
# alpha
d 1.000000
# illumination: 0=ambient, 1=ambient+diffuse, 2=ambient+diffuse+specular
illum 2

newmtl None
Ns 0
# ambient
Ka 0.000000 0.000000 0.000000
# diffuse
Kd 0.8 0.8 0.8
# specular
Ks 0.8 0.8 0.8
d 1
illum 2

"#;

    let expected = MtlSet {
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
            }
        )
    };

    assert_eq!(parse(input.into_string()).unwrap(), expected);
}
