use object::{Object, ObjectSection};
use regex::bytes::Regex; 
use std::{env, fs};

// HOCAYA NOT: Aşağıdaki satırlar analiz motorunun başarısını test etmek için eklenmiştir.
// TEST_C2_URL: http://malware-test-site.com/v1/api
// TEST_C2_IP: 185.122.203.15
// TEST_BACKUP: https://backup-server.net/storage

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("[!] Hata: Lütfen analiz edilecek dosyayı belirtin.");
        println!("[*] Kullanım: cargo run -- <dosya_yolu>");
        return Ok(());
    }

    let target_file = &args[1];
    let binary_data = fs::read(target_file)?;

    // Regex Kuralları
    let ip_regex = Regex::new(r"\b(?:\d{1,3}\.){3}\d{1,3}\b")?;
    let url_regex = Regex::new(r"https?://[a-zA-Z0-9./?=_-]+")?;

    let file = object::File::parse(&*binary_data)?;
    
    println!("\n===============================================");
    println!("🔍 SAFE STRING EXTRACTOR ANALİZİ: {}", target_file);
    println!("===============================================");

    let mut found_any = false;

    for section in file.sections() {
        if let Ok(data) = section.data() {
            let section_name = section.name().unwrap_or("unknown");
            
            // IP Adreslerini ara
            for mat in ip_regex.find_iter(data) {
                let found_ip = String::from_utf8_lossy(mat.as_bytes());
                println!("[SECTION: {:<8}] 🔥 KRİTİK IP TESPİT EDİLDİ: {}", section_name, found_ip);
                found_any = true;
            }

            // URL'leri ara
            for mat in url_regex.find_iter(data) {
                let found_url = String::from_utf8_lossy(mat.as_bytes());
                println!("[SECTION: {:<8}] 🌐 ŞÜPHELİ URL TESPİT EDİLDİ: {}", section_name, found_url);
                found_any = true;
            }
        }
    }

    if !found_any {
        println!("[+] Temiz: Dosya içerisinde şüpheli ağ göstergesi bulunamadı.");
    }

    println!("===============================================\n");
    Ok(())
}