# 🚀 Akademik Proje Yol Haritası: Güvenlik ve Tersine Mühendislik

Bu doküman, "Güvenli Web Yazılımı Geliştirme" ve "Tersine Mühendislik" dersleri kapsamında yürütülecek olan üç bağımsız projenin teknik uygulama adımlarını ve analiz hedeflerini içermektedir.

---

## 🏗️ PROJE 1: Gitleaks Altyapı ve Güvenlik Analizi (Zorunlu Ortak Kısım)
*Hedef: Kategori 1 (SecOps) listesinde bulunan Gitleaks açık kaynak projesinin kurulum, izolasyon, CI/CD ve kaynak kod güvenliğini "Güvenlik Uzmanı" perspektifiyle denetlemek.*

### Adım 1: Kurulum ve `install.sh` Analizi (Reverse Engineering)
- **Görev:** Gitleaks kurulum scriptinin bağımlılık çekme yöntemlerini ve yetki taleplerini analiz etmek.
- **Kritik Soru:** Bash script üzerinden çekilen paketlerde imza (checksum/hash) doğrulaması yapılıyor mu, yoksa Supply Chain (Tedarik Zinciri) saldırılarına açık bir `curl | bash` mantığı mı yürütülüyor?

### Adım 2: İzolasyon ve İz Bırakmadan Temizlik (Forensics)
- **Görev:** Aracın sanal makine (VM) ortamına kurularak test edilmesi ve ardından tamamen kaldırılması.
- **Kritik Soru:** Kurulum sonrası sistemde kalan log dosyaları, ortam değişkenleri (environment variables) ve kalıntı dizinler Forensic yöntemlerle nasıl tespit edilip temizlenir?

### Adım 3: İş Akışları ve CI/CD Pipeline Analizi (.github/workflows)
- **Görev:** Gitleaks reposundaki GitHub Actions dosyalarının (build, test, release) incelenmesi.
- **Kritik Soru:** Pipeline üzerindeki "Webhook" tetikleyicileri otomasyonu nasıl yönetiyor ve dış müdahalelere karşı ne kadar güvenli?

### Adım 4: Docker Mimarisi ve Konteyner Güvenliği
- **Görev:** Projenin `Dockerfile` katmanlarının analiz edilmesi.
- **Kritik Soru:** İmaj oluşturulurken "Least Privilege" (En Az Yetki) prensibine uyulmuş mu? Konteynerin host sisteme erişimi nasıl kısıtlanmıştır?

### Adım 5: Kaynak Kod ve Tehdit Modelleme (Threat Modeling)
- **Görev:** Projenin ana giriş noktasını (Entrypoint) incelemek ve Regex motorunun çalışma mantığındaki riskleri bulmak.
- **Kritik Soru:** Bir saldırgan araca kasıtlı olarak devasa ve karmaşık metinler (ReDoS attack) göndererek sistemin çökmesini sağlayabilir mi?

---

## 🛡️ PROJE 2: Güvenli Web Yazılımı Geliştirme (Bağımsız Proje)
**Konu: Kimlik Doğrulama ve Oturum Güvenliği (JWT Security)**

- **Amacı:** Web uygulamalarında sıkça karşılaşılan "Broken Authentication" (Bozuk Kimlik Doğrulama) zafiyetlerini engellemek için güvenli bir JSON Web Token (JWT) mimarisi kurgulamak.
- **Uygulama Adımları:**
    1. Token imzalama sürecinde zayıf algoritmaların (örn: algoritma = none) engellenmesi.
    2. Payload (veri) kısmında hassas bilgi taşınmasının önüne geçilmesi.
    3. Token süresi (Expiration) ve yenileme (Refresh Token) mekanizmalarının güvenli bir şekilde implemente edilmesi.
- **Senaryo:** Dışarıdan yetkisiz bir kullanıcının token manipülasyonu ile Admin paneline erişimini engelleme.

---

## 🔍 PROJE 3: Tersine Mühendislik (Bağımsız Proje)
**Konu: 8. String Çıkarıcı (Safe String Extractor) - RUST İLE TAMAMLANDI**

- **Amacı:** Zararlı yazılımların (Malware) PE/ELF bölümlerindeki gizli C2 (Komuta Kontrol) sunucu adreslerini, IP'leri ve URL'leri tespit etmek.
- **Uygulama Adımları:**
    1. İkili (Binary) dosyanın `object` kütüphanesi ile bölüm farkındalıklı (Section-Aware: .text, .data vb.) olarak parse edilmesi.
    2. Düzenli ifadeler (Regex) kullanılarak IP, URL ve gizli şifrelerin makine kodundan ayıklanması.
    3. Bulguların diğer siber güvenlik araçlarıyla entegre olabilmesi için `JSON` formatında yapılandırılmış (Structured) olarak dışa aktarılması.
- **Araç Kutusu:** `Rust`, `object`, `regex`, `serde_json`
- **Durum:** Kodlandı, test edildi ve GitHub reposu oluşturuldu.

---

## 📅 Takvim ve Teslimat
1. **1. Aşama:** Proje 3 (String Extractor) - Tamamlandı ve GitHub'a eklendi.
2. **2. Aşama:** Proje 1 (Gitleaks) - 5 Adımlık Teknik Analiz Raporunun Hazırlanması.
3. **3. Aşama:** Proje 2 (JWT Güvenliği) - Web zafiyeti koruma senaryosunun kodlanması.
