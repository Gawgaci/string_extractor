use object::{Object, ObjectSection};
use regex::bytes::Regex;
use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    // Kullanım kontrolü - Argüman eksikse hata ver
    if args.len() < 2 {
        eprintln!("[!] Kullanım: {} <binary_dosya_yolu>", args[0]);
        eprintln!("[*] Örnek: cargo run -- malware_sample.exe");
        process::exit(1);
    }

    let target_file = &args[1];
    println!("\n🔍 SAFE STRING EXTRACTOR v1.0");
    println!("========================================");
    println!("Hedef: {}", target_file);

    if let Err(e) = run_analysis(target_file) {
        eprintln!("[!] Hata: {}", e);
        process::exit(1);
    }
}

fn run_analysis(target_file: &str) -> Result<(), Box<dyn std::error::Error>> {
    // 1. Binary dosyasını diskten oku
    let binary_data = fs::read(target_file)
        .map_err(|e| format!("Dosya okunamadı: {}", e))?;

    // 2. Binary'yi parse et (PE veya ELF formatı desteklenir)
    let file = object::File::parse(&*binary_data)
        .map_err(|e| format!("Binary parse hatası: {}", e))?;

    // 3. Regex desenleri tanımla (binary seviyesinde arama)
    let ip_regex = Regex::new(r"\b(?:\d{1,3}\.){3}\d{1,3}\b")?;
    let url_regex = Regex::new(r"https?://[a-zA-Z0-9./?=_%&#-]+")?;

    let mut found = false;

    // 4. Section-aware tarama (sadece ilgili bölümler incelenir)
    for section in file.sections() {
        let section_name = section.name().unwrap_or("<unknown>");

        if let Ok(data) = section.data() {
            // IP adresi arama
            for mat in ip_regex.find_iter(data) {
                let ip = String::from_utf8_lossy(mat.as_bytes());
                if ip != "0.0.0.0" && ip != "255.255.255.255" {
                    println!("[SECTION: {:<12}] 🔥 KRİTİK IP (C2?) → {}", section_name, ip);
                    found = true;
                }
            }

            // URL arama
            for mat in url_regex.find_iter(data) {
                let url = String::from_utf8_lossy(mat.as_bytes());
                println!("[SECTION: {:<12}] 🌐 ŞÜPHELİ URL (C2/Callback) → {}", section_name, url);
                found = true;
            }
        }
    }

    if !found {
        println!("[+] Temiz: Şüpheli ağ göstergesi (IP/URL) bulunamadı.");
    }

    println!("========================================\n");
    Ok(())
}