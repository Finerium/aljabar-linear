// Nama  : Ghaisan Khoirul Badruzaman
// NIM   : 251524048
// Kelas : 2B-D4
// Modul 2 - Task III: Studi Kasus Kemiripan Dokumen (TF Sederhana)

use nalgebra::SVector;

type Dok = SVector<f64, 5>;

pub const KOSAKATA: [&str; 5] = ["rust", "vektor", "matriks", "cepat", "aman"];

pub fn cos_sim(u: &Dok, v: &Dok) -> Option<f64> {
    let d = u.norm() * v.norm();
    if d < 1e-12 { None } else { Some(u.dot(v) / d) }
}

pub fn dokumen() -> [(&'static str, Dok); 3] {
    [
        (
            "d1 (dokumen Rust)",
            SVector::from([3.0, 2.0, 1.0, 1.0, 2.0]),
        ),
        (
            "d2 (dokumen teori)",
            SVector::from([1.0, 3.0, 2.0, 0.0, 1.0]),
        ),
        (
            "d3 (dokumen iklan)",
            SVector::from([4.0, 0.0, 0.0, 3.0, 3.0]),
        ),
    ]
}

pub fn kueri() -> Dok {
    SVector::from([2.0, 1.0, 0.0, 1.0, 1.0])
}

/// Peringkat berdasarkan kosinus, dari yang paling mirip (skor terbesar).
pub fn peringkat_kosinus(q: &Dok, docs: &[(&'static str, Dok)]) -> Vec<(&'static str, f64)> {
    let mut hasil: Vec<_> = docs
        .iter()
        .map(|(nama, d)| (*nama, cos_sim(q, d).unwrap_or(0.0)))
        .collect();
    hasil.sort_by(|a, b| b.1.total_cmp(&a.1));
    hasil
}

/// Peringkat berdasarkan jarak Euclidean, dari yang paling dekat (jarak terkecil).
pub fn peringkat_euclidean(q: &Dok, docs: &[(&'static str, Dok)]) -> Vec<(&'static str, f64)> {
    let mut hasil: Vec<_> = docs
        .iter()
        .map(|(nama, d)| (*nama, (q - d).norm()))
        .collect();
    hasil.sort_by(|a, b| a.1.total_cmp(&b.1));
    hasil
}

fn cetak(judul: &str, peringkat: &[(&str, f64)]) {
    println!("{judul}");
    for (i, (nama, skor)) in peringkat.iter().enumerate() {
        println!("  {}. {nama:<20} {skor:.4}", i + 1);
    }
}

pub fn jalankan() {
    println!("=== Task III: Studi Kasus Kemiripan Dokumen ===");
    println!("kosakata = {KOSAKATA:?}");
    let q = kueri();
    let mut docs = dokumen();
    println!("kueri    = {}", crate::fmt_vec(q.iter()));
    for (nama, d) in &docs {
        println!("{nama:<19}= {}", crate::fmt_vec(d.iter()));
    }

    cetak(
        "\nPeringkat kesamaan kosinus (besar = mirip):",
        &peringkat_kosinus(&q, &docs),
    );
    cetak(
        "\nPeringkat jarak Euclidean (kecil = mirip):",
        &peringkat_euclidean(&q, &docs),
    );

    // simulasi dokumen panjang: isi d3 sama, cuma tiap kata diulang 100 kali
    docs[2].1 *= 100.0;
    println!("\nSetelah d3 dikali 100 (dokumen panjang):");
    cetak("Peringkat kesamaan kosinus:", &peringkat_kosinus(&q, &docs));
    cetak(
        "Peringkat jarak Euclidean:",
        &peringkat_euclidean(&q, &docs),
    );
    println!();
}

#[cfg(test)]
mod tests {
    use super::*;
    use proptest::prelude::*;

    #[test]
    fn urutan_kosinus_d1_d3_d2() {
        let urut: String = peringkat_kosinus(&kueri(), &dokumen())
            .iter()
            .map(|x| &x.0[..2])
            .collect::<Vec<_>>()
            .join(" ");
        assert_eq!(urut, "d1 d3 d2");
    }

    #[test]
    fn urutan_euclidean_d1_d2_d3() {
        let urut: String = peringkat_euclidean(&kueri(), &dokumen())
            .iter()
            .map(|x| &x.0[..2])
            .collect::<Vec<_>>()
            .join(" ");
        assert_eq!(urut, "d1 d2 d3");
    }

    #[test]
    fn nilai_kosinus_d1() {
        // 11 / (sqrt(7) * sqrt(19))
        let c = cos_sim(&kueri(), &dokumen()[0].1).unwrap();
        assert!((c - 11.0 / (7.0_f64.sqrt() * 19.0_f64.sqrt())).abs() < 1e-12);
    }

    #[test]
    fn skala_100_kosinus_tetap_euclidean_berubah() {
        let q = kueri();
        let d3 = dokumen()[2].1;
        let besar = 100.0 * d3;
        assert!((cos_sim(&q, &d3).unwrap() - cos_sim(&q, &besar).unwrap()).abs() < 1e-12);
        assert!((q - besar).norm() > 100.0 * (q - d3).norm());
    }

    proptest! {
        #![proptest_config(ProptestConfig::with_cases(200))]

        #[test]
        fn kosinus_invarian_skala_positif(k in 0.01f64..1e4, i in 0usize..3) {
            let q = kueri();
            let d = dokumen()[i].1;
            let beda = (cos_sim(&q, &d).unwrap() - cos_sim(&q, &(k * d)).unwrap()).abs();
            prop_assert!(beda < 1e-12);
        }
    }
}
