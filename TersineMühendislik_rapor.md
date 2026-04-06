**Tersine Mühendislik Dersi Vize Projesi**  
**Proje:** Safe String Extractor (Rust-based RE Tool)  
**Repo:** https://github.com/Gawgaci/string_extractor

**Adım 1: Kurulum ve Install Analizi**  
- `git clone` ve `cargo build --release` ile tamamen yerel olarak kuruldu.  
- Dışarıdan `curl | bash` tarzı hiçbir kör kurulum yapılmadı.  
- Bağımlılıklar (object, regex) Cargo’nun hash doğrulamalı sistemiyle indirildi.

**Adım 2: İzolasyon ve İz Bırakmadan Temizlik**  
- `cargo clean` ve `Remove-Item -Path .\target -Recurse -Force` ile temizlik yapıldı.  
- Kalıntı kontrolü ile hiçbir iz kalmadığı doğrulandı.  
- Donanım kısıtlaması nedeniyle sanal makine kullanılamadı.

**Adım 5: Kaynak Kod ve Akış Analizi (Threat Modeling)**  
- Entry point: `main()` fonksiyonu  
- Akış: Binary oku → `object::File::parse()` → section’ları tara → `regex::bytes` ile IP/URL ara.  
- Threat Modeling: Bir hacker bu aracı kullanarak malware içindeki gizli C2 IP ve URL’leri kolayca bulabilir. Rust sayesinde bellek güvenliği (memory safety) sağlanır, buffer overflow gibi zafiyetler compile-time’da engellenir.

**Test:** Windows’un kendi notepad.exe binary’si kullanıldı (güvenli test için).