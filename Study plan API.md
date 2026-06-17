# Study Plan: API Programming (Rust Concurrency)

This plan is optimized for the **Programmazione di Sistema (API Programming)** exam held by Prof. Giovanni Malnati at Politecnico di Torino.

## Core Objectives
1. **Correctness**: Absolute thread safety and race-condition avoidance.
2. **Efficiency**: No busy-waiting (always use `Condvar`).
3. **Robustness**: Handling "shutdown" states and avoiding `panic!`.
4. **Time Management**: Mastering `wait_timeout` for delayed tasks.

---

## Phase 1: Foundations (Week 1)
*   **Goal**: Master Rust ownership, borrowing, and basic syntax.
*   **Resources**: `Esercizi/rustlings`, `Appunti/The Rust Programming Language.pdf`.
*   **Tasks**:
    *   Complete all **Rustlings** exercises, focusing on `threads`, `move` semantics, and `error_handling`.
    *   Read Chapters 4 (Ownership), 15 (Smart Pointers), and 16 (Concurrency) of the Rust Book.
    *   Understand the `Send` and `Sync` traits.

## Phase 2: Synchronization Primitives (Week 2)
*   **Goal**: Master the "Mutex + Condvar" pattern.
*   **Resources**: `Esercizi/condvar`, `pds-rust-exams/src/mpmc_channel/mutex.rs`.
*   **Tasks**:
    *   Implement a **Bounded Buffer** from scratch using `cv.wait_while()`.
    *   Practice sharing state using `Arc<Mutex<T>>`.
    *   Handle lock poisoning and return `Option<T>`/`Result<T, E>`.

## Phase 3: Core Exam Patterns (Week 3)
*   **Goal**: Solve classic synchronization problems.
*   **Resources**: `pds-rust-exams/original` (Older PDFs), `pds-rust-exams/src/`.
*   **Exams to master**:
    1.  **Exchanger** (Feb 2021): Two-way synchronization.
    2.  **Circular Buffer** (Sept 2021): FIFO logic.
    3.  **Ranking Barrier** (Jan 2023): Group synchronization and ordered results.

## Phase 4: Advanced Concurrency (Week 4)
*   **Goal**: Handle complex states, timeouts, and shutdowns.
*   **Resources**: `pds-rust-exams/original` (2023-2025 tracks).
*   **Exams to master**:
    1.  **MpMcChannel** (June 2023): Shutdown logic and pending data flush.
    2.  **DelayedQueue** (July 2023): `wait_timeout` and `Instant`.
    3.  **Token Manager** (June 2025): Resource allocation and priority.

## Phase 5: Refinement & Simulation (Final Days)
*   **Goal**: Speed and "Panic-Free" code.
*   **Tasks**:
    *   **Panic-Safety**: Eliminate `unwrap()` on locks.
    *   **Generics**: Validate `T: Send` constraints.
    *   **Mock Exam**: Solve a recent track (e.g., *2025-01-13 delayed-executor.pdf*) in 90 minutes.

---

## Technical Cheat Sheet
When implementing exam structures, follow this robust pattern:
```rust
pub fn send(&self, e: E) -> Option<()> {
    let mut lock = self.c_buffer.lock().ok()?; // Handle poisoning
    
    // Wait for space while the channel is open
    lock = self.cv.wait_while(lock, |(state, buf)| {
        buf.len() == self.buffer_size && *state == Open
    }).ok()?;

    if *lock.0 == Closed { return None; }
    
    lock.1.push(e);
    self.cv.notify_all(); 
    Some(())
}
```
