# POSTĘP — The Zero Latency Builder Sprint

**Data aktualizacji:** 2026-09-17  
**Repozytorium:** https://github.com/mojealterego/The-Zero-Latency-Builder-Sprint  
**Zasada raportowania:** rozróżniamy stan potwierdzony, propozycje i zadania niewykonane. Brak pomiaru/testu nie jest wynikiem pozytywnym.

## Stan bazowy

- Rust crate, CLI i leksykalny selektor są zapisane w repozytorium.
- W terminalu użytkownika wykonano syntetyczny benchmark: 1 000 rekordów/iteracji, mean 70 226 ns, p50 68 337 ns, p95 75 497 ns, p99 128 796 ns. To nie jest wynik produkcyjny ani end-to-end.
- Workflow `.github/workflows/rust-ci.yml` uruchamia testy i benchmark; jego aktualnego wyniku nie potwierdzono w tej aktualizacji.
- Moss, Zenoh, trwały event log, runtime policy gate i rzeczywisty replay pozostają niezaimplementowane.

## Wykonano teraz — 5 zadań

### 1. Ulepszono benchmark

`benches/retrieval.rs` ma teraz rozgrzewkę i konfigurowalne zmienne środowiskowe: `BENCH_RECORDS`, `BENCH_WARMUP`, `BENCH_ITERATIONS`, `BENCH_TOP_K`, `BENCH_QUERY`. Raportuje średnią oraz p50/p95/p99 i jawnie opisuje ograniczenie syntetycznego microbenchmarku. Sprzęt i wersję Rust nadal trzeba dopisać do raportu przy uruchomieniu.

### 2. Dodano testy regresji jakości retrieval

`tests/retrieval_quality.rs` sprawdza oczekiwany top-2, odrzucanie dokumentu niepowiązanego i twardy limit top-k. Są to testy zapisane w repo; nie twierdzę, że przeszły — wynik CI wymaga sprawdzenia.

### 3. Zapisano kontrakt replay/audytu

`docs/REPLAY_AND_AUDIT.md` definiuje kopertę zdarzenia, sekwencjonowanie, kontrolę integralności, zasady replay bez wykonywania skutków ubocznych i testy akceptacyjne. To specyfikacja, nie gotowy event store.

### 4. Zapisano kontrakt polityki narzędzi

`docs/TOOL_POLICY_CONTRACT.md` definiuje walidację allowlisty i schematu, decyzje allow/require_approval/deny, powiązanie akceptacji z hashem żądania, idempotency i fail-closed. To specyfikacja; runtime enforcement nie jest jeszcze zaimplementowany.

### 5. Zapisano bramkę integracji Moss

`docs/MOSS_ADAPTER_GATE.md` określa wąski wewnętrzny interfejs i checklistę weryfikacji oficjalnej dokumentacji, wersji SDK, lokalizacji danych, błędów i kryteriów akceptacji. Nie wpisano zmyślonych endpointów ani sygnatur. Integracja Moss nadal jest zablokowana do czasu weryfikacji oficjalnego API.

## Linki do zmian

- [Benchmark](https://github.com/mojealterego/The-Zero-Latency-Builder-Sprint/blob/main/benches/retrieval.rs)
- [Testy jakości](https://github.com/mojealterego/The-Zero-Latency-Builder-Sprint/blob/main/tests/retrieval_quality.rs)
- [Replay i audyt](https://github.com/mojealterego/The-Zero-Latency-Builder-Sprint/blob/main/docs/REPLAY_AND_AUDIT.md)
- [Polityka narzędzi](https://github.com/mojealterego/The-Zero-Latency-Builder-Sprint/blob/main/docs/TOOL_POLICY_CONTRACT.md)
- [Bramka Moss](https://github.com/mojealterego/The-Zero-Latency-Builder-Sprint/blob/main/docs/MOSS_ADAPTER_GATE.md)

## Następne działania

1. Sprawdzić wynik GitHub Actions po ostatnich commitach i naprawić ewentualne błędy kompilacji/testów.
2. Uruchomić benchmark z zapisaniem `rustc --version`, OS/CPU/RAM i parametrów.
3. Zweryfikować obowiązki hackathonu oraz aktualne oficjalne API Moss, po czym zrealizować adapter.
4. Zaimplementować i przetestować policy gate oraz event log/replay — dokumenty same nie zapewniają ochrony.
5. Dopiero po testach połączyć ścieżkę end-to-end i raportować osobno jakość oraz latency.
