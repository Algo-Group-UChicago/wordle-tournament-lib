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

#### Basic Usage - WordleHint

```python
from wordle_tournament_lib import WordleHint

# Create a WordleHint instance with a hint string
# 'O' = correct position (green)
# '~' = present but wrong position (yellow)
# 'X' = absent (gray)
hint = WordleHint("hello", "O~XX~")

# Access properties
print(hint.word)  # "hello"
print(hint.hints)  # "O~XX~" (returns string)
print(hint.word_hint_pairs)  # [('h', 'O'), ('e', '~'), ('l', 'X'), ('l', 'X'), ('o', '~')]

# Visualize (prints to stdout)
hint.visualize_hint()
# Output:
# H E L L O
# 🟩 🟨 ⬜ ⬜ 🟨

# String representation
print(repr(hint))  # WordleHint(word='hello', hints=['O', '~', 'X', 'X', '~'])
```

#### Creating a Bot

```python
from wordle_tournament_lib import UChicagoWordleBotBase, WordleHint

class MyBot(UChicagoWordleBotBase):
    def __init__(self, team_id: str):
        # Load your word lists, initialize state, etc.
        pass
    
    def guess(self, hints: list[WordleHint]) -> str:
        """Implement your guessing strategy here."""
        if not hints:
            return "crane"  # First guess
        # Analyze hints and return next guess
        return "stare"

# Create and test your bot
bot = MyBot("my-team-id")

# Test on a single word
guesses = bot.evaluate_on_word("crane", logging=True)
print(f"Solved in {guesses} guesses")

# Run full tournament evaluation (1000 words)
score = bot.evaluate(grade_local=True)
print(f"Average guesses: {score:.2}")
```

#### Hint Characters

The library uses a simple string-based API:
- **'O'** - Correct letter in correct position (🟩 green)
- **'~'** - Letter is present but in wrong position (🟨 yellow)
- **'X'** - Letter is absent from the word (⬜ gray)

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
├── corpus.txt                    # Valid guess words (embedded in binary)
├── possible_answers.txt          # Answer key words (embedded in binary)
├── src/
│   ├── lib.rs                   # Main library entry point (PyO3 module)
│   ├── hint.rs                  # Hint types and WordleHint implementation
│   ├── grade.rs                 # Wordle grading algorithm
│   ├── corpus.rs                # Word corpus management
│   └── wordle_bot_base.rs       # UChicagoWordleBotBase class
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
   fn wordle_tournament_lib(m: &Bound<'_, PyModule>) -> PyResult<()> {
       m.add_class::<hint::WordleHint>()?;
       m.add_class::<wordle_bot_base::UChicagoWordleBotBase>()?;
       m.add_class::<new_module::NewClass>()?;  // Add your new class
       Ok(())
   }
   ```

4. **Rebuild**:
   ```bash
   maturin develop
   ```

## Key Features

### Tournament Evaluation

- **`evaluate(grade_local: bool)`**: Runs full tournament (1000 words, 20 max guesses each)
  - Returns average number of guesses per word (float)
  - `grade_local=True`: Grades locally without server (for testing)
  - `grade_local=False`: Submits to tournament server

- **`evaluate_on_word(answer: str, logging: bool = True)`**: Test on a single word
  - Returns number of guesses needed (int)
  - `logging=True`: Shows visual progress with emoji squares
  - `logging=False`: Silent mode for batch testing

### Word Validation

- All guesses are validated against the embedded corpus
- Invalid words raise `ValueError` immediately
- Corpus is embedded in binary (no file I/O at runtime)

### How It Works

#### Internal vs External Types

- **In Rust**: `HintType` is a type-safe enum used internally for performance
- **In Python**: Users work with simple strings ('O', '~', 'X') for ease of use
- **Conversion**: PyO3 automatically converts between Rust types and Python types

This gives you the best of both worlds:
- Rust code gets type safety and performance
- Python users get a simple, intuitive API

#### Grading Algorithm

The library implements the standard Wordle grading algorithm:
1. First pass: Mark correct positions (green)
2. Second pass: Mark present letters (yellow), ensuring duplicates are handled correctly
3. Remaining letters: Mark as absent (gray)

## API Reference

### UChicagoWordleBotBase

Base class for creating Wordle bots. Subclass this and implement `guess()`.

**Methods:**
- `evaluate(grade_local: bool) -> float`: Run full tournament evaluation
- `evaluate_on_word(answer: str, logging: bool = True) -> int`: Test on single word
- `guess(hints: list[WordleHint]) -> str`: **Abstract** - implement in subclass

### WordleHint

Represents a guess and its hint pattern.

**Properties:**
- `word: str`: The guessed word
- `hints: str`: Hint pattern string ('O~XX~' etc.)
- `word_hint_pairs: list[tuple[str, str]]`: Pairs of (letter, hint)

**Methods:**
- `visualize_hint()`: Print visual representation with emoji squares

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

### "Guess is not a valid word"

- Your `guess()` method returned a word not in the corpus
- Make sure to only return valid 5-letter words from the corpus

## Performance

The Rust implementation provides:
- Fast execution speed
- Memory safety
- Zero-cost abstractions

For maximum performance, use `maturin develop --release` during development.

