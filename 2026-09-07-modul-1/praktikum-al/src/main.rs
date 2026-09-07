// Nama  : Ghaisan Khoirul Badruzaman
// NIM   : 251524048
// Kelas : 2B-D4
// Modul 1: Pengenalan Rust & Dasar Komputasi Vektor

mod task1;
mod task2;
mod task3;

use nalgebra::Vector3;

/// Task 0: memastikan rustc, cargo, dan crate nalgebra benar-benar terpasang.
fn task0() {
    println!("Hello, Aljabar Linear!");
    println!("Rust version check: OK");

    let v = Vector3::new(1.0, 2.0, 3.0);
    println!("nalgebra OK, contoh vektor: {v}");
}

fn main() {
    // Argumen opsional dipakai buat menjalankan satu task saja saat
    // mendemokan. Tanpa argumen, keempat task jalan berurutan.
    match std::env::args().nth(1).as_deref() {
        Some("0") => task0(),
        Some("1") => task1::jalankan(),
        Some("2") => task2::jalankan(),
        Some("3") => task3::jalankan(),
        _ => {
            task0();
            task1::jalankan();
            task2::jalankan();
            task3::jalankan();
        }
    }
}
