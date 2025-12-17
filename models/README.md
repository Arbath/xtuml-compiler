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
  "events": []
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

| Field  | Tipe   | Wajib  | Deskripsi                                  |
| ------ | ------ | -----  | ------------------------------------------ |
| `name` | string | ✅     | Nama atribut                               |
| `type` | string | ✅     | Tipe data (int, str, float, datetime, dll) |

---

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

## 8. Prinsip Desain Format

✔ Human readable
✔ Valid JSON
✔ Mudah di-extend
✔ Cocok untuk code generation
✔ Mendukung xtUML concept

### Yang **Tidak Disarankan**

* Menulis business logic kompleks di `body`
* Mengandalkan JSON sebagai full source code
* Menyimpan multiline string tanpa `\n`

---

## 9. Alur Umum Penggunaan

1. Buat file JSON model
2. Jalankan `xtuml-compiler`
3. Compiler:

   * Parse JSON
   * Bangun AST
   * Generate JavaScript / target lain

---

## 10. Contoh Use Case

* Backend code generator
* Prototype domain model
* Dokumentasi arsitektur
* Event-driven system modeling

---

📌 **Catatan Akhir**

Format ini dirancang sebagai **model definition**, bukan implementation detail.
Jika butuh logic kompleks, letakkan di layer implementasi, bukan di JSON.
