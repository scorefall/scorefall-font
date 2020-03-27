# Scorefall Font
Convert a [SMuFL](https://github.com/w3c/smufl) SVG font into a ScoreFall Font.
ScoreFall fonts are efficiently stored so that they can transferred over the
network without causing delays.

## Generating The "Modern" ScoreFall Font
```bash
cargo run --release --example from_smufl modern.sfff font-main/main.svg font-main/meta.json
```
