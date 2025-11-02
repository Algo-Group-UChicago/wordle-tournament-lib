# Wordle Tournament Library (Rust)

A high-performance Rust library for Wordle tournament functionality, exposed to Python via PyO3.

## Prerequisites

1. **Rust**: Install from [rustup.rs](https://rustup.rs/)
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```

2. **Python**: Python 3.7+ is required

3. **maturin**: Build tool for Rust-Python integration
   ```bash
   pip install maturin
   ```

## Development Setup

### Building and Installing Locally

1. **Develop mode** (recommended for development - installs in editable mode):
   ```bash
   cd wordle-tournament-lib
   maturin develop
   ```
   This will:
   - Compile the Rust code
   - Create a Python extension module
   - Install it in your current Python environment

2. **Release mode** (optimized, but slower to compile):
   ```bash
   maturin develop --release
   ```

### Testing the Library

After installing with `maturin develop`, you can test it in Python:

```python
from wordle_tournament_lib import WordleHint

# Create a WordleHint instance with simple strings
# 'W' = correct position (green)
# '*' = present but wrong position (yellow)
# 'L' = absent (gray)
hint = WordleHint("hello", ["W", "*", "L", "L", "*"])

# Access properties
print(hint.word)  # "hello"
print(hint.hints)  # ['W', '*', 'L', 'L', '*']
print(hint.word_hint_pairs)  # [('h', 'W'), ('e', '*'), ('l', 'L'), ('l', 'L'), ('o', '*')]

# Visualize (prints to stdout, like the original Python version)
hint.visualize_hint()
# Output:
# H E L L O
# 🟩 🟨 ⬜ ⬜ 🟨

# String representation
print(repr(hint))  # WordleHint(word='hello', hints=['W', '*', 'L', 'L', '*'])
```

The library uses a simple string-based API for Python users:
- **'W'** - Correct letter in correct position (🟩 green)
- **'*'** - Letter is present but in wrong position (🟨 yellow)
- **'L'** - Letter is absent from the word (⬜ gray)

## Building Distribution Packages

### Building a Wheel

```bash
maturin build
```

This creates a wheel file in `target/wheels/` that can be installed with pip.

### Building for Specific Python Versions

```bash
# Build for Python 3.9
maturin build --interpreter python3.9
```

### Publishing to PyPI

1. Build the wheel:
   ```bash
   maturin build --release
   ```

2. Upload to PyPI:
   ```bash
   maturin publish
   ```

   Or use twine:
   ```bash
   pip install twine
   twine upload target/wheels/*
   ```

## Project Structure

```
wordle-tournament-lib/
├── Cargo.toml                    # Rust package configuration
├── pyproject.toml                # Python package configuration
├── wordle_tournament_lib.pyi     # Type stubs for IDE support
├── src/
│   ├── lib.rs                   # Main library entry point (PyO3 module)
│   └── hint.rs                  # Hint types and WordleHint implementation
└── README.md
```

## Adding New Modules

When adding new Rust modules to the library:

1. **Create the Rust module file** (e.g., `src/new_module.rs`)

2. **Add the module to `src/lib.rs`**:
   ```rust
   mod new_module;
   ```

3. **Expose Python classes/functions in the `#[pymodule]` function**:
   ```rust
   #[pymodule]
   fn wordle_tournament_lib(_py: Python, m: &PyModule) -> PyResult<()> {
       m.add_class::<hint::WordleHint>()?;
       m.add_class::<new_module::NewClass>()?;  // Add your new class
       Ok(())
   }
   ```

4. **Rebuild**:
   ```bash
   maturin develop
   ```

## How It Works

### Internal vs External Types

- **In Rust**: `HintType` is a type-safe enum used internally for performance and correctness
- **In Python**: Users work with simple strings ('W', '*', 'L') for ease of use
- **Conversion**: PyO3 automatically converts between Rust types and Python types via the `#[getter]` methods

This gives you the best of both worlds:
- Rust code gets type safety and performance
- Python users get a simple, intuitive API

## Common Issues

### "No module named 'wordle_tournament_lib'"

- Make sure you've run `maturin develop` or installed the package
- Check that you're using the correct Python environment

### Compilation Errors

- Ensure you have the latest Rust toolchain: `rustup update`
- Check that PyO3 version matches your Python version

### Import Errors After Rebuilding

- Restart your Python interpreter/kernel
- If using Jupyter, restart the kernel

## Performance

The Rust implementation provides:
- Fast execution speed
- Memory safety
- Zero-cost abstractions

For maximum performance, use `maturin develop --release` during development.

