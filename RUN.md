On host:

```bash
docker build -t tectonic_svg .
docker run -it tectonic_svg
```

On `tectonic_svg`:

```bash
cargo run -- test.tex
```
