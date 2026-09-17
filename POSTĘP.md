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

**Status: pliki zapisane na `main`; kod benchmarku został uruchomiony w środowisku użytkownika, ale nie ma niezależnego logu CI.**

Dodano:

- `Cargo.toml` — minimalny crate Rust, bez zewnętrznych zależności.
- `src/lib.rs` — typy `ContextRecord`/`RankedRecord` i deterministyczny leksykalny selektor kontekstu: liczy dopasowane unikalne terminy, rozstrzyga remisy po ID, ogranicza wynik parametrem `limit`.
- `src/main.rs` — CLI demonstrator z małym, wbudowanym korpusem; przyjmuje zapytanie z argumentów.
- `benches/retrieval.rs` — syntetyczny benchmark lokalnego selektora.

W `src/lib.rs` zapisano 7 testów jednostkowych: ranking, deterministyczne remisy, limit zerowy, brak dopasowania, dopasowanie pełnych tokenów bez rozróżniania wielkości liter, deduplikacja terminów zapytania i puste zapytanie. Testy są zapisane w kodzie; nie mam potwierdzenia ich wykonania w CI.

Użytkownik uruchomił `cargo bench --bench retrieval`. Zrzut terminala pokazuje zakończony benchmark: 1 000 rekordów, 1 000 iteracji, mean 70 226 ns, p50 68 337 ns, p95 75 497 ns, p99 128 796 ns. To syntetyczny microbenchmark jednego procesu, nie pomiar produkcyjnej ścieżki ani integracji Moss.

Ograniczenia:

- To bazowy lexical matching, nie BM25, nie semantyczne wyszukiwanie i nie integracja Moss.
- Brak Zenoh, SQLite, trwałego event logu, warstwy polityk i telemetryki.
- Wyniki zależą od środowiska; należy zapisać sprzęt, wersję Rust, profil i dokładną komendę.

## Etap 4 — CI dla Rust

**Status: workflow dodany do `main`; jego uruchomienie i wynik nie zostały jeszcze zweryfikowane.**

Dodano `.github/workflows/rust-ci.yml`, który na push do `main` i pull request uruchamia:

1. `cargo test --locked`
2. `cargo bench --bench retrieval`

Workflow ma `contents: read`. Nie oznacza to jeszcze, że testy przechodzą — sprawdzić kartę Actions po uruchomieniu.

## Decyzje architektoniczne robocze

- Rust + Zenoh pozostaje kierunkiem do zweryfikowania, a nie gotową implementacją.
- Moss musi być rzeczywiście użyty jako warstwa retrieval i mieć adapter oparty na zweryfikowanym API/wersji; nie wolno zastępować go samym transportem Zenoh.
- Cel „1 ms” wymaga precyzyjnej granicy pomiaru i benchmarku. Nie jest obecnie potwierdzonym wynikiem.
- Nie przedstawiać symulowanej wysyłki jako rzeczywistej dostawy.

## Kolejne kroki — kolejność wykonania

1. **Sprawdzić GitHub Actions** dla ostatniego commita: potwierdzić wynik `cargo test --locked` i benchmarku; naprawić błędy, jeśli wystąpią.
2. **Zwiększyć wiarygodność benchmarku:** dodać rozgrzewkę, wielokrotne serie, raport środowiska i rozdzielić koszt tokenizacji od selekcji; zachować surowe wyniki.
3. **Zweryfikować specyfikację hackathonu i oficjalne API Moss:** ustalić obowiązkowe użycie, SDK/wersję, sposób wywołania oraz format wejścia/wyjścia. Nie implementować adaptera na podstawie zgadywania.
4. **Zbudować adapter Moss za interfejsem:** jawny timeout, limit wyników i tokenów, obsługa błędów, testy kontraktowe; lokalny lexical fallback musi być oznaczony jako fallback.
5. **Dodać mały zestaw jakości retrieval:** pytania + oczekiwane dokumenty, Recall@k/MRR oraz porównanie lexical vs Moss.
6. **Dodać kontrakt zdarzenia i replay deterministyczny:** identyfikator, czas, typ, wejście, wersja polityki, wynik; testy powtórzeń i kolizji idempotencyjnej.
7. **Dodać bramkę polityki przed wykonaniem narzędzia:** walidacja schematu, allowlist akcji, tryb dry-run, jawne potwierdzenie dla skutków zewnętrznych.
8. **Dopiero potem rozważyć Zenoh/SQLite** na podstawie mierzalnej potrzeby; mierzyć oddzielnie retrieval, transport i pełny end-to-end.
9. **Przygotować demo:** jeden scenariusz od zapytania do cytowanego kontekstu, z widocznym śladem i porównywalnymi pomiarami.

## Stan wykonania

| Obszar | Stan |
|---|---|
| Dokumentacja repozytorium | Odczytana |
| Rust crate + CLI | Dodane; CI do weryfikacji |
| Unit tests | 7 testów zapisanych; brak potwierdzonego wyniku CI |
| Benchmark | Uruchomiony przez użytkownika; wyniki syntetyczne |
| GitHub Actions | Workflow dodany; wynik niezweryfikowany |
| Moss | Niezaimplementowany; API/wersja do weryfikacji |
| Zenoh | Niezaimplementowane |
| Trwałość/audyt/replay | Niezaimplementowane |
| Benchmark end-to-end | Brak |

---

## Źródła wewnętrzne

- `README2.md`
- `RUNTIME-SECURITY-REVIEW.md`
- `SOURCE-COMPONENT-MATRIX.md`
