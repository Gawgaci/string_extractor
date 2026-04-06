# SAFE STRING EXTRACTOR

**Tersine Mühendislik Dersi Vize Projesi**

Bu proje, zararlı yazılımların (malware) PE ve ELF formatındaki ikili (binary) dosyalarından gizli C2 (Command & Control) sunucu adreslerini, IP’leri ve URL’leri otomatik olarak tespit eden Rust tabanlı bir tersine mühendislik aracıdır.

## Projenin Amacı

Modern malware’ler C2 adreslerini, callback URL’lerini ve komuta-kontrol altyapısını binary içinde gizler. Klasik `strings` komutu çok fazla false positive verir ve section farkındalığı yoktur. Bu araç, **section-aware parsing** yöntemiyle sadece ilgili binary bölümlerini (.text, .data, .rsrc vb.) tarayarak şüpheli ağ göstergelerini daha doğru ve hızlı bir şekilde çıkarır.

## İncelenen Zafiyet

**Zafiyet:** Malware’lerin binary içinde gizlediği C2 IP/URL ve ağ iletişim bilgilerinin statik analizle kolayca ortaya çıkarılması (Information Disclosure / Hardcoded Secrets).

Bu zafiyet, tersine mühendislik yapan saldırganların veya savunma ekiplerinin malware’in komuta-kontrol altyapısını hızlıca haritalandırmasına olanak tanır. Birçok malware, bu bilgileri şifrelemeden veya obfuskasyon uygulamadan binary’ye gömer.

## Zafiyet Çözümü ve Kullanılan Teknikler

- **Section-Aware Parsing**: `object` crate’i ile binary’nin sadece ilgili section’larını tarar. Tüm binary’yi byte-byte taramak yerine, `.text`, `.data`, `.rsrc` gibi bölümleri tek tek inceler.
- **Regex ile Pattern Matching**: `regex::bytes` kullanılarak binary seviyesinde IP ve HTTP/HTTPS URL desenleri aranır.
- **Memory Safety**: Rust dili sayesinde buffer overflow, use-after-free gibi klasik C/C++ zafiyetleri compile-time’da engellenir.
- **Tam Error Handling**: Hiçbir yerde `unwrap()` kullanılmamıştır.

Bu sayede araç hem daha güvenli hem de daha az false positive üretir.

## Adım 1: Kurulum ve Install Analizi

```bash
git clone https://github.com/Gawgaci/string_extractor.git
cd string_extractor
cargo build --release