// Nama  : Ghaisan Khoirul Badruzaman
// NIM   : 251524048
// Kelas : 2B-D4
// Modul 2 - Task II: Jarak, Kesamaan Kosinus & Proyeksi

use nalgebra::Vector3;

const EPS: f64 = 1e-12;

pub fn jarak_euclidean(u: &Vector3<f64>, v: &Vector3<f64>) -> f64 {
    (u - v).norm()
}

pub fn jarak_manhattan(u: &Vector3<f64>, v: &Vector3<f64>) -> f64 {
    (u - v).lp_norm(1)
}

/// Kesamaan kosinus. None kalau salah satu vektor nol, karena sudutnya tidak terdefinisi.
pub fn cos_sim(u: &Vector3<f64>, v: &Vector3<f64>) -> Option<f64> {
    let d = u.norm() * v.norm();
    if d < EPS { None } else { Some(u.dot(v) / d) }
}

/// Proyeksi u ke arah v, dikembalikan bersama komponen tegak lurusnya.
/// None kalau v vektor nol (tidak punya arah untuk diproyeksikan).
pub fn proyeksi(u: &Vector3<f64>, v: &Vector3<f64>) -> Option<(Vector3<f64>, Vector3<f64>)> {
    let nn = v.norm_squared();
    if nn < EPS {
        return None;
    }
    let proj = (u.dot(v) / nn) * v;
    Some((proj, u - proj))
}

pub fn jalankan() {
    println!("=== Task II: Jarak, Kesamaan Kosinus & Proyeksi ===");

    let u = Vector3::new(1.0, 2.0, 3.0);
    let v = Vector3::new(4.0, 0.0, -1.0);
    println!(
        "u = {}, v = {}",
        crate::fmt_vec(u.iter()),
        crate::fmt_vec(v.iter())
    );
    println!("jarak Euclidean = {:.4}", jarak_euclidean(&u, &v));
    println!("jarak Manhattan = {:.4}", jarak_manhattan(&u, &v));

    match cos_sim(&u, &v) {
        Some(c) => println!(
            "cos(u, v) = {c:.4}, sudut = {:.4} derajat",
            c.acos().to_degrees()
        ),
        None => println!("cos(u, v) tidak terdefinisi"),
    }
    let nol = Vector3::zeros();
    println!(
        "cos(u, 0) = {:?}  (vektor nol tidak punya arah)",
        cos_sim(&u, &nol)
    );

    let (proj, tegak) = proyeksi(&u, &v).expect("v bukan vektor nol");
    println!("proj_v u   = {}", crate::fmt_vec(proj.iter()));
    println!("tegak      = {}", crate::fmt_vec(tegak.iter()));
    println!("tegak . v  = {:e}  (harus nol)", tegak.dot(&v));
    println!(
        "proj+tegak = {}  (kembali ke u)",
        crate::fmt_vec((proj + tegak).iter())
    );

    let unit = v.try_normalize(EPS).expect("vektor nol");
    println!(
        "unit v     = {}, ||unit v|| = {:.12}",
        crate::fmt_vec(unit.iter()),
        unit.norm()
    );
    println!("normalisasi vektor nol: {:?}", nol.try_normalize(EPS));
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn jarak_contoh() {
        let u: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
        let v: Vector3<f64> = Vector3::new(4.0, 6.0, 3.0);
        assert!((jarak_euclidean(&u, &v) - 5.0).abs() < EPS);
        assert!((jarak_manhattan(&u, &v) - 7.0).abs() < EPS);
    }

    #[test]
    fn kosinus_searah_tegak_lurus_berlawanan() {
        let u: Vector3<f64> = Vector3::new(1.0, 2.0, 2.0);
        assert!((cos_sim(&u, &(3.0 * u)).unwrap() - 1.0).abs() < EPS);
        assert!((cos_sim(&u, &(-u)).unwrap() + 1.0).abs() < EPS);
        let tegak: Vector3<f64> = Vector3::new(2.0, -1.0, 0.0);
        assert!(cos_sim(&u, &tegak).unwrap().abs() < EPS);
    }

    #[test]
    fn kosinus_vektor_nol_none() {
        let u: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(cos_sim(&u, &Vector3::zeros()), None);
        assert_eq!(cos_sim(&Vector3::zeros(), &Vector3::zeros()), None);
    }

    #[test]
    fn proyeksi_ke_vektor_nol_none() {
        let u: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
        assert!(proyeksi(&u, &Vector3::zeros()).is_none());
    }

    #[test]
    fn normalisasi() {
        let v: Vector3<f64> = Vector3::new(4.0, 0.0, -3.0);
        let unit = v.try_normalize(EPS).expect("vektor nol");
        assert!((unit.norm() - 1.0).abs() < EPS);
        assert!(Vector3::<f64>::zeros().try_normalize(EPS).is_none());
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(200))]

        #[test]
        fn komponen_tegak_ortogonal(
            ux in -1e3f64..1e3, uy in -1e3f64..1e3, uz in -1e3f64..1e3,
            vx in -1e3f64..1e3, vy in -1e3f64..1e3, vz in -1e3f64..1e3,
        ) {
            let u: Vector3<f64> = Vector3::new(ux, uy, uz);
            let v: Vector3<f64> = Vector3::new(vx, vy, vz);
            prop_assume!(v.norm() > 1e-6);
            let (proj, tegak) = proyeksi(&u, &v).unwrap();
            // toleransi relatif terhadap ukuran u dan v
            let tol = EPS * (1.0 + u.norm() * v.norm());
            prop_assert!(tegak.dot(&v).abs() <= tol);
            prop_assert!((proj + tegak - u).norm() <= EPS * (1.0 + u.norm()));
        }

        #[test]
        fn kosinus_selalu_di_rentang(
            ux in -1e3f64..1e3, uy in -1e3f64..1e3, uz in -1e3f64..1e3,
            vx in -1e3f64..1e3, vy in -1e3f64..1e3, vz in -1e3f64..1e3,
        ) {
            let u: Vector3<f64> = Vector3::new(ux, uy, uz);
            let v: Vector3<f64> = Vector3::new(vx, vy, vz);
            if let Some(c) = cos_sim(&u, &v) {
                prop_assert!((-1.0 - 1e-12..=1.0 + 1e-12).contains(&c));
            }
        }
    }
}
