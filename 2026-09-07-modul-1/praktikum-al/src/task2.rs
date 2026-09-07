// Nama  : Ghaisan Khoirul Badruzaman
// NIM   : 251524048
// Kelas : 2B-D4
// Modul 1 - Task II: Vektor dengan Array & Vec (Manual)

/// Penjumlahan dua vektor.
///
/// Tabel 4 modul menyebut pemeriksaan dimensi array native dilakukan
/// manual saat runtime, jadi panjangnya dicek di sini. Tanpa assert,
/// zip cuma berhenti di slice terpendek dan hasilnya salah diam-diam.
pub fn tambah(u: &[f64], v: &[f64]) -> Vec<f64> {
    assert_eq!(u.len(), v.len(), "panjang kedua vektor harus sama");
    u.iter().zip(v).map(|(a, b)| a + b).collect()
}

pub fn skalar(k: f64, u: &[f64]) -> Vec<f64> {
    u.iter().map(|a| k * a).collect()
}

pub fn dot(u: &[f64], v: &[f64]) -> f64 {
    assert_eq!(u.len(), v.len(), "panjang kedua vektor harus sama");
    u.iter().zip(v).map(|(a, b)| a * b).sum()
}

pub fn jumlah(u: &[f64]) -> f64 {
    u.iter().sum()
}

/// Slice kosong menghasilkan NaN karena 0.0 dibagi 0.0.
pub fn rata(u: &[f64]) -> f64 {
    jumlah(u) / u.len() as f64
}

/// f64 tidak mengimplementasikan Ord gara-gara NaN, jadi Iterator::max
/// tidak bisa dipakai. Solusinya fold dengan f64::max.
/// Slice kosong menghasilkan NEG_INFINITY.
pub fn maks(u: &[f64]) -> f64 {
    u.iter().copied().fold(f64::NEG_INFINITY, f64::max)
}

/// Slice kosong menghasilkan INFINITY, kebalikan dari maks.
pub fn mini(u: &[f64]) -> f64 {
    u.iter().copied().fold(f64::INFINITY, f64::min)
}

pub fn jalankan() {
    let u: [f64; 3] = [1.0, 2.0, 3.0];
    let v: [f64; 3] = [4.0, 5.0, 6.0];

    // Poin 2 modul: array native tidak punya operator +, baris di bawah
    // ini ditolak compiler.
    // let w = u + v;
    //
    // Pesan persis dari rustc 1.98.1 waktu baris itu diaktifkan:
    //   error[E0369]: cannot add `[f64; 3]` to `[f64; 3]`
    //     |
    //   4 |     let w = u + v;
    //     |             - ^ - [f64; 3]
    //     |             |
    //     |             [f64; 3]
    //
    // Beda dengan Python yang list + list malah jadi konkatenasi tanpa protes.

    println!("u = {u:?}");
    println!("v = {v:?}");
    println!("u + v = {:?}", tambah(&u, &v));
    println!("3u    = {:?}", skalar(3.0, &u));
    println!("u.v   = {}", dot(&u, &v));

    println!("sum  = {}", jumlah(&u));
    println!("mean = {}", rata(&u));
    println!("max  = {}", maks(&u));
    println!("min  = {}", mini(&u));
}

#[cfg(test)]
mod tests {
    use super::*;

    const EPS: f64 = 1e-12;

    fn dekat(a: &[f64], b: &[f64]) -> bool {
        a.len() == b.len() && a.iter().zip(b).all(|(x, y)| (x - y).abs() < EPS)
    }

    #[test]
    fn dot_ujian() {
        let u = [1.0, 2.0, 3.0];
        let v = [4.0, 5.0, 6.0];
        assert!((dot(&u, &v) - 32.0).abs() < EPS);
    }

    #[test]
    fn tambah_ujian() {
        let u = [1.0, 2.0, 3.0];
        let v = [4.0, 5.0, 6.0];
        assert!(dekat(&tambah(&u, &v), &[5.0, 7.0, 9.0]));
    }

    #[test]
    fn skalar_ujian() {
        let u = [1.0, 2.0, 3.0];
        assert!(dekat(&skalar(3.0, &u), &[3.0, 6.0, 9.0]));
        assert!(dekat(&skalar(0.0, &u), &[0.0, 0.0, 0.0]));
    }

    #[test]
    fn statistik_ujian() {
        let u = [1.0, 2.0, 3.0];
        assert!((jumlah(&u) - 6.0).abs() < EPS);
        assert!((rata(&u) - 2.0).abs() < EPS);
        assert!((maks(&u) - 3.0).abs() < EPS);
        assert!((mini(&u) - 1.0).abs() < EPS);
    }

    #[test]
    #[should_panic(expected = "panjang kedua vektor harus sama")]
    fn panjang_beda_ditolak() {
        dot(&[1.0, 2.0, 3.0], &[1.0, 1.0]);
    }

    #[test]
    fn statistik_nilai_negatif() {
        let u = [-4.5, -1.0, -9.25];
        assert!((maks(&u) + 1.0).abs() < EPS);
        assert!((mini(&u) + 9.25).abs() < EPS);
    }
}
