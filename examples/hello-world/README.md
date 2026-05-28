# Hello World VTRACE Example

This example demonstrates a complete VTRACE adoption on a deliberately tiny
program.

The program prints `Hello, VTRACE!` and exits successfully.

## Validation

```powershell
python src\hello_world.py
python -m py_compile src\hello_world.py
```

Expected output:

```text
Hello, VTRACE!
```
