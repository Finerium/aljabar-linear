// Nama  : Ghaisan Khoirul Badruzaman
// NIM   : 251524048
// Kelas : 2B-D4
// Modul 2 - Task I: Norma-p Manual & Verifikasi Sifat

use nalgebra::Vector2;

/// Norma-p pada slice. p = f64::INFINITY menghasilkan norma maksimum (Chebyshev).
pub fn norma_p(v: &[f64], p: f64) -> f64 {
    if p.is_infinite() {
        v.iter().map(|x| x.abs()).fold(f64::NEG_INFINITY, f64::max)
    } else {
        v.iter().map(|x| x.abs().powf(p)).sum::<f64>().powf(1.0 / p)
    }
}

pub fn jalankan() {
    println!("=== Task I: Norma-p Manual & Verifikasi Sifat ===");

    let v = [3.0, -4.0];
    println!("v = {}", crate::fmt_vec(&v));
    println!(
        "manual  : ||v||_1 = {}, ||v||_2 = {}, ||v||_inf = {}",
        norma_p(&v, 1.0),
        norma_p(&v, 2.0),
        norma_p(&v, f64::INFINITY)
    );

    // nalgebra belum punya lp_norm untuk p tak hingga, jadi L-inf pakai fold
    let nv: Vector2<f64> = Vector2::new(3.0, -4.0);
    let l_inf = nv.iter().fold(0.0_f64, |m, x| m.max(x.abs()));
    println!(
        "nalgebra: ||v||_1 = {}, ||v||_2 = {}, ||v||_inf = {}",
        nv.lp_norm(1),
        nv.norm(),
        l_inf
    );

    // norma-p turun saat p membesar: L1 >= L2 >= L-inf
    println!(
        "urutan  : 7 >= 5 >= 4 -> {}",
        norma_p(&v, 1.0) >= norma_p(&v, 2.0) && norma_p(&v, 2.0) >= l_inf
    );
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    const EPS: f64 = 1e-12;

    #[test]
    fn norma_p_contoh_modul() {
        let v = [3.0, -4.0];
        assert!((norma_p(&v, 1.0) - 7.0).abs() < EPS);
        assert!((norma_p(&v, 2.0) - 5.0).abs() < EPS);
        assert!((norma_p(&v, f64::INFINITY) - 4.0).abs() < EPS);
    }

    #[test]
    fn manual_sama_dengan_nalgebra() {
        let v: Vector2<f64> = Vector2::new(3.0, -4.0);
        assert!((v.norm() - 5.0).abs() < EPS);
        assert!((v.lp_norm(1) - 7.0).abs() < EPS);
        assert!((v.lp_norm(2) - 5.0).abs() < EPS);
        assert!((v.lp_norm(1) - norma_p(v.as_slice(), 1.0)).abs() < EPS);
    }

    #[test]
    fn norma_vektor_nol_adalah_nol() {
        assert_eq!(norma_p(&[0.0, 0.0, 0.0], 2.0), 0.0);
        assert_eq!(norma_p(&[0.0, 0.0, 0.0], f64::INFINITY), 0.0);
    }

    // Toleransi relatif: galat pembulatan ikut membesar kalau komponennya besar,
    // jadi epsilon mutlak 1e-12 bisa gagal untuk vektor sekitar 1e3.
    fn toleransi(skala: f64) -> f64 {
        EPS * (1.0 + skala)
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(200))]

        #[test]
        fn homogenitas(x in -1e3f64..1e3, y in -1e3f64..1e3, k in -10f64..10f64) {
            let v: Vector2<f64> = Vector2::new(x, y);
            let kiri = (k * v).norm();
            let kanan = k.abs() * v.norm();
            prop_assert!((kiri - kanan).abs() <= toleransi(kanan));
        }

        #[test]
        fn homogenitas_kali_dua(x in -1e3f64..1e3, y in -1e3f64..1e3) {
            let v: Vector2<f64> = Vector2::new(x, y);
            prop_assert!(((2.0 * v).norm() - 2.0 * v.norm()).abs() <= toleransi(v.norm()));
        }

        #[test]
        fn segitiga(ax in -1e3f64..1e3, ay in -1e3f64..1e3, bx in -1e3f64..1e3, by in -1e3f64..1e3) {
            let (a, b): (Vector2<f64>, Vector2<f64>) = (Vector2::new(ax, ay), Vector2::new(bx, by));
            let batas = a.norm() + b.norm();
            prop_assert!((a + b).norm() <= batas + toleransi(batas));
        }

        #[test]
        fn cauchy_schwarz(ax in -1e3f64..1e3, ay in -1e3f64..1e3, bx in -1e3f64..1e3, by in -1e3f64..1e3) {
            let (a, b): (Vector2<f64>, Vector2<f64>) = (Vector2::new(ax, ay), Vector2::new(bx, by));
            let batas = a.norm() * b.norm();
            prop_assert!(a.dot(&b).abs() <= batas + toleransi(batas));
        }

        #[test]
        fn norma_p_turun_saat_p_naik(x in -1e3f64..1e3, y in -1e3f64..1e3, z in -1e3f64..1e3) {
            let v = [x, y, z];
            let (l1, l2, linf) = (norma_p(&v, 1.0), norma_p(&v, 2.0), norma_p(&v, f64::INFINITY));
            prop_assert!(linf <= l2 + toleransi(l2) && l2 <= l1 + toleransi(l1));
        }
    }
}
