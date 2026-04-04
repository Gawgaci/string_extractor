use object::{Object, ObjectSection};
use regex::bytes::Regex;
use std::{env, fs, process};

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("[!] Kullanım: {} <binary_dosya_yolu>", args[0]);
        eprintln!("[*] Örnek: cargo run -- malware_sample.exe");
        process::exit(1);
    }

    let target_file = &args[1];
    println!("\n🔍 SAFE STRING EXTRACTOR v1.0");
    println!("========================================");
    println!("Hedef: {}", target_file);

    // Binary dosyasını oku
    let binary_data = fs::read(target_file)
        .map_err(|e| format!("Dosya okunamadı: {}", e))?;

    // Binary parse et (PE veya ELF destekler)
    let file = object::File::parse(&*binary_data)
        .map_err(|e| format!("Binary parse hatası: {}", e))?;

    // Regex'ler (binary seviyesinde)
    let ip_regex = Regex::new(r"\b(?:\d{1,3}\.){3}\d{1,3}\b")?;
    let url_regex = Regex::new(r"https?://[a-zA-Z0-9./?=_%&#-]+")?;

    let mut found = false;

    for section in file.sections() {
        let section_name = section.name().unwrap_or("<unknown>");
        if let Ok(data) = section.data() {
            // IP Tespiti
            for mat in ip_regex.find_iter(data) {
                let ip = String::from_utf8_lossy(mat.as_bytes());
                println!("[SECTION: {:<12}] 🔥 KRİTİK IP (C2?) → {}", section_name, ip);
                found = true;
            }

            // URL Tespiti
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