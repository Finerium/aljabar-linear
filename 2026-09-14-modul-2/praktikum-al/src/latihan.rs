// Nama  : Ghaisan Khoirul Badruzaman
// NIM   : 251524048
// Kelas : 2B-D4
// Modul 2 - III.5 Tugas & Latihan

use nalgebra::{SVector, Vector3};

use crate::task1::norma_p;

/// Tugas rumah: jarak Minkowski untuk dimensi berapa pun (const generic D).
/// p = f64::INFINITY memberi jarak Chebyshev. None kalau p < 1, karena untuk
/// p < 1 ketaksamaan segitiga tidak berlaku lagi sehingga bukan jarak.
pub fn jarak_minkowski<const D: usize>(
    u: &SVector<f64, D>,
    v: &SVector<f64, D>,
    p: f64,
) -> Option<f64> {
    if p.is_nan() || p < 1.0 {
        return None;
    }
    Some(norma_p((u - v).as_slice(), p))
}

/// Pembangkit angka acak kecil (xorshift64) supaya latihan 1 tidak perlu crate rand.
struct Acak(u64);

impl Acak {
    fn berikut(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        // ubah ke rentang [-100, 100)
        (self.0 >> 11) as f64 / (1u64 << 53) as f64 * 200.0 - 100.0
    }
}

pub fn latihan1() {
    println!("--- Latihan 1: ||v||_2 <= ||v||_1 <= sqrt(n) ||v||_2, n = 8 ---");
    let n = 8.0_f64;
    let mut acak = Acak(20260914);
    let mut lolos = 0;
    let mut rasio_maks: f64 = 0.0;
    for _ in 0..100 {
        let v: SVector<f64, 8> = SVector::from_fn(|_, _| acak.berikut());
        let (l1, l2) = (v.lp_norm(1), v.norm());
        if l2 <= l1 && l1 <= n.sqrt() * l2 {
            lolos += 1;
        }
        rasio_maks = rasio_maks.max(l1 / (n.sqrt() * l2));
    }
    println!("vektor yang memenuhi: {lolos}/100");
    println!("rasio ||v||_1 / (sqrt(8) ||v||_2) terbesar = {rasio_maks:.4} (tidak pernah lewat 1)");
}

pub fn latihan2() {
    println!("--- Latihan 2: u = (2, 1, -2), v = (1, -1, 1) ---");
    // anotasi f64 wajib, tanpa itu .acos() ditolak E0282 (sama seperti di Modul 1)
    let u: Vector3<f64> = Vector3::new(2.0, 1.0, -2.0);
    let v: Vector3<f64> = Vector3::new(1.0, -1.0, 1.0);
    let proj = (u.dot(&v) / v.norm_squared()) * v;
    let tegak = u - proj;
    let sudut = (u.dot(&v) / (u.norm() * v.norm())).acos().to_degrees();
    println!("u . v = {}, ||v||^2 = {}", u.dot(&v), v.norm_squared());
    println!("proj_v u = {}  (= -1/3 v)", crate::fmt_vec(proj.iter()));
    println!(
        "tegak    = {}  (= (7/3, 2/3, -5/3))",
        crate::fmt_vec(tegak.iter())
    );
    println!("tegak . v = {:e}", tegak.dot(&v));
    println!("sudut(u, v) = {sudut:.4} derajat");
}

pub fn tugas_rumah() {
    println!("--- Tugas Rumah: jarak_minkowski(u, v, p) ---");
    let u: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
    let v: Vector3<f64> = Vector3::new(4.0, 6.0, 3.0);
    for (label, p) in [("1", 1.0), ("2", 2.0), ("3", 3.0), ("inf", f64::INFINITY)] {
        println!(
            "p = {label:<3} -> {:.4}",
            jarak_minkowski(&u, &v, p).unwrap()
        );
    }
    println!(
        "p = 0.5 -> {:?} (bukan jarak)",
        jarak_minkowski(&u, &v, 0.5)
    );
}

pub fn tantangan() {
    println!("--- Tantangan: Cauchy-Schwarz jadi persamaan ---");
    let u: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
    let v = 2.0 * u; // sejajar dengan u
    let kiri = u.dot(&v).abs();
    let kanan = u.norm() * v.norm();
    println!(
        "|u . v| = {kiri}, ||u|| ||v|| = {kanan}, selisih = {:e}",
        (kiri - kanan).abs()
    );
}

pub fn jalankan() {
    println!("=== III.5 Tugas & Latihan ===");
    latihan1();
    latihan2();
    tugas_rumah();
    tantangan();
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    const EPS: f64 = 1e-12;

    #[test]
    fn minkowski_p1_p2_pinf() {
        let u: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
        let v: Vector3<f64> = Vector3::new(4.0, 6.0, 3.0);
        assert!((jarak_minkowski(&u, &v, 1.0).unwrap() - 7.0).abs() < EPS);
        assert!((jarak_minkowski(&u, &v, 2.0).unwrap() - 5.0).abs() < EPS);
        assert!((jarak_minkowski(&u, &v, f64::INFINITY).unwrap() - 4.0).abs() < EPS);
    }

    #[test]
    fn minkowski_sama_dengan_nalgebra() {
        let u: SVector<f64, 4> = SVector::from([1.0, -2.0, 0.5, 7.0]);
        let v: SVector<f64, 4> = SVector::from([-3.0, 2.0, 2.5, 1.0]);
        assert!((jarak_minkowski(&u, &v, 1.0).unwrap() - (u - v).lp_norm(1)).abs() < EPS);
        assert!((jarak_minkowski(&u, &v, 2.0).unwrap() - (u - v).norm()).abs() < EPS);
    }

    #[test]
    fn minkowski_p_tidak_valid() {
        let u: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
        assert_eq!(jarak_minkowski(&u, &u, 0.5), None);
        assert_eq!(jarak_minkowski(&u, &u, f64::NAN), None);
    }

    #[test]
    fn minkowski_ke_diri_sendiri_nol() {
        let u: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
        for p in [1.0, 2.0, 3.0, f64::INFINITY] {
            assert_eq!(jarak_minkowski(&u, &u, p), Some(0.0));
        }
    }

    #[test]
    fn latihan2_proyeksi() {
        let u: Vector3<f64> = Vector3::new(2.0, 1.0, -2.0);
        let v: Vector3<f64> = Vector3::new(1.0, -1.0, 1.0);
        let proj = (u.dot(&v) / v.norm_squared()) * v;
        let tegak = u - proj;
        assert!((proj - Vector3::new(-1.0, 1.0, -1.0) / 3.0).norm() < EPS);
        assert!((tegak - Vector3::new(7.0, 2.0, -5.0) / 3.0).norm() < EPS);
        assert!(tegak.dot(&v).abs() < EPS);
    }

    #[test]
    fn tantangan_cauchy_schwarz_sejajar() {
        let u: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
        let v = -4.0 * u;
        assert!((u.dot(&v).abs() - u.norm() * v.norm()).abs() < 1e-10);
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(100))]

        #[test]
        fn latihan1_hubungan_norma(arr in prop::array::uniform8(-1e3f64..1e3)) {
            let v: SVector<f64, 8> = SVector::from(arr);
            let (l1, l2) = (v.lp_norm(1), v.norm());
            let tol = EPS * (1.0 + l1);
            prop_assert!(l2 <= l1 + tol);
            prop_assert!(l1 <= 8.0_f64.sqrt() * l2 + tol);
        }

        #[test]
        fn minkowski_simetris(a in prop::array::uniform3(-1e3f64..1e3), b in prop::array::uniform3(-1e3f64..1e3), p in 1f64..5.0) {
            let (u, v): (Vector3<f64>, Vector3<f64>) = (Vector3::from(a), Vector3::from(b));
            let (duv, dvu) = (jarak_minkowski(&u, &v, p).unwrap(), jarak_minkowski(&v, &u, p).unwrap());
            prop_assert!((duv - dvu).abs() <= EPS * (1.0 + duv));
        }
    }
}
