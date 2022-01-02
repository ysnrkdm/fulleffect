use crate::camera::Ray;
use crate::config;
use crate::material::SampleResult;
use crate::vector::Vector3;

fn get_tangent_space_basis_gram_schmidtd(normal: &Vector3) -> (Vector3, Vector3) {
    let up = if normal.x.abs() > config::EPS {
        Vector3::new(0.0, 1.0, 0.0)
    } else {
        Vector3::new(1.0, 0.0, 0.0)
    };
    let tangent = up.cross(&normal).normalized();
    let binormal = normal.cross(&tangent);
    (tangent, binormal)
}

// https://github.com/githole/edupt/blob/master/radiance.h
pub fn importance_sample_diffuse(random: (f64, f64), normal: &Vector3) -> Vector3 {
    let (tangent, binormal) = get_tangent_space_basis_gram_schmidtd(normal);
    let phi = config::PI2 * random.0;
    (tangent * phi.cos() + binormal * phi.sin()) * random.1.sqrt()
        + *normal * (1.0 - random.1).sqrt()
}

pub fn sample_refraction(
    random: (f64, f64),
    position: &Vector3,
    view: &Vector3,
    normal: &Vector3,
    refractive_index: f64,
) -> Option<SampleResult> {
    let is_incoming = view.dot(&normal).is_sign_negative();
    let oriented_normal = if is_incoming { *normal } else { -*normal };
    let nnt = if is_incoming {
        refractive_index.recip()
    } else {
        refractive_index
    };
    let reflect_direction = view.reflect(&oriented_normal);
    let refract_direction = view.refract(&oriented_normal, nnt);
    if refract_direction == Vector3::zero() {
        // total reflection
        Some(SampleResult {
            ray: Ray {
                origin: *position + config::OFFSET * oriented_normal,
                direction: reflect_direction,
            },
            reflectance: 1.0,
        })
    } else {
        // fresnel reflection r
        // i: incident angle, t: refraction angle, r_s: S wave rate, r_p: P wave rate, fr: fresnel reflection rate
        let cos_i = view.dot(&-oriented_normal);
        //float cos_t = sqrt(1.0 - nnt * nnt * (1.0 - cos_i * cos_i));
        let cos_t = refract_direction.dot(&-oriented_normal);
        let r_s = (nnt * cos_i - cos_t) * (nnt * cos_i - cos_t)
            / ((nnt * cos_i + cos_t) * (nnt * cos_i + cos_t));
        let r_p = (nnt * cos_t - cos_i) * (nnt * cos_t - cos_i)
            / ((nnt * cos_t + cos_i) * (nnt * cos_t + cos_i));
        let fr = 0.5 * (r_s + r_p);

        if random.0 <= fr {
            // reflection
            Some(SampleResult {
                ray: Ray {
                    origin: *position + config::OFFSET * oriented_normal,
                    direction: reflect_direction,
                },
                reflectance: 1.0,
            })
        } else {
            // refraction
            Some(SampleResult {
                ray: Ray {
                    origin: *position - config::OFFSET * oriented_normal,
                    direction: refract_direction,
                },
                reflectance: nnt * nnt,
            })
        }
    }
}
