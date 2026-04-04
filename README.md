#  String Extractor (Rust-based RE Tool)

##  Proje Özeti
Bu proje, **Tersine Mühendislik** dersi vize ödevi kapsamında geliştirilmiş, bellek güvenli (**Memory-Safe**) bir binary analiz aracıdır. Standart `strings` komutunun ötesine geçerek, çalıştırılabilir dosyaların (PE/ELF) iç yapısını analiz eder ve siber güvenlik odaklı hassas verileri (C2 IP, URL, API Keys) otomatik olarak sınıflandırır.

##  Teknik Altyapı ve Metodoloji
* **Dil:** Rust (Bellek güvenliği ve performans için tercih edilmiştir).
* **Kütüphane:** `object` (Binary dosya bölümlerini -sections- ayrıştırmak için).
* **Regex Motoru:** `regex::bytes` (Binary veriler içinde ham bayt seviyesinde arama yapmak için).
* **Analiz Yaklaşımı:** "Section-Aware Parsing" yöntemiyle dosyanın sadece `.text`, `.data` veya `.rdata` gibi belirli bölümleri taranarak yanlış pozitif (False Positive) oranı minimize edilmiştir.

##  Öne Çıkan Özellikler 
1.  **Hata Yönetimi (Result<>):** Kod içerisinde `unwrap()` kullanılmamış, tüm dosya okuma ve ayrıştırma süreçleri Rust'ın `Result` yapısıyla güvenli hale getirilmiştir.
2.  **Otomatik Sınıflandırma:** Bulunan stringler sadece ekrana basılmaz; IPv4 adresleri ve HTTP/HTTPS uç noktaları olarak kategorize edilir.
3.  **Cross-Platform Destek:** Hem Windows (PE) hem de Linux (ELF) dosyalarını aynı motor üzerinden analiz edebilir.
4.  **Unit Testler:** Kodun doğruluğu `cargo test` ile kanıtlanmış, Regex desenleri test senaryolarıyla doğrulanmıştır.

##  Örnek Analiz Akışı
```text
Input: malware.exe ➔ [Parser] ➔ [Section Identification] ➔ [Regex Filter] ➔ Output
