Generate snowflake images

http://flake.randomnumberpicker.co.uk

Snowflake lib from following article code on https://joshleeb.com/posts/rust-wasm-snowhash/
Currently using local modified version of the snowflake lib.

On prod
- cargo build --release
- sudo supervisorctl restart flake