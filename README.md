# Aljabar Linear

Praktikum Aljabar Linear (25TI2103), 1 SKS, wajib. Sarjana Terapan Teknik Informatika,
Jurusan Teknik Komputer dan Informatika, Politeknik Negeri Bandung.
Semester Ganjil 2026/2027.

- Nama  : Ghaisan Khoirul Badruzaman
- NIM   : 251524048
- Kelas : 2B-D4

Praktikumnya memakai Rust, bukan Python atau MATLAB. Ada 16 modul, dari setup toolchain
sampai proyek terpadu PCA dan PageRank di modul terakhir.

## Jadwal

| Jenis | Waktu | Ruang | Dosen |
|---|---|---|---|
| Praktikum | Senin 08.40 - 10.40 | D102 Lab. MT | Muhammad Rizqi Sholahuddin, S.Si., M.T. |
| Teori | Rabu 08.40 - 10.40 | D111 | Dr. Nurjannah Syakrani, DRA., M.T. |

## Progres

| Modul | Topik | Status |
|---|---|---|
| [Modul 1](2026-09-07-modul-1/) | Pengenalan Rust dan dasar komputasi vektor | selesai, 17 test lolos |
| [Modul 2](2026-09-14-modul-2/) | Operasi vektor, norma, dan jarak | selesai, 28 test lolos (18 unit, 10 property test), sudah dikumpulkan lewat Teams Selasa 15 Sep 2026 |

## Struktur

Satu folder per pertemuan, diawali tanggalnya biar urut kronologis.

```
materi/
  2026-09-07-modul-1/      modul dan slide dari dosen, tidak di-commit

2026-09-07-modul-1/
  praktikum-al/            crate cargo, nama proyek ditentukan modul
    src/main.rs            Task 0 dan pemanggil task lainnya
    src/task1.rs           Task I, sintaks dan sistem tipe
    src/task2.rs           Task II, vektor manual pakai array dan Vec
    src/task3.rs           Task III, vektor dengan nalgebra
  laporan/                 laporan praktikum .pdf dan .docx
  screenshot/              tangkapan layar cargo run dan cargo test
  bukti-error/             berkas yang sengaja gagal kompilasi untuk Task II
  submission/              arsip .zip yang diunggah ke Teams
  riwayat-cargo-test.txt   keluaran build, clippy, fmt, dan test

2026-09-14-modul-2/
  praktikum-al/            crate cargo, nalgebra + proptest (dev-dependency)
    src/main.rs            pemilih bagian lewat argumen 1 sampai 4
    src/task1.rs           Task I, norma-p manual dan property test sifat norma
    src/task2.rs           Task II, jarak, kesamaan kosinus, proyeksi
    src/task3.rs           Task III, studi kasus kemiripan dokumen
    src/latihan.rs         latihan 1 dan 2, tugas rumah jarak Minkowski, tantangan
  laporan/                 laporan praktikum .pdf
  screenshot/              tangkapan layar toolchain, cargo run, dan cargo test
  submission/              arsip .zip yang diunggah ke Teams
  riwayat-cargo-test.txt   keluaran cargo test
```

Folder di `materi/` memakai penamaan tanggal yang sama dengan folder tugasnya,
jadi materi dan pengerjaan satu pertemuan gampang dipasangkan.

## Menjalankan

```
cd 2026-09-07-modul-1/praktikum-al
cargo run          # jalankan keempat task berurutan
cargo run -- 2     # jalankan satu task saja, argumen 0 sampai 3
cargo test         # 17 unit test

cd 2026-09-14-modul-2/praktikum-al
cargo run -- 3     # argumen 1 sampai 4: Task I, II, III, latihan
cargo test         # 28 test, termasuk property test proptest
cargo clippy --all-targets
cargo fmt --check
```

Toolchain: rustc 1.98.1 stable, edition 2024, nalgebra 0.35.0, proptest 1.11.0 (mulai Modul 2).

## Pengumpulan

Lewat Teams atau e-learning Polban. Tiap modul dikumpulkan sebagai satu arsip
`[AL2026_2B_D4_2025]_Modul<N>_048.zip` berisi seluruh proyek cargo tanpa folder
`target/`, riwayat `cargo test`, dan Laporan Praktikum `.pdf`. Arsip final tiap modul
disimpan di `submission/` supaya berkas yang benar-benar diunggah ikut terarsip.

Angka terakhir di kurung siku adalah angkatan, yaitu 2025 sesuai Tabel 2 di Modul 2 dan instruksi Teams.
Arsip Modul 1 terlanjur memakai `2026` di posisi itu.

Rubrik penilaian menghargai `cargo build` tanpa warning dan kode yang lolos
`cargo clippy`, jadi keduanya dijalankan sebelum tiap modul dianggap selesai.

## Catatan

Folder `materi/` berisi modul dari dosen dan sengaja tidak ikut di-commit karena
hak ciptanya milik JTK POLBAN. Modulnya terbit di
https://kyeiki.github.io/alin-praktikum/, jadi bisa dibaca ulang kapan saja. Repo GitHub
sumbernya (`kyeiki/alin-praktikum`) per 14 September 2026 sudah tidak bisa diakses publik.
