use crate::camera::Ray;
use crate::color::Color;
use crate::config;
use crate::material_utils::importance_sample_diffuse;
use crate::material_utils::sample_refraction;
use crate::math::saturate;
use crate::texture::Texture;
use crate::vector::Vector3;

#[derive(Clone, Debug)]
pub enum SurfaceType {
    Diffuse,
    Specular,
    Refraction { refractive_index: f64 },
    GGX { f0: f64 },
}

pub struct SampleResult {
    pub ray: Ray,

    pub reflectance: f64,
}

#[derive(Debug)]
pub struct Material {
    pub surface: SurfaceType,
    pub albedo: Texture,
    pub emission: Texture,
    pub roughness: Texture,
}

#[derive(Clone, Debug)]
pub struct PointMaterial {
    pub surface: SurfaceType,
    pub albedo: Color,
    pub emission: Color,
    pub roughness: f64,
}

impl PointMaterial {
    pub fn sample(
        &self,
        random: (f64, f64),
        position: &Vector3,
        view: &Vector3,
        normal: &Vector3,
    ) -> Option<SampleResult> {
        let ray = -*view;

        match self.surface {
            SurfaceType::Diffuse => Some(SampleResult {
                ray: Ray {
                    origin: *position + *normal * config::OFFSET,
                    direction: importance_sample_diffuse(random, normal),
                },
                reflectance: 1.0,
            }),
            SurfaceType::Specular => Some(SampleResult {
                ray: Ray {
                    origin: *position + *normal * config::OFFSET,
                    direction: ray.reflect(normal),
                },
                reflectance: 1.0,
            }),
            SurfaceType::Refraction { refractive_index } => {
                sample_refraction(random, position, view, normal, refractive_index)
            }
            SurfaceType::GGX { f0 } => {
                let alpha2 = roughness_to_alpha2(self.roughness);
                let half = importance_sample_ggx_half(random, normal, alpha2);
                let next_direction = ray.reflect(&half);

                // 半球外が選ばれた場合はBRDFを0にする
                let l_dot_n = next_direction.dot(normal);
                if l_dot_n.is_sign_negative() {
                    None
                } else {
                    let v_dot_n = view.dot(normal);
                    let v_dot_h = view.dot(&half);
                    let h_dot_n = half.dot(normal);

                    // G: Masking-Shadowing Fucntion
                    let g = g_smith_joint(l_dot_n, v_dot_n, alpha2);

                    // F: Fresnel term
                    let f = f_schlick_f64(v_dot_h, f0);

                    Some(SampleResult {
                        ray: Ray {
                            origin: *position + *normal * config::OFFSET,
                            direction: next_direction,
                        },
                        reflectance: f * saturate(g * v_dot_h / (h_dot_n * v_dot_n)),
                    })
                }
            }
        }
    }

    pub fn nee_available(&self) -> bool {
        match self.surface {
            SurfaceType::Specular => false,
            SurfaceType::Refraction {
                refractive_index: _,
            } => false,
            SurfaceType::Diffuse => true,
            SurfaceType::GGX { f0: _ } => true,
        }
    }

    pub fn bsdf(&self, view: &Vector3, normal: &Vector3, light: &Vector3) -> f64 {
        match self.surface {
            SurfaceType::Diffuse => config::PI.recip(),
            SurfaceType::Specular => unimplemented!(),
            SurfaceType::Refraction {
                refractive_index: _,
            } => unimplemented!(),
            SurfaceType::GGX { f0 } => {
                // https://qiita.com/_Pheema_/items/f1ffb2e38cc766e6e668
                // (https://schuttejoe.github.io/post/ggximportancesamplingpart1/
                //  i: view, g: light, m: half)

                let alpha2 = roughness_to_alpha2(self.roughness);
                let half = (*light + *view).normalized();

                let l_dot_n = light.dot(normal);
                if l_dot_n.is_sign_negative() {
                    return 0.0;
                }

                let v_dot_n = view.dot(normal);
                let v_dot_h = view.dot(&half);
                let h_dot_n = half.dot(normal);

                // D: Microfacet Distribution Functions GGX(Trowbridge-Reitz model)
                let tmp = 1.0 - (1.0 - alpha2) * h_dot_n * h_dot_n;
                let d = alpha2 / (config::PI * tmp * tmp);

                // G: Masking-Shadowing Fucntion
                let g = g_smith_joint(l_dot_n, v_dot_n, alpha2);

                // F: Fresnel term
                let f = f_schlick_f64(v_dot_h, f0);

                d * g * f / (4.0 * l_dot_n * v_dot_n)
            }
        }
    }
}

fn roughness_to_alpha2(roughness: f64) -> f64 {
    // There're multiple roughness -> alpha mapping methods.
    // Refer https://qiita.com/_Pheema_/items/f1ffb2e38cc766e6e668#表面の粗さαについて
    let alpha = roughness;
    alpha * alpha
}

fn g_smith_joint_lambda(x_dot_n: f64, alpha2: f64) -> f64 {
    let a = (x_dot_n * x_dot_n).recip() - 1.0;
    0.5 * (1.0 + alpha2 * a).sqrt() - 0.5
}

fn g_smith_joint(l_dot_n: f64, v_dot_n: f64, alpha2: f64) -> f64 {
    let lambda_l = g_smith_joint_lambda(l_dot_n, alpha2);
    let lambda_v = g_smith_joint_lambda(v_dot_n, alpha2);
    (1.0 + lambda_l + lambda_v).recip()
}

fn f_schlick_f64(v_dot_h: f64, f0: f64) -> f64 {
    f0 + (1.0 - f0) * (1.0 - v_dot_h).powi(5)
}

fn tangent_space_basis_gram_schmidtd(normal: &Vector3) -> (Vector3, Vector3) {
    let up = if normal.x.abs() > config::EPS {
        Vector3::new(0.0, 1.0, 0.0)
    } else {
        Vector3::new(1.0, 0.0, 0.0)
    };
    let tangent = up.cross(&normal).normalized();
    let binormal = normal.cross(&tangent);
    (tangent, binormal)
}

// ImportanceSampleGGX from UE4
// Refer to http://project-asura.com/blog/?p=3124
fn importance_sample_ggx_half(random: (f64, f64), normal: &Vector3, alpha2: f64) -> Vector3 {
    let (tangent, binormal) = tangent_space_basis_gram_schmidtd(normal);

    let phi = config::PI2 * random.0;
    let cos_theta = ((1.0 - random.1) / (1.0 + (alpha2 - 1.0) * random.1)).sqrt();
    let sin_theta = (1.0 - cos_theta * cos_theta).sqrt();

    let h = Vector3::new(sin_theta * phi.cos(), sin_theta * phi.sin(), cos_theta);
    tangent * h.x + binormal * h.y + *normal * h.z
}
