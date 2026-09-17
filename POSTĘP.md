# POSTĘP — The Zero Latency Builder Sprint

**Data aktualizacji:** 2026-09-17  
**Repozytorium:** https://github.com/mojealterego/The-Zero-Latency-Builder-Sprint  
**Zasada raportowania:** rozróżniamy stan potwierdzony, propozycje i zadania niewykonane. Brak pomiaru/testu nie jest wynikiem pozytywnym.

## Stan bazowy

- Rust crate, CLI i leksykalny selektor są zapisane w repozytorium.
- W terminalu użytkownika wykonano syntetyczny benchmark: 1 000 rekordów/iteracji, mean 70 226 ns, p50 68 337 ns, p95 75 497 ns, p99 128 796 ns. To nie jest wynik produkcyjny ani end-to-end.
- Wcześniejszy GitHub Actions run zakończył się failure na kroku testów; przyczyna nie została potwierdzona z logów.
- Użytkownik uruchomił `cargo bench --bench retrieval`; na zrzucie widać zakończenie benchmarku i powyższe wyniki.
- Moss, Zenoh, trwały event log i integracja end-to-end pozostają niezaimplementowane.

## Wykonane etapy

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

### 6. Dodano wykonywalny policy gate

`src/policy.rs` implementuje dokładne dopasowanie allowlisty, tryb `DryRun` oraz wymóg jawnego zatwierdzenia dla akcji oznaczonych jako wymagające zgody. Dodano testy jednostkowe. To lokalna funkcja decyzyjna.

### 7. Dodano in-memory replay ledger

`src/replay.rs` zapisuje uporządkowane zdarzenia, odrzuca puste identyfikatory, duplikaty `event_id` i konflikt klucza idempotencji. `replay()` zwraca kopię historii i nie wywołuje narzędzi. Dane są wyłącznie w pamięci — brak trwałości i kryptograficznej integralności.

### 8. Dodano CLI demo policy + audit

`src/main.rs` wyświetla wyniki lokalnego retrieval, demonstruje decyzję `DryRun` dla jawnie dozwolonej akcji i zapisuje przykładowe zdarzenie do pamięciowego ledgeru. Demo nie wywołuje narzędzi zewnętrznych.

### 9. Dodano GitHub Actions dla Rust

`.github/workflows/rust.yml` wykonuje `cargo fmt --all -- --check`, `cargo test --all-targets` i `cargo clippy --all-targets -- -D warnings` dla push/PR do `main`. Workflow został zapisany; jego wynik nie został jeszcze potwierdzony.

## Ważne: co oznacza benchmark

Widoczny wynik pochodzi z syntetycznego, jednowątkowego benchmarku leksykalnego. Nie mierzy Moss, Zenoh, bazy trwałej, konkurencji, jakości semantycznej ani opóźnienia całej ścieżki aplikacji. Nie należy przedstawiać go jako dowodu „sub-10ms” całego produktu.

## Linki

- [Repozytorium](https://github.com/mojealterego/The-Zero-Latency-Builder-Sprint)
- [Rust library](https://github.com/mojealterego/The-Zero-Latency-Builder-Sprint/blob/main/src/lib.rs)
- [CLI demo](https://github.com/mojealterego/The-Zero-Latency-Builder-Sprint/blob/main/src/main.rs)
- [Policy gate](https://github.com/mojealterego/The-Zero-Latency-Builder-Sprint/blob/main/src/policy.rs)
- [Replay ledger](https://github.com/mojealterego/The-Zero-Latency-Builder-Sprint/blob/main/src/replay.rs)
- [Benchmark](https://github.com/mojealterego/The-Zero-Latency-Builder-Sprint/blob/main/benches/retrieval.rs)
- [Rust CI workflow](https://github.com/mojealterego/The-Zero-Latency-Builder-Sprint/blob/main/.github/workflows/rust.yml)
- [GitHub Actions](https://github.com/mojealterego/The-Zero-Latency-Builder-Sprint/actions)

## Następne działania

1. Sprawdzić nowy przebieg Actions i naprawić ewentualne błędy formatowania/testów/Clippy.
2. Dodać trwałość event logu i testy restart/replay, bez ponawiania efektów zewnętrznych.
3. Powiązać zatwierdzenie z kanonicznym hashem dokładnego żądania (akcja + argumenty + zakres), nie tylko z flagą boolean.
4. Zweryfikować oficjalne API Moss i zbudować adapter dopiero po potwierdzeniu pakietu, wersji, uwierzytelniania i kontraktów.
5. Zmierzyć jakość retrieval (Recall@k/MRR) i latency end-to-end na opisanym sprzęcie, przy jawnych warunkach warm/cold i współbieżności.
