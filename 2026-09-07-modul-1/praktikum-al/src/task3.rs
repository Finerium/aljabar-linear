// Nama  : Ghaisan Khoirul Badruzaman
// NIM   : 251524048
// Kelas : 2B-D4
// Modul 1 - Task III: Introduksi nalgebra untuk Vektor

use nalgebra::{SVector, Vector2, Vector3};

pub fn jalankan() {
    println!("=== Task III: Vektor dengan nalgebra ===");

    // A. Pembuatan vektor
    let a = Vector3::new(1.0, 2.0, 3.0);
    let b4: SVector<f64, 4> = SVector::from([1.0, 2.0, 3.0, 4.0]);
    println!("a = {a}");
    println!("dimensi a: {}", a.nrows());
    println!("b4 = {b4}");
    println!("dimensi b4: {}", b4.nrows());

    // B. Aritmetika element-wise lewat operator
    let b = Vector3::new(4.0, 5.0, 6.0);
    println!("a + b = {}", a + b);
    println!("a - b = {}", a - b);
    println!("2a    = {}", 2.0 * a);
    println!("a o b = {}", a.component_mul(&b)); // perkalian Hadamard

    // C. Dot product, norma, dan sudut
    let dot_ab = a.dot(&b);
    let norm_a = a.norm();
    // anotasi : f64 wajib di sini, tanpa itu tipe hasil bagi masih generik
    // dan pemanggilan .acos() ditolak dengan E0282 type annotations needed
    let cos: f64 = a.dot(&b) / (a.norm() * b.norm());
    let sudut = cos.acos().to_degrees();
    println!("a . b = {dot_ab}");
    println!("||a|| = {norm_a}");
    println!("sudut a dan b = {sudut} derajat");

    // D. Fungsi statistik
    println!("sum  = {}", a.sum());
    println!("mean = {}", a.mean());
    println!("max  = {}", a.max());
    println!("min  = {}", a.min());

    // E.1 Norma, dot product, dan sudut untuk dua vektor 2 dimensi
    let p = Vector2::new(3.0, 4.0);
    let q = Vector2::new(4.0, 0.0);
    let cos_pq: f64 = p.dot(&q) / (p.norm() * q.norm());
    println!("||p|| = {}, ||q|| = {}", p.norm(), q.norm());
    println!("p . q = {}", p.dot(&q));
    println!("sudut p dan q = {} derajat", cos_pq.acos().to_degrees());

    // E.2 Vektor 10 dimensi berisi 1 sampai 10
    let w: SVector<f64, 10> = SVector::from_iterator((1..=10).map(f64::from));
    // dicetak dalam bentuk transpos supaya tidak memakan 10 baris terminal
    println!("w = {}", w.transpose());
    println!("jumlah w    = {}", w.sum());
    println!("rata-rata w = {}", w.mean());
    println!("maksimum w  = {}", w.max());
    println!("minimum w   = {}", w.min());
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-10;

    // E.3 identitas u . u = ||u||^2, pakai toleransi karena floating point
    #[test]
    fn dot_diri_sendiri_sama_dengan_kuadrat_norma() {
        let a: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
        assert!((a.dot(&a) - a.norm().powi(2)).abs() < EPS);
    }

    #[test]
    fn dot_product_dan_norma() {
        let a: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
        let b: Vector3<f64> = Vector3::new(4.0, 5.0, 6.0);
        assert!((a.dot(&b) - 32.0).abs() < EPS);
        assert!((a.norm() - 14.0_f64.sqrt()).abs() < EPS);
    }

    #[test]
    fn operasi_element_wise() {
        let a: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
        let b: Vector3<f64> = Vector3::new(4.0, 5.0, 6.0);
        assert_eq!(a + b, Vector3::new(5.0, 7.0, 9.0));
        assert_eq!(a - b, Vector3::new(-3.0, -3.0, -3.0));
        assert_eq!(2.0 * a, Vector3::new(2.0, 4.0, 6.0));
        assert_eq!(a.component_mul(&b), Vector3::new(4.0, 10.0, 18.0));
    }

    #[test]
    fn statistik_vektor_a() {
        let a: Vector3<f64> = Vector3::new(1.0, 2.0, 3.0);
        assert!((a.sum() - 6.0).abs() < EPS);
        assert!((a.mean() - 2.0).abs() < EPS);
        assert!((a.max() - 3.0).abs() < EPS);
        assert!((a.min() - 1.0).abs() < EPS);
    }

    #[test]
    fn statistik_vektor_w() {
        let w: SVector<f64, 10> = SVector::from_iterator((1..=10).map(f64::from));
        assert!((w.sum() - 55.0).abs() < EPS);
        assert!((w.mean() - 5.5).abs() < EPS);
        assert!((w.max() - 10.0).abs() < EPS);
        assert!((w.min() - 1.0).abs() < EPS);
    }

    #[test]
    fn norma_dan_sudut_p_q() {
        let p: Vector2<f64> = Vector2::new(3.0, 4.0);
        let q: Vector2<f64> = Vector2::new(4.0, 0.0);
        assert!((p.norm() - 5.0).abs() < EPS);
        assert!((q.norm() - 4.0).abs() < EPS);
        assert!((p.dot(&q) - 12.0).abs() < EPS);

        let cos: f64 = p.dot(&q) / (p.norm() * q.norm());
        assert!((cos.acos().to_degrees() - 53.1301).abs() < 1e-4);
    }
}
