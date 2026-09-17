# POSTĘP — The Zero Latency Builder Sprint

**Data aktualizacji:** 2026-09-17  
**Repozytorium:** https://github.com/mojealterego/The-Zero-Latency-Builder-Sprint  
**Zasada raportowania:** rozróżniamy stan potwierdzony, propozycje i zadania niewykonane. Brak pomiaru/testu nie jest wynikiem pozytywnym.

## Stan bazowy

- Rust crate, CLI i leksykalny selektor są zapisane w repozytorium.
- W terminalu użytkownika wykonano syntetyczny benchmark: 1 000 rekordów/iteracji, mean 70 226 ns, p50 68 337 ns, p95 75 497 ns, p99 128 796 ns. To nie jest wynik produkcyjny ani end-to-end.
- GitHub Actions dla commita `7b1fb71137a42795b8dcd184a3acb7d5067c6435` zakończył się **failure** na kroku testów; benchmark został pominięty. Przyczyna nie jest jeszcze potwierdzona z logów.
- Workflow zmieniono: `cargo test --locked` zastąpiono `cargo test`, aby nie wymagać istniejącego lockfile na etapie prototypu. Nowy przebieg CI trzeba sprawdzić.
- Moss, Zenoh, trwały event log i integracja end-to-end pozostają niezaimplementowane.

## Wykonano — ostatnie zadania

### 1. Ulepszono benchmark

`benches/retrieval.rs` ma rozgrzewkę i zmienne: `BENCH_RECORDS`, `BENCH_WARMUP`, `BENCH_ITERATIONS`, `BENCH_TOP_K`, `BENCH_QUERY`. Raportuje średnią oraz p50/p95/p99 i ograniczenia syntetycznego microbenchmarku. Sprzęt i wersję Rust należy dopisać przy uruchomieniu.

### 2. Dodano testy regresji jakości retrieval

`tests/retrieval_quality.rs` sprawdza oczekiwany top-2, odrzucanie dokumentu niepowiązanego i limit top-k. Testy zapisane; wynik wymaga CI.

### 3. Zapisano kontrakt replay/audytu

`docs/REPLAY_AND_AUDIT.md` opisuje kopertę zdarzenia, sekwencjonowanie, kontrolę integralności i replay bez skutków ubocznych.

### 4. Zapisano kontrakt polityki narzędzi

`docs/TOOL_POLICY_CONTRACT.md` opisuje walidację, allowlistę, zatwierdzanie, idempotency i fail-closed.

### 5. Zapisano bramkę integracji Moss

`docs/MOSS_ADAPTER_GATE.md` określa interfejs i checklistę weryfikacji oficjalnego API. Nie wpisano niezweryfikowanych endpointów ani sygnatur.

## Wykonano teraz — implementacja kodu

### 6. Dodano wykonywalny policy gate

`src/policy.rs` implementuje dokładne dopasowanie allowlisty, tryb `DryRun` oraz wymóg jawnego zatwierdzenia dla akcji oznaczonych jako wymagające zgody. Dodano testy jednostkowe. To lokalna funkcja decyzyjna; nie jest jeszcze podłączona do żadnego wykonawcy narzędzi.

### 7. Dodano in-memory replay ledger

`src/replay.rs` zapisuje uporządkowane zdarzenia, odrzuca puste identyfikatory, duplikaty `event_id` i konflikt klucza idempotencji. `replay()` zwraca kopię historii i nie wywołuje narzędzi. Dane są wyłącznie w pamięci — brak trwałości i kryptograficznej integralności.

### 8. Wyeksportowano moduły z biblioteki

`src/lib.rs` udostępnia `policy` i `replay` obok selektora kontekstu.

### 9. Zmieniono komendę testów CI

`.github/workflows/rust-ci.yml` uruchamia teraz `cargo test`, a następnie benchmark. Zmiana została zapisana, ale nowy run nie został jeszcze zweryfikowany.

## Linki

- [Rust library](https://github.com/mojealterego/The-Zero-Latency-Builder-Sprint/blob/main/src/lib.rs)
- [Policy gate](https://github.com/mojealterego/The-Zero-Latency-Builder-Sprint/blob/main/src/policy.rs)
- [Replay ledger](https://github.com/mojealterego/The-Zero-Latency-Builder-Sprint/blob/main/src/replay.rs)
- [CI workflow](https://github.com/mojealterego/The-Zero-Latency-Builder-Sprint/blob/main/.github/workflows/rust-ci.yml)
- [GitHub Actions](https://github.com/mojealterego/The-Zero-Latency-Builder-Sprint/actions)

## Następne działania

1. Sprawdzić nowy przebieg Actions i odczytać konkretny błąd, jeśli nadal zawiedzie.
2. Naprawić kompilację/testy, dopiero potem uznać implementację za zweryfikowaną.
3. Dodać trwałość do event logu i testy restart/replay, bez ponawiania efektów zewnętrznych.
4. Podłączyć policy gate do rzeczywistego interfejsu wykonania, z zatwierdzeniem powiązanym z dokładnym żądaniem.
5. Zweryfikować oficjalne API Moss i zbudować adapter, a następnie porównać jakość retrieval oraz latency end-to-end.
