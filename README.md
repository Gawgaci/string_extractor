# Proje Önerisi: Zararlı Yazılım Analizi İçin Güvenli String Çıkarıcı Aracı

**Öğrenci:** Burhan Yavaş
**Üniversite / Bölüm:** İstinye Üniversitesi / Bilişim Güvenliği Teknolojisi
**Ders:** Tersine Mühendislik

## 1. Projenin Amacı ve Kapsamı
Bu projenin temel amacı, derlenmiş çalıştırılabilir dosyalardan (PE, ELF) ve bellek dökümlerinden (memory dumps) okunabilir metin dizilerini tespit edip çıkaran, zararlı yazılım statik analiz süreçleri için optimize edilmiş bir araç geliştirmektir. Proje; gizli anahtarların, C2 (Command & Control) IP adreslerinin, URL'lerin ve API çağrılarının tespitini güvenli bir mimari üzerinden otomatize etmeyi hedeflemektedir.

## 2. Teknik Altyapı ve Metodoloji
* **Programlama Dili:** Rust da . Zararlı yazılımların ayrıştırılması sırasında oluşabilecek bellek zafiyetlerinden buffer overflow gibi. kaçınmak ve memory-safe bir analiz aracı geliştirmek amacıyla tercih edilmiştir.
* **Geliştirme Ortamı:** Ubuntu Linux.
* **Ayrıştırma ve Filtreleme:** Binary üzerinden okunan ham metinler içerisindeki potansiyel ağ göstergeleri (IP/URL) ve dosya yolları, Regex kullanılarak filtrelenecek ve anlamlı kategorilere ayrılacaktır. Gerekli durumlarda temel disassembly işlemleri için Capstone engine entegrasyonu sağlanacaktır.
* **Güvenlik Standartları:** Proje kapsamında ağ tabanlı bir işlem veya kriptografik süreç gerekmesi durumunda, OpenSSL'in bilinen zafiyetlerinden kaçınmak adına `rustls` ve `ring` kullanılacaktır.

## 3. Versiyon Kontrolü ve Süreç Yönetimi
Geliştirme sürecinin tamamı GitHub üzerinden şeffaf bir şekilde yürütülecektir. Proje reposunda "updated" gibi jenerik commit mesajları kullanılmayacak; bunun yerine yapılan işlemi net bir şekilde özetleyen anlamlı ve profesyonel commit mesajları tercih edilecektir.

## 4. Proje Kilometre Taşları (3 Günlük Çevik/Agile Sprint)
Teslimat süresinin kısalığı göz önüne alınarak, geliştirme süreci 3 günlük yoğun bir sprint olarak planlanmıştır:
* **Gün 1 (Mimari ve Temel Ayrıştırma):** `object` crate kullanılarak PE/ELF yapılarını tanıyan (section-aware) okuma altyapısının kurulması. (Sadece `.data`, `.rdata` gibi ilgili bölümlerin hedeflenmesi).
* **Gün 2 (Analiz Motoru ve Heuristikler):** Çoklu kodlama desteği (ASCII ve UTF-16LE) ile Regex tabanlı C2 IP, URL ve API sınıflandırma motorunun entegrasyonu.
* **Gün 3 (Test, Raporlama ve Teslimat):** İzole Sandbox ortamında zararlı yazılım hash'leri ile testlerin yapılması, çıktıların **JSON** formatına dönüştürülmesi ve mevcut araçlarla (GNU `strings`) benchmark karşılaştırması.