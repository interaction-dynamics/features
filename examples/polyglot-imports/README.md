# Polyglot imports fixture

Example project used by `projects/cli/tests/polyglot_imports_test.rs`. It checks that dependencies between features are found from the imports of every supported language, not only JavaScript and Rust. Scan it from this folder:

```bash
features scan src --json --skip-changes
```

Each feature imports the next one with the import syntax of its language:

| Feature | File | Import | Depends on |
| --- | --- | --- | --- |
| Checkout | `cart.rb` | `require_relative "../payments/gateway"` | Payments (sibling) |
| Payments | `charge.php` | `require_once "../ledger/ledger.php"` | Ledger (sibling) |
| Ledger | `export.sh` | `source ../reporting/report.sh` | Reporting (sibling) |
| Reporting | `chart.c` | `#include "../theme/colors.h"` | Theme (sibling) |
| Theme | `theme.css` | `@import "../checkout/cart.css"` | Checkout (sibling) |
| Catalog | `loader.cpp` | `#include "./features/search/search.h"` | Search (child) |
| Search | `index.sh` | `. ../../catalog.sh` | Catalog (parent) |
| Orders | Java, Kotlin, C#, Go, Python | external packages only | nothing |
