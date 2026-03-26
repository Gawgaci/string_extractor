use object::{Object, ObjectSection};
use regex::bytes::Regex;
use serde_json::json;
use std::env;
use std::fs;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // 1. Kullanıcıdan hedef dosyayı alalım
    let args: Vec<String> = env::args().collect();
    
    if args.len() < 2 {
        println!("[-] Hata: Lütfen analiz edilecek dosyayı belirtin.");
        println!("[*] Kullanım: cargo run -- <dosya_yolu>");
        println!("[*] Örnek : cargo run -- src/main.rs");
        return Ok(());
    }

    let target_file = &args[1];
    let binary_data = fs::read(target_file)?;

    // 2. Regex Kuralları
    let ip_regex = Regex::new(r"\b(?:\d{1,3}\.){3}\d{1,3}\b").unwrap();
    let url_regex = Regex::new(r"https?://[a-zA-Z0-9./?=_-]+").unwrap();
    // İşte sarı uyarıyı çözen kısım: Artık standart stringleri de okuyoruz!
    let string_regex = Regex::new(r"[ -~]{5,}").unwrap(); 

    let mut all_sections_data = Vec::new();

    // 3. Bölüm Farkındalıklı (Section-Aware) Analiz
    match object::File::parse(&*binary_data) {
        Ok(parsed_file) => {
            for section in parsed_file.sections() {
                if let Ok(name) = section.name() {
                    let data = section.data()?;
                    if data.is_empty() { continue; }

                    let mut ips = Vec::new();
                    let mut urls = Vec::new();
                    let mut general_strings = Vec::new();

                    for mat in ip_regex.find_iter(data) {
                        ips.push(String::from_utf8_lossy(mat.as_bytes()).to_string());
                    }
                    for mat in url_regex.find_iter(data) {
                        urls.push(String::from_utf8_lossy(mat.as_bytes()).to_string());
                    }
                    for mat in string_regex.find_iter(data) {
                        let found_str = String::from_utf8_lossy(mat.as_bytes()).to_string();
                        // IP veya URL değilse, standart string olarak say
                        if !ips.contains(&found_str) && !urls.contains(&found_str) {
                            general_strings.push(found_str);
                        }
                    }

                    // Sadece içi dolu olan bölümleri rapora ekle
                    if !ips.is_empty() || !urls.is_empty() || !general_strings.is_empty() {
                        let section_json = json!({
                            "bolum_adi": name,
                            "bulunan_c2_ipler": ips,
                            "bulunan_urller": urls,
                            "diger_string_sayisi": general_strings.len() 
                        });
                        all_sections_data.push(section_json);
                    }
                }
            }
        }
        Err(_) => println!("[-] Hata: Dosya PE/ELF formatında değil."),
    }

    // 4. Hocanın İstediği Yapılandırılmış JSON Çıktısı
    let final_report = json!({
        "analiz_edilen_dosya": target_file,
        "analiz_sonuclari": all_sections_data
    });

    println!("{}", serde_json::to_string_pretty(&final_report)?);

    Ok(())
}