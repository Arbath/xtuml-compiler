# xtUML JSON Model Format

Dokumen ini menjelaskan **format JSON** yang digunakan untuk mendeskripsikan model sistem pada project **xtuml-compiler**. Format ini bertujuan agar:

* Mudah dibaca manusia
* Tetap valid JSON
* Mudah diproses oleh compiler / code generator
* Mewakili konsep **xtUML / domain-driven modeling** (Domain, Class, State Machine, Event)

---

## 1. Struktur Umum

```json
{
  "model_name": "Toko Online",
  "version": "1.0",
  "domains": [],
  "classes": [],
  "events": [],
  "associations": []
}
```

### Penjelasan

| Field        | Tipe   | Wajib  | Deskripsi                       |
| ------------ | ------ | -----  | ------------------------------- |
| `model_name` | string | ✅     | Nama sistem / model             |
| `version`    | string | ❌     | Versi model                     |
| `domains`    | array  | ❌     | Pengelompokan domain (opsional) |
| `classes`    | array  | ✅     | Daftar class dalam sistem       |
| `events`     | array  | ❌     | Event global lintas class       |
| `associations` | array | ✅ | Definisi hubungan antar class |

---

## 2. Domains

Domain digunakan untuk **mengelompokkan class secara konseptual**, bukan teknis.

```json
{
  "name": "UserOrder",
  "description": "Sistem Order Produk"
}
```

| Field         | Tipe   | Wajib  | Deskripsi         |
| ------------- | ------ | -----  | ----------------- |
| `name`        | string | ✅     | Nama domain       |
| `description` | string | ❌     | Penjelasan domain |

Class dapat mereferensikan domain menggunakan `domain_ref`.

---

## 3. Classes

Setiap class merepresentasikan **entity utama** dalam sistem.

```json
{
  "name": "Customer",
  "domain_ref": "UserOrder",
  "attributes": [],
  "methods": [],
  "state_model": []
}
```

### Field Class

| Field         | Tipe   | Wajib  | Deskripsi                |
| ------------- | ------ | -----  | ------------------------ |
| `name`        | string | ✅     | Nama class               |
| `domain_ref`  | string | ❌     | Referensi ke domain      |
| `attributes`  | array  | ❌     | Atribut data             |
| `methods`     | array  | ❌     | Method / behavior        |
| `state_model` | array  | ❌     | State machine (jika ada) |

---

## 4. Attributes

Atribut adalah **data yang dimiliki class**.

```json
{ "name": "email", "type": "str" }

```

| Field | Tipe | Wajib | Deskripsi |
| --- | --- | --- | --- |
| `name` | string | ✅ | Nama atribut |
| `type` | string | ✅ | Tipe data domain (lihat tabel di bawah) |

### Supported Data Types (Tipe Data yang Didukung)

Compiler secara otomatis menerjemahkan tipe data generik/xtUML menjadi tipe data JavaScript yang valid (untuk JSDoc dan Logic).

| Kategori | Tipe di JSON (Input) | Hasil di JavaScript (Output) | Contoh Penggunaan |
| --- | --- | --- | --- |
| **Text** | `str`, `string`, `text`, `char`, `email` | `string` | Nama, Alamat, Deskripsi |
| **Identifier** | `uuid`, `unique_id` | `string` | Primary Key, Foreign Key |
| **Number** | `int`, `integer`, `long` | `number` | ID Auto Inc, Stok, Urutan |
| **Decimal** | `float`, `real`, `double`, `decimal` | `number` | Harga, Berat, Persentase |
| **Boolean** | `bool`, `boolean` | `boolean` | `isActive`, `isDeleted` |
| **Date/Time** | `datetime`, `timestamp`, `date` | `Date` | `createdAt`, `birthDate` |
| **Object** | `json`, `jsonb`, `map`, `binary`, `obj` | `Object` | Metadata, Config, Blob |
| **Collection** | `list`, `array`, `inst_ref_set` | `Array` | List item sederhana |
| **Void** | `void` | `void` | Return type method kosong |
| **Lainnya** | *(tidak dikenali)* | `any` | Fallback type |

> **Catatan:** Semua tipe data di atas bersifat informasi sehingga implementasinya dilakukan pada tingkat database.

---

### Contoh Implementasi di JSON

```json
"attributes": [
  { "name": "orderID", "type": "uuid" },       // -> string
  { "name": "totalAmount", "type": "decimal" }, // -> number
  { "name": "isPaid", "type": "bool" },         // -> boolean
  { "name": "items", "type": "list" },          // -> Array
  { "name": "meta", "type": "json" }            // -> Object
]

```

## 5. Methods

Method mendeskripsikan **behavior class**.

```json
{
  "name": "calculate_total",
  "body": "let total = 0;\nthis.total_amount = total;"
}
```

### Catatan Penting tentang `body`

* `body` adalah **string JavaScript-like**
* Multi-line dilakukan menggunakan `\n`
* JSON **tidak mendukung multiline string native**

Contoh:

```json
"body": "line1\nline2\nline3"
```

> Disarankan untuk **tidak membuat logic terlalu kompleks** di JSON.
> JSON hanya sebagai **deskripsi model**, bukan source code utama.

---

## 6. State Model (State Machine)

State model digunakan untuk class yang memiliki **lifecycle**.

```json
"state_model": [
  {
    "initial_state": "Created",
    "states": [],
    "transitions": []
  }
]
```

### State Model Fields

| Field           | Tipe   | Wajib  | Deskripsi            |
| --------------- | ------ | -----  | -------------------- |
| `initial_state` | string | ✅     | State awal           |
| `states`        | array  | ✅     | Daftar state         |
| `transitions`   | array  | ✅     | Transisi antar state |

### State

```json
{ "name": "Confirmed" }
```

### Transition

```json
{
  "from": "Created",
  "event": "OrderConfirmed",
  "to": "Confirmed",
  "action": "calculate_total()"
}
```

| Field    | Tipe   | Wajib  | Deskripsi          |
| -------- | ------ | -----  | ------------------ |
| `from`   | string | ✅     | State asal         |
| `event`  | string | ✅     | Event pemicu       |
| `to`     | string | ✅     | State tujuan       |
| `action` | string | ❌     | Aksi saat transisi |

---

## 7. Events

Event merepresentasikan **kejadian bisnis** yang bisa memicu perubahan state atau aksi.

```json
{
  "name": "OrderConfirmed",
  "trigger": "Order",
  "action": "status = 'confirmed'"
}
```

| Field     | Tipe   | Wajib  | Deskripsi    |
| --------- | ------ | -----  | ------------ |
| `name`    | string | ✅     | Nama event   |
| `trigger` | string | ❌     | Class pemicu |
| `action`  | string | ❌     | Aksi event   |

Event dapat digunakan oleh:

* Method
* State transition
* Code generator

---

## 8. Associations (Hubungan)

Bagian ini mendefinisikan hubungan antar class. Format JSON mendukung **4 jenis asosiasi**:

1. **Binary** (1-1, 1-N, N-N)
2. **Linked** (Many-to-Many dengan Class Perantara)
3. **Generalization** (Inheritance / Pewarisan)
4. **Reflexive** (Hubungan ke diri sendiri, menggunakan struktur Binary)

### Field Umum Association

| Field | Tipe | Wajib | Deskripsi |
| --- | --- | --- | --- |
| `rel_id` | string | ✅ | ID unik relasi (contoh: R1, R2) |
| `type` | string | ✅ | Jenis relasi (`binary`, `linked`, `generalization`) |

---

### A. Binary & Reflexive Association

Digunakan untuk hubungan standar antar dua class atau class yang sama (Reflexive).

```json
{
  "rel_id": "R1",
  "type": "binary",
  "side_a": {"class": "Customer", "mult": "1", "phrase": "melakukan"},
  "side_b": {"class": "Order", "mult": "0..*", "phrase": "dilakukan oleh"}
}

```

**Field Side A & Side B:**

| Field | Tipe | Deskripsi |
| --- | --- | --- |
| `class` | string | Nama class target |
| `mult` | string | Multiplicity (`1`, `0..1`, `0..*`, `1..*`) |
| `phrase` | string | Kata kerja yang menjelaskan peran (untuk code) |

---

### B. Linked Association (Associative Class)

Digunakan untuk hubungan **Many-to-Many** yang memiliki data tambahan, sehingga membutuhkan class perantara (*Link Class*).

```json
{
  "rel_id": "R2",
  "type": "linked",
  "side_a": { "class": "Order", "mult": "0..*", "phrase": "berisi" },
  "side_b": { "class": "Product", "mult": "0..*", "phrase": "termasuk" },
  "link_class": "OrderItem"
}

```

| Field | Tipe | Deskripsi |
| --- | --- | --- |
| `link_class` | string | Nama class perantara (wajib ada di `classes`) |

---

### C. Generalization (Inheritance)

Digunakan untuk konsep pewarisan (Parent-Child).

```json
{
  "rel_id": "R3",
  "type": "generalization",
  "superclass": "Payment",
  "subclasses": ["CreditCard", "BankTransfer"]
}

```

| Field | Tipe | Deskripsi |
| --- | --- | --- |
| `superclass` | string | Class Induk (Parent) |
| `subclasses` | array | Daftar Class Anak (Children) |

---

## 9. Prinsip Desain Format

* Human readable
* Valid JSON
* Mudah di-extend
* Cocok untuk code generation
* Mendukung xtUML concept

#### Yang **Tidak Disarankan**

* Menulis business logic kompleks di `body`
* Mengandalkan JSON sebagai full source code
* Menyimpan multiline string tanpa `\n`

---

## 10. Alur Umum Penggunaan

1. Buat file JSON model
2. Jalankan `xtuml-compiler`
3. Compiler:

   * Parse JSON
   * Bangun AST
   * Generate JavaScript / target lain

---

## 11. Contoh Use Case

* Backend code generator
* Prototype domain model
* Dokumentasi arsitektur
* Event-driven system modeling

---

📌 **Catatan Akhir**

Format ini dirancang sebagai **model definition**, bukan implementation detail.
Jika butuh logic kompleks, letakkan di layer implementasi, bukan di JSON.
