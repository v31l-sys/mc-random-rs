# mc-random-rs
Java and Chunk Random implementations

In Cargo.toml:

[dependencies]<br>
random = { git = "https://github.com/v31l-sys/mc-random-rs" }

================================================================

In main.rs:<br><br>
use random::chunk_random::*;

use random::chunk_random::*; (also implemented for direct java_random usage)

```rust
let mut cr = ChunkRandom::default();
cr.get_random(32141);
cr.rand.next_int_fast(16); //rand is an instance of JavaRandom
```
