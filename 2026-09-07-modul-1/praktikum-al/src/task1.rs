// Nama  : Ghaisan Khoirul Badruzaman
// NIM   : 251524048
// Kelas : 2B-D4
// Modul 1 - Task I: Dasar Sintaks & Tipe Data Rust

/// Ambang toleransi buat membandingkan dua f64.
const EPSILON: f64 = 1e-10;

fn judul(teks: &str) {
    println!("\n{}", "-".repeat(56));
    println!("{teks}");
    println!("{}", "-".repeat(56));
}

/// Perbandingan float pakai == gampang meleset karena galat pembulatan,
/// jadi selisihnya yang dicek terhadap EPSILON.
fn hampir_sama(a: f64, b: f64) -> bool {
    (a - b).abs() < EPSILON
}

fn klasifikasi(nilai: f64) -> &'static str {
    if nilai as i64 % 2 == 0 {
        "genap"
    } else {
        "ganjil"
    }
}

pub fn jalankan() {
    judul("Bagian 1: Variable Binding & Immutability");
    let n: i32 = 10;
    // 3.14159 mendekati PI, tanpa atribut ini clippy menolak
    #[allow(clippy::approx_constant)]
    let phi: f64 = 3.14159;
    let aktif: bool = true;
    let nama: &str = "Vektor";
    println!("n     (i32)  = {n}");
    println!("phi   (f64)  = {phi}");
    println!("aktif (bool) = {aktif}");
    println!("nama  (&str) = {nama}");

    // binding default immutable, mau diubah nilainya harus ditandai mut dulu
    let tetap = 5;
    let mut berubah = 5;
    berubah += 3;
    println!("tetap (tanpa mut, nilainya kunci) = {tetap}");
    println!("berubah (pakai mut, setelah += 3) = {berubah}");
    println!("EPSILON (const)                   = {EPSILON:e}");
    println!(
        "0.1 + 0.2 hampir sama 0.3 ?       = {}",
        hampir_sama(0.1 + 0.2, 0.3)
    );

    judul("Bagian 2: Operasi Aritmetika i32 dan f64");
    let (a, b) = (7.0f64, 2.0f64);
    println!("a / b        = {}", a / b);
    println!("a % b        = {}", a % b);
    println!("a.powf(b)    = {}", a.powf(b));
    // pembagian antar i32 membuang bagian pecahannya, bukan dibulatkan
    println!("7 / 2  (i32) = {}", 7 / 2);
    println!("7 % 2  (i32) = {}", 7 % 2);
    // sqrt cuma ada di tipe float, integer harus di-cast dulu
    println!("9.0.sqrt()   = {}", 9.0f64.sqrt());

    judul("Bagian 3: Array & Indexing");
    let v: [f64; 5] = [10.0, 20.0, 30.0, 40.0, 50.0];
    let potongan = &v[1..4];
    println!("v            = {v:?}");
    println!("v[0]         = {}", v[0]);
    println!("v[len - 1]   = {}", v[v.len() - 1]);
    println!("&v[1..4]     = {potongan:?}");

    judul("Bagian 4: Perulangan & Percabangan");
    for nilai in v {
        println!("{nilai} -> {}", klasifikasi(nilai));
    }

    // Isi v kebetulan genap semua, jadi cabang else belum kebukti jalan.
    // Array kecil ini dipakai supaya kedua cabang sama-sama tercetak.
    for nilai in [3.0, 4.0, 7.0] {
        println!("{nilai} -> {}", klasifikasi(nilai));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn klasifikasi_genap_ganjil() {
        assert_eq!(klasifikasi(10.0), "genap");
        assert_eq!(klasifikasi(40.0), "genap");
        assert_eq!(klasifikasi(25.0), "ganjil");
        assert_eq!(klasifikasi(-3.0), "ganjil");
    }

    #[test]
    fn hampir_sama_menoleransi_galat_float() {
        assert!(hampir_sama(0.1 + 0.2, 0.3));
        assert!(!hampir_sama(1.0, 1.001));
    }

    #[test]
    fn pembagian_i32_terpotong() {
        assert_eq!(7 / 2, 3);
        assert_eq!(7 % 2, 1);
        assert!(hampir_sama(7.0 / 2.0, 3.5));
    }

    #[test]
    fn operasi_float_dasar() {
        assert!(hampir_sama(7.0f64.powf(2.0), 49.0));
        assert!(hampir_sama(9.0f64.sqrt(), 3.0));
        assert!(hampir_sama(7.0f64 % 2.0, 1.0));
    }

    #[test]
    fn indexing_dan_slicing() {
        let v: [f64; 5] = [10.0, 20.0, 30.0, 40.0, 50.0];
        assert_eq!(v.len(), 5);
        assert!(hampir_sama(v[0], 10.0));
        assert!(hampir_sama(v[v.len() - 1], 50.0));
        assert_eq!(&v[1..4], &[20.0, 30.0, 40.0]);
    }
}
