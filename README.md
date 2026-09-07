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
```

Folder di `materi/` memakai penamaan tanggal yang sama dengan folder tugasnya,
jadi materi dan pengerjaan satu pertemuan gampang dipasangkan.

## Menjalankan

```
cd 2026-09-07-modul-1/praktikum-al
cargo run          # jalankan keempat task berurutan
cargo run -- 2     # jalankan satu task saja, argumen 0 sampai 3
cargo test         # 17 unit test
cargo clippy --all-targets
cargo fmt --check
```

Toolchain: rustc 1.98.1 stable, edition 2024, nalgebra 0.35.0.

## Pengumpulan

Lewat Teams atau e-learning Polban. Tiap modul dikumpulkan sebagai satu arsip
`[AL2026_2B_D4_2026]_Modul<N>_048.zip` berisi seluruh proyek cargo tanpa folder
`target/`, riwayat `cargo test`, dan Laporan Praktikum `.pdf`. Arsip final tiap modul
disimpan di `submission/` supaya berkas yang benar-benar diunggah ikut terarsip.

Rubrik penilaian menghargai `cargo build` tanpa warning dan kode yang lolos
`cargo clippy`, jadi keduanya dijalankan sebelum tiap modul dianggap selesai.

## Catatan

Folder `materi/` berisi modul dari dosen dan sengaja tidak ikut di-commit karena
hak ciptanya milik JTK POLBAN. Sumber modulnya publik di
[kyeiki/alin-praktikum](https://github.com/kyeiki/alin-praktikum), jadi bisa diambil
ulang kapan saja kalau salinan lokalnya hilang.
