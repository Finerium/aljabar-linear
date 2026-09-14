// Nama  : Ghaisan Khoirul Badruzaman
// NIM   : 251524048
// Kelas : 2B-D4
// Modul 2: Operasi Vektor, Norma, dan Jarak

mod latihan;
mod task1;
mod task2;
mod task3;

/// Format vektor jadi satu baris, misal [2.0000, -1.0000], supaya output
/// tidak sepanjang tampilan kolom bawaan nalgebra.
pub fn fmt_vec<'a>(v: impl IntoIterator<Item = &'a f64>) -> String {
    let isi: Vec<String> = v.into_iter().map(|x| format!("{x:.4}")).collect();
    format!("[{}]", isi.join(", "))
}

fn main() {
    // Argumen opsional buat menjalankan satu bagian saja saat demo.
    // Tanpa argumen semua bagian jalan berurutan.
    match std::env::args().nth(1).as_deref() {
        Some("1") => task1::jalankan(),
        Some("2") => task2::jalankan(),
        Some("3") => task3::jalankan(),
        Some("4") => latihan::jalankan(),
        _ => {
            task1::jalankan();
            task2::jalankan();
            task3::jalankan();
            latihan::jalankan();
        }
    }
}
