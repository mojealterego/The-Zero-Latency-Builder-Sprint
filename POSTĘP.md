# POSTĘP — The Zero Latency Builder Sprint

**Data aktualizacji:** 2026-09-17  
**Repozytorium:** https://github.com/mojealterego/The-Zero-Latency-Builder-Sprint  
**Zasada raportowania:** rozróżniamy stan potwierdzony, propozycje i zadania niewykonane. Brak pomiaru/testu nie jest wynikiem pozytywnym.

## Etap 1 — Rozpoznanie dokumentacji repozytorium

**Status: wykonano — przegląd dokumentacji źródłowej, bez uruchamiania kodu.**

- Potwierdzono, że repozytorium jest publiczne, gałąź domyślna `main`.
- Przejrzano `README2.md`: opisuje architekturę jako propozycję; deklaruje, że pomiary wydajności i integracja Moss nie są potwierdzone.
- Przejrzano `RUNTIME-SECURITY-REVIEW.md`: to przegląd źródeł komponentów z `Nowe-projekty`; testy nie były uruchomione. Nie należy traktować tych komponentów jako już zintegrowanych z repozytorium sprintu.
- Odczytano `SOURCE-COMPONENT-MATRIX.md`; szczegóły poniżej.

## Etap 2 — Analiza macierzy komponentów źródłowych

**Status: wykonano — wnioski oparte na inspekcji źródeł opisanej w macierzy.**

Kandydat do pionowego wycinka: lokalny wybór kontekstu + kontrolowane wykonanie narzędzia + ślad audytowy/replay. Macierz wskazuje istniejące komponenty w osobnym repozytorium `Nowe-projekty`, nie w bieżącym repozytorium sprintu.

Ryzyka wymagające testów/naprawy przed użyciem:

1. Nieprawidłowe daty faktur mogą propagować `NaN` do wyniku.
2. Brak kontroli zgodności `invoice.customerId` z `history.customerId`.
3. Walidacja adresu e-mail oparta tylko o obecność `@`.
4. Funkcja wysyłki zwraca plan `approved_for_delivery`; nie wysyła wiadomości do zewnętrznego dostawcy.
5. Wykrywanie destrukcyjnych akcji może ominąć nazwy z przestrzenią nazw, np. `collections.send_message`.
6. Brak pełnej walidacji wejściowych wartości polityki (kształt danych, koszt skończony/nieujemny, dozwolony poziom ryzyka).

Macierz rekomenduje testy m.in. dla błędnych danych, rozbieżności tożsamości, opt-out, nieznanych akcji ubocznych, replay/kolizji idempotencyjnej oraz zmian wersji polityki.

## Etap 3 — Minimalny szkielet Rust i lokalny retrieval

**Status: pliki zapisane na `main`; kompilacja/testy nieuruchomione w tym środowisku.**

Dodano:

- `Cargo.toml` — minimalny crate Rust, bez zewnętrznych zależności.
- `src/lib.rs` — typy `ContextRecord`/`RankedRecord` i deterministyczny leksykalny selektor kontekstu: liczy dopasowane unikalne terminy, rozstrzyga remisy po ID, ogranicza wynik parametrem `limit`.
- `src/main.rs` — CLI demonstrator z małym, wbudowanym korpusem; przyjmuje zapytanie z argumentów.

Testy jednostkowe zapisane w `src/lib.rs`: ranking, deterministyczne remisy, limit zerowy i brak dopasowania. **To są testy w kodzie, nie potwierdzenie ich wykonania.**

Ograniczenia i ryzyka:

- To bazowy lexical matching, nie BM25, nie semantyczne wyszukiwanie i nie integracja Moss.
- Brak Zenoh, SQLite, trwałego event logu, warstwy polityk i telemetryki.
- Nie zmierzono latency; brak podstaw do deklaracji 1 ms / sub-10 ms.
- W tej sesji nie było lokalnego środowiska kompilacji/testów; potrzebny `cargo test` i `cargo run -- "local context"` w środowisku z Rust.

## Decyzje architektoniczne robocze

- Rust + Zenoh pozostaje kierunkiem do zweryfikowania, a nie gotową implementacją.
- Moss musi być rzeczywiście użyty jako warstwa retrieval i mieć adapter oparty na zweryfikowanym API/wersji; nie wolno zastępować go samym transportem Zenoh.
- Cel „1 ms” wymaga precyzyjnej granicy pomiaru i benchmarku. Nie jest obecnie potwierdzonym wynikiem.
- Nie przedstawiać symulowanej wysyłki jako rzeczywistej dostawy.

## Stan wykonania

| Obszar | Stan |
|---|---|
| Repozytorium i dokumenty | Odczytane: README2, przegląd bezpieczeństwa, macierz komponentów |
| Rust crate | Dodano manifest, bibliotekę i CLI; kompilacja niezweryfikowana |
| Unit tests | Zapisano 4 testy; nieuruchomione |
| Moss | Niezaimplementowany; API/wersja do weryfikacji |
| Zenoh | Niezaimplementowane |
| Trwałość/audyt/replay | Niezaimplementowane |
| Benchmark p50/p95/p99 | Brak |

## Następne kroki

1. Uruchomić `cargo test` i naprawić ewentualne błędy kompilacji/testów.
2. Dodać reproducible benchmark lokalnego selektora z raportem p50/p95/p99 i opisem sprzętu/korpusu.
3. Zweryfikować oficjalne wymagania sprintu i API Moss, po czym dodać adapter rzeczywiście wywołujący Moss.
4. Dodać kontrakt zdarzenia i deterministyczny replay; następnie bramkę polityki dla kontrolowanego narzędzia.
5. Dodać Zenoh tylko dla uzasadnionej ścieżki komunikacyjnej i zmierzyć osobno koszt transportu.

---

## Źródła wewnętrzne

- `README2.md`
- `RUNTIME-SECURITY-REVIEW.md`
- `SOURCE-COMPONENT-MATRIX.md`
