Nama = Go Nadine Audelia

1. What are the key differences between unary, server streaming, and bi-directional streaming RPC (Remote Procedure Call) methods, and in what scenarios would each be most suitable?
Unary berfungsi untuk melibatkan satu request dan satu response dan cocok untuk operasi CRUD standar. Server streaming memungkinkan satu request dibalas dengan aliran data yang berkelanjutan dari server dan ideal untuk mengunduh file besar atau menerima feed data real-time. Bi-directional streaming memungkinkan kedua belah pihak mengirim dan menerima aliran data secara serentak yang sangat cocok untuk aplikasi interaktif berkelanjutan seperti chat atau game multiplayer.

2. What are the potential security considerations involved in implementing a gRPC service in Rust, particularly regarding authentication, authorization, and data encryption?
Keamanan gRPC di Rust sangat bergantung pada penggunaan Transport Layer Security untuk mengenkripsi data yang transit, mengingat gRPC secara bawaan mengirim data tanpa enkripsi. Selain itu, autentikasi dan otorisasi harus diimplementasikan menggunakan interceptors untuk mencegat dan memvalidasi kredensial pada setiap request. Perlindungan tambahan juga diperlukan untuk membatasi ukuran payload agar mencegah serangan Denial of Service dan menghindari kebocoran memori.

3. What are the potential challenges or issues that may arise when handling bidirectional streaming in Rust gRPC, especially in scenarios like chat applications?
Menangani bi-directional streaming di Rust memiliki tantangan utama dalam hal manajemen state dan konkurensi. Selain itu, server harus tangguh dalam menangani disconnect untuk menghindari koneksi yang memakan memori. 

4. What are the advantages and disadvantages of using the tokio_stream::wrappers::ReceiverStream for streaming responses in Rust gRPC services?
Keuntungan menggunakan ReceiverStream adalah kemudahan dalam menjembatani channel receiver mpsc bawaan Tokio menjadi antarmuka Stream standar yang diwajibkan oleh framework Tonic di Rust. Sedangkan, kelemahannya adalah adanya sedikit lapisan overhead tambahan dari abstraksi ini. Jika ukuran batas buffer pada channel tersebut tidak dikonfigurasi dengan tepat, pesan dapat tertahan atau terbuang saat terjadi lonjakan traffic.

5. In what ways could the Rust gRPC code be structured to facilitate code reuse and modularity, promoting maintainability and extensibility over time?
Untuk memfasilitasi penggunaan kembali dan modularitas kode, struktur proyek gRPC harus memisahkan logika bisnis dari handler layanan gRPC. Hal ini bisa dicapai dengan menempatkan kode Protobuf yang digenerate di module tersendiri dan menggunakan pola Dependency Injection. Dengan cara ini, layanan gRPC hanya bertugas mengurus rute dan format komunikasi, sementara pemrosesan data dilakukan oleh modul independen lain yang bisa dites dan diubah tanpa merusak jaringan komunikasi.

6. In the MyPaymentService implementation, what additional steps might be necessary to handle more complex payment processing logic?
Dalam implementasi yang lebih kompleks, MyPaymentService tidak bisa sekadar mengembalikan nilai sukses statis melainkan harus terhubung ke sistem basis data untuk memvalidasi ketersediaan dana dan mencatat riwayat transaksi. Selain itu, diperlukan integrasi API dengan sistem penyedia payment gateway eksternal untuk otorisasi nyata. Proses ini juga mengharuskan penerapan error handling ketat yang mengonversi kegagalan logika bisnis menjadi kode status gRPC standar.

7. What impact does the adoption of gRPC as a communication protocol have on the overall architecture and design of distributed systems, particularly in terms of interoperability with other technologies and platforms?
Adopsi gRPC memberikan dampak besar pada arsitektur sistem terdistribusi dengan sangat memfasilitasi lingkungan polyglot dimana berbagai layanan yang ditulis dalam bahasa pemrograman berbeda dapat berkomunikasi dengan mulus berkat kode yang digenerate Protobuf. 

8. What are the advantages and disadvantages of using HTTP/2, the underlying protocol for gRPC, compared to HTTP/1.1 or HTTP/1.1 with WebSocket for REST APIs?
HTTP/2 memiliki keunggulan multiplexing yaitu banyak request berjalan paralel dalam satu koneksi, transmisi data berformat biner yang lebih efisien, dan kompresi header yang meringankan beban jaringan. Kelemahannya dibandingkan HTTP/1.1 adalah kerumitan dalam melakukan debugging karena datanya tidak berwujud teks biasa yang bisa dibaca manusia. Selain itu, HTTP/2 masih rentan terhadap masalah antrean di level TCP jika kondisi internet tidak stabil.

9. How does the request-response model of REST APIs contrast with the bidirectional streaming capabilities of gRPC in terms of real-time communication and responsiveness?
Model request-response pada REST API bersifat stateless dan untuk mendapatkan komunikasi real-time, klien harus melakukan teknik polling yang boros dan membebani server. Sedangkan, bi-directional streaming pada gRPC mempertahankan satu koneksi yang terus terbuka tanpa putus. Hal ini memungkinkan server untuk langsung push data baru ke klien kapan saja sehingga membuat respons menjadi instan dan sangat efisien untuk beban kerja tinggi.

10. What are the implications of the schema-based approach of gRPC, using Protocol Buffers, compared to the more flexible, schema-less nature of JSON in REST API payloads?
Pendekatan berbasis skema pada Protobuf membuat payload data terkompresi menjadi ukuran yang sangat kecil dan aman dari kesalahan tipe data, namun sifatnya kaku karena klien dan server harus selalu menyepakati versi proto yang sama. Sedangkan, format JSON pada REST API bersifat schemaless sehingga sangat fleksibel dan mudah ditambahkan parameter baru tanpa merusak sistem klien lama. Namun, JSON menghasilkan ukuran muatan data yang membengkak dan membutuhkan waktu ekstra untuk parsing.