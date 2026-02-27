## 2025-02-26 - Optimized File Reading Loop
**Learning:** `BufRead::lines()` allocates a new `String` for every line, causing massive allocation overhead in tight loops. Using `read_line(&mut buffer)` reuses the buffer and eliminates this overhead.
**Action:** Always use `read_line` with a reusable buffer for performance-critical file reading loops instead of `lines()`.
