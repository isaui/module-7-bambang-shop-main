# BambangShop Publisher App
Tutorial and Example for Advanced Programming 2024 - Faculty of Computer Science, Universitas Indonesia

---

## About this Project
In this repository, we have provided you a REST (REpresentational State Transfer) API project using Rocket web framework.

This project consists of four modules:
1.  `controller`: this module contains handler functions used to receive request and send responses.
    In Model-View-Controller (MVC) pattern, this is the Controller part.
2.  `model`: this module contains structs that serve as data containers.
    In MVC pattern, this is the Model part.
3.  `service`: this module contains structs with business logic methods.
    In MVC pattern, this is also the Model part.
4.  `repository`: this module contains structs that serve as databases and methods to access the databases.
    You can use methods of the struct to get list of objects, or operating an object (create, read, update, delete).

This repository provides a basic functionality that makes BambangShop work: ability to create, read, and delete `Product`s.
This repository already contains a functioning `Product` model, repository, service, and controllers that you can try right away.

As this is an Observer Design Pattern tutorial repository, you need to implement another feature: `Notification`.
This feature will notify creation, promotion, and deletion of a product, to external subscribers that are interested of a certain product type.
The subscribers are another Rocket instances, so the notification will be sent using HTTP POST request to each subscriber's `receive notification` address.

## API Documentations

You can download the Postman Collection JSON here: https://ristek.link/AdvProgWeek7Postman

After you download the Postman Collection, you can try the endpoints inside "BambangShop Publisher" folder.
This Postman collection also contains endpoints that you need to implement later on (the `Notification` feature).

Postman is an installable client that you can use to test web endpoints using HTTP request.
You can also make automated functional testing scripts for REST API projects using this client.
You can install Postman via this website: https://www.postman.com/downloads/

## How to Run in Development Environment
1.  Set up environment variables first by creating `.env` file.
    Here is the example of `.env` file:
    ```bash
    APP_INSTANCE_ROOT_URL="http://localhost:8000"
    ```
    Here are the details of each environment variable:
    | variable              | type   | description                                                |
    |-----------------------|--------|------------------------------------------------------------|
    | APP_INSTANCE_ROOT_URL | string | URL address where this publisher instance can be accessed. |
2.  Use `cargo run` to run this app.
    (You might want to use `cargo check` if you only need to verify your work without running the app.)

## Mandatory Checklists (Publisher)
-   [x] Clone https://gitlab.com/ichlaffterlalu/bambangshop to a new repository.
-   **STAGE 1: Implement models and repositories**
    -   [x] Commit: `Create Subscriber model struct.`
    -   [x] Commit: `Create Notification model struct.`
    -   [x] Commit: `Create Subscriber database and Subscriber repository struct skeleton.`
    -   [x] Commit: `Implement add function in Subscriber repository.`
    -   [x] Commit: `Implement list_all function in Subscriber repository.`
    -   [x] Commit: `Implement delete function in Subscriber repository.`
    -   [x] Write answers of your learning module's "Reflection Publisher-1" questions in this README.
-   **STAGE 2: Implement services and controllers**
    -   [x] Commit: `Create Notification service struct skeleton.`
    -   [x] Commit: `Implement subscribe function in Notification service.`
    -   [x] Commit: `Implement subscribe function in Notification controller.`
    -   [x] Commit: `Implement unsubscribe function in Notification service.`
    -   [x] Commit: `Implement unsubscribe function in Notification controller.`
    -   [x] Write answers of your learning module's "Reflection Publisher-2" questions in this README.
-   **STAGE 3: Implement notification mechanism**
    -   [ ] Commit: `Implement update method in Subscriber model to send notification HTTP requests.`
    -   [ ] Commit: `Implement notify function in Notification service to notify each Subscriber.`
    -   [ ] Commit: `Implement publish function in Program service and Program controller.`
    -   [ ] Commit: `Edit Product service methods to call notify after create/delete.`
    -   [ ] Write answers of your learning module's "Reflection Publisher-3" questions in this README.

## Your Reflections
This is the place for you to write reflections:

### Mandatory (Publisher) Reflections

#### Reflection Publisher-1
1. Penggunaan single model struct sudah cukup. Hal tersebut karena publisher berinteraksi hanya dengan satu tipe observer maka penggunaan single model struct sudah cukup karena tidak perlu polimorfisme.
2. Penggunaan DashMap disini penting karena kita tahu bahwa id di Program dan url di Subscriber itu unik sehingga kita dapat mempercepat operasi insert, lookup, dan deletion data Program maupun Subscriber dengan kompleksitas O(1), daripada Vec yang mana memperlukan kompleksitas O(N). Itu termasuk pengoptimalan algoritma sehingga dapat mengoptimalkan performa aplikasi rust ini.
3. Kita tetap membutuhkan DashMap, meskipun kita sudah mengimplementasikan singleton pattern yang mana tentu saja struktur data yang ada di dalam singleton itu harus thread safe juga karena singleton tidak sepenuhnya menjamin thread safe. Oleh karena itu diperlukan penggunaan DashMap.
4. Dalam usecase saat ini, menggunakan lazy static variabel yang berisi dashmap dirasa sudah cukup karena dashmap dapat diakses secara thread safe. Selain itu, meskipun seandainya kita mengimplementasikan singleton, itu juga belum menjamin thread safe sehingga dalam singleton tersebut perlu mekanisme thread safe salah satunya dengan menggunakan struktur data yang bersifat thread safety contohnya DashMap.
#### Reflection Publisher-2
1. karena dengan memisahkan service dan repository menjadi layer yang terpisah, kita memiliki fleksibilitas untuk memperbarui satu bagian tanpa harus mempengaruhi bagian yang lain. Selain itu, hal ini juga terkait dengan prinsip single object responsibility, yang mana setiap komponen atau layer harus bertanggung jawab atas satu hal, sehingga hal ini membuat kode mudah dimaintain kedepannya karena kita sudah mengelompokkan kode-kode terkait pada layer yang sesuai dan perubahan  pada layer tersebut tidak akan tidak memengaruhi bagian lain dari layer lainnya.
2. Jika kita hanya menggunakan Model tanpa menggunakan pola MVC, maka kita cenderung akan memiliki logika bisnis, logika tampilan, dan interaksi data menjadi tercampur aduk dalam satu entitas. Ini dapat menyebabkan peningkatan kompleksitas kode, sulitnya pemeliharaan, dan kurangnya fleksibilitas dalam pengembangan aplikasi. Misalnya, jika kita hanya memiliki Model untuk Program, Subscriber, dan Notification, semua logika untuk mengelola data, bisnis, dan tampilan akan berada dalam Model tersebut, menyebabkan Model menjadi terlalu bertanggung jawab dan kompleks. Setiap kali kita perlu mengubah atau menambahkan fitur, kita harus berurusan dengan seluruh logika di dalam Model, meningkatkan risiko kesalahan dan menyulitkan pemeliharaan kode.

Dalam situasi ini, interaksi antara setiap model akan menjadi rumit karena semua tugas, seperti pemrosesan data, validasi, dan logika bisnis, harus ditangani di dalam Model itu sendiri. Hal ini dapat mengarah pada Model yang besar dan sulit dipahami, karena harus menangani banyak tanggung jawab sekaligus. Misalnya, ketika ada interaksi antara Program dan Subscriber, Model Program harus menangani logika yang terkait dengan notifikasi ke Subscriber, pengelolaan langganan, dan lain-lain, yang semuanya meningkatkan kompleksitas kode dan mengurangi keterbacaan. Dengan memisahkan Model ke dalam komponen yang berbeda sesuai dengan pola MVC, kita dapat mengurangi kompleksitas dan meningkatkan pemeliharaan serta fleksibilitas aplikasi.
3. Ya, saya telah menjelajahi Postman dan menemukan bahwa ini adalah alat yang sangat berguna untuk pengujian API dan pengembangan perangkat lunak. Postman membantu saya dengan cepat membuat permintaan HTTP ke endpoint API, memeriksa respons yang diterima, dan mengelola koleksi permintaan. Saya terutama tertarik dengan fitur-fitur seperti pengaturan variabel lingkungan, pengelolaan koleksi, dan kemampuan untuk membuat dan menjalankan skrip pengujian otomatis. Fitur-fitur ini membantu saya mengotomatiskan sebagian besar proses pengujian dan menambah efisiensi dalam pengembangan perangkat lunak, baik itu untuk proyek kelompok atau proyek perangkat lunak masa depan. Dengan Postman, saya dapat dengan mudah menguji fungsionalitas API, memvalidasi permintaan dan respons, serta mengotomatiskan pengujian untuk memastikan kualitas dan keandalan kode yang saya kembangkan.
#### Reflection Publisher-3
