# SAFE STRING EXTRACTOR

**Tersine Mühendislik Dersi Vize Projesi**

Bu proje, zararlı yazılımların (malware) PE ve ELF formatındaki ikili (binary) dosyalarından gizli C2 (Command & Control) sunucu adreslerini, IP’leri ve URL’leri otomatik olarak tespit eden Rust tabanlı bir tersine mühendislik aracıdır.

## Projenin Amacı

Modern malware’ler C2 adreslerini, callback URL’lerini ve komuta-kontrol altyapısını binary içinde gizler. Klasik `strings` komutu çok fazla false positive verir ve section farkındalığı yoktur. Bu araç, **section-aware parsing** yöntemiyle sadece ilgili binary bölümlerini (.text, .data, .rsrc vb.) tarayarak şüpheli ağ göstergelerini daha doğru ve hızlı bir şekilde çıkarır.

## İncelenen Zafiyet

**Zafiyet:** Malware’lerin binary içinde gizlediği C2 IP/URL ve ağ iletişim bilgilerinin statik analizle kolayca ortaya çıkarılması (Information Disclosure / Hardcoded Secrets).

Bu zafiyet, tersine mühendislik yapan saldırganların veya savunma ekiplerinin malware’in komuta-kontrol altyapısını hızlıca haritalandırmasına olanak tanır. Birçok malware, bu bilgileri şifrelemeden veya obfuskasyon uygulamadan binary’ye gömer.

##  Öne Çıkan Özellikler 
1.  **Hata Yönetimi (Result<>):** Kod içerisinde `unwrap()` kullanılmamış, tüm dosya okuma ve ayrıştırma süreçleri Rust'ın `Result` yapısıyla güvenli hale getirilmiştir.
2.  **Otomatik Sınıflandırma:** Bulunan stringler sadece ekrana basılmaz; IPv4 adresleri ve HTTP/HTTPS uç noktaları olarak kategorize edilir.
3.  **Cross-Platform Destek:** Hem Windows (PE) hem de Linux (ELF) dosyalarını aynı motor üzerinden analiz edebilir.
4.  **Unit Testler:** Kodun doğruluğu `cargo test` ile kanıtlanmış, Regex desenleri test senaryolarıyla doğrulanmıştır.

## Adım 1: Kurulum ve Install Analizi

```bash
git clone https://github.com/Gawgaci/string_extractor.git
cd string_extractor
cargo build --release