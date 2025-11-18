# Quadratic Solver GUI

A modern, user-friendly graphical application for solving quadratic equations with support for both real and complex roots. Built with Rust and egui.

![Rust](https://img.shields.io/badge/rust-%23000000.svg?style=for-the-badge&logo=rust&logoColor=white)
![License](https://img.shields.io/badge/license-MIT-blue.svg)

## 📋 Overview

This application solves quadratic equations of the form **ax² + bx + c = 0** and provides detailed information about the discriminant and the nature of the roots. It handles all cases including real roots, complex roots, and degenerate equations.

## ✨ Features

### Core Functionality

- **Complete Quadratic Solver**: Solves equations of the form ax² + bx + c = 0
- **High-Precision Results**: 12 decimal digit accuracy for D, x1, and x2
- **Complex Number Support**: Handles both real and complex roots seamlessly
- **Discriminant Display**: Shows D = b² - 4ac with color coding
- **Root Classification**: Automatically identifies:
  - Two Real Distinct Roots (D > 0)
  - One Real Repeated Root (D ≈ 0)
  - Two Complex Conjugate Roots (D < 0)

### User Interface

- **Large, Readable Fonts**: Easy-to-read 18-20pt fonts for inputs and outputs
- **Color-Coded Results**:
  - 🟢 Green discriminant for real roots (D ≥ 0)
  - 🟠 Orange discriminant for complex roots (D < 0)
  - 🔵 Cyan blue for root type indicator
  - 🔴 Red for error messages
- **Responsive Layout**: Clean, centered design with proper spacing
- **Theme Support**: Toggle between dark 🌙 and light ☀ modes

### Enhanced Usability

- **Keyboard Support**: Press Enter in any input field to compute
- **Clear Button**: Reset all fields with one click
- **Quick Examples**: One-click preset equations demonstrating various precision levels:
  - x² - 5x + 6 = 0 (Simple integer roots)
  - x² + 1 = 0 (Pure complex roots)
  - x² - 2 = 0 (Irrational roots: ±√2)
  - x² - 3x + 1 = 0 (Golden ratio related)
  - 2x² - 7x + 3 = 0 (Fractional roots)
  - x² - √2x + 1 = 0 (Irrational coefficients)
- **Input Validation**: Clear error messages for invalid inputs
- **Smart Formatting**: Displays complex numbers in readable format (a + bi) with 12-digit precision

## 🚀 Getting Started

### Prerequisites

- Rust 1.70 or higher
- Cargo (comes with Rust)

### Installation

1. Clone the repository:

```bash
git clone https://github.com/yourusername/quad_complex_gui.git
cd quad_complex_gui
```

2. Build and run:

```bash
cargo run --release
```

### Quick Start

1. Enter coefficients for a, b, and c
2. Click "Compute" or press Enter
3. View the discriminant (D), root type, and solutions (x1, x2)

## 🎮 Usage Examples

### Example 1: Real Distinct Roots

**Equation**: x² - 5x + 6 = 0

- **Coefficients**: a=1, b=-5, c=6
- **Discriminant**: D = 1.000000000000
- **Root Type**: Two Real Distinct Roots
- **Solutions**: x1 = 3.000000000000, x2 = 2.000000000000

### Example 2: Complex Roots

**Equation**: x² + 1 = 0

- **Coefficients**: a=1, b=0, c=1
- **Discriminant**: D = -4.000000000000
- **Root Type**: Two Complex Conjugate Roots
- **Solutions**: x1 = 0.000000000000 +1.000000000000i, x2 = 0.000000000000 -1.000000000000i

### Example 3: Repeated Root

**Equation**: x² - 2x + 1 = 0

- **Coefficients**: a=1, b=-2, c=1
- **Discriminant**: D = 0.000000000000
- **Root Type**: One Real Repeated Root
- **Solutions**: x1 = 1.000000000000, x2 = 1.000000000000

### Example 4: Irrational Roots (High Precision)

**Equation**: x² - 2 = 0

- **Coefficients**: a=1, b=0, c=-2
- **Discriminant**: D = 8.000000000000
- **Root Type**: Two Real Distinct Roots
- **Solutions**: x1 = 1.414213562373, x2 = -1.414213562373
- **Note**: Demonstrates precision for √2 ≈ 1.414213562373095...

### Example 5: Golden Ratio Related (High Precision)

**Equation**: x² - 3x + 1 = 0

- **Coefficients**: a=1, b=-3, c=1
- **Discriminant**: D = 5.000000000000
- **Root Type**: Two Real Distinct Roots
- **Solutions**: x1 = 2.618033988750, x2 = 0.381966011250
- **Note**: x1 ≈ φ + 1 (golden ratio plus one), x2 ≈ 1/φ

### Example 6: Irrational Coefficient (Highest Precision Need)

**Equation**: x² - √2x + 1 = 0

- **Coefficients**: a=1, b=-1.414213562373, c=1
- **Discriminant**: D = 2.000000000000
- **Root Type**: Two Real Distinct Roots
- **Solutions**: x1 = 1.707106781187, x2 = 0.707106781187
- **Note**: Uses irrational coefficient √2, showcasing the full 12-digit precision

## 🏗️ Project Structure

```
quad_complex_gui/
├── src/
│   ├── main.rs          # GUI application and main logic
│   └── quadratic.rs     # Core quadratic solver algorithm
├── Cargo.toml           # Project dependencies
└── README.md            # This file
```

### Architecture: Why Two Separate Files?

The project follows a **modular architecture** pattern, separating concerns into distinct modules:

#### **`src/quadratic.rs`** - Pure Mathematical Logic

- Contains the core `quadratic_roots()` function implementing the quadratic formula
- Handles all complex number arithmetic using `num-complex`
- **Completely independent** of GUI code - no UI dependencies
- Includes comprehensive unit tests for the solver algorithm
- **Reusable**: Can be imported into CLI tools, web services, or other Rust libraries

#### **`src/main.rs`** - User Interface Layer

- Implements the GUI window using `eframe`/`egui`
- Manages user input, buttons, themes, and visual elements
- Handles result formatting and display logic
- Provides validation, error messages, and user feedback
- **Imports and calls** the `quadratic_roots()` function from `quadratic.rs`

#### Benefits of This Separation:

1. **Separation of Concerns**: Math logic is isolated from UI code - each module has one clear responsibility

2. **Testability**: The solver can be thoroughly unit-tested independently without launching a GUI

3. **Reusability**: The `quadratic.rs` module could be reused in:

   - Command-line interface (CLI) applications
   - Web application backends
   - Other Rust libraries or crates
   - Different GUI frameworks (GTK, Qt, etc.)

4. **Maintainability**:

   - UI changes (colors, layout, themes) don't affect the math module
   - Algorithm improvements don't require touching GUI code
   - Easier to locate and fix bugs

5. **Team Development**: Multiple developers can work on UI and core logic simultaneously without merge conflicts

This design pattern is fundamental to software engineering and enables clean, maintainable, and scalable code architecture.

## 🧪 Testing

Run the comprehensive test suite:

```bash
cargo test
```

The test suite includes:

- Real distinct roots verification
- Complex conjugate roots handling
- Linear equation edge cases
- Degenerate equation detection

## 📦 Dependencies

- **eframe** (v0.27): Modern GUI framework based on egui
- **num-complex** (v0.4): Complex number arithmetic

## 🔧 Technical Details

### Algorithm

The solver uses the quadratic formula with complex arithmetic:

- x = (-b ± √(b² - 4ac)) / 2a
- Automatic handling of negative discriminants through `Complex<f64>`
- Epsilon-based comparison for floating-point precision (1e-12)
- **12-decimal digit display precision** for all results (D, x1, x2)

### High-Precision Display

**12 Decimal Digit Accuracy**: All results (discriminant and roots) are displayed with 12 decimal places to ensure:

- Accurate representation of irrational numbers (√2, √3, π-related values, etc.)
- Precise golden ratio and Fibonacci-related calculations
- Minimal floating-point rounding errors
- Scientific and engineering-grade precision

**Why 12 Digits?**

- Standard `f64` (double precision) provides ~15-17 significant digits
- 12 decimal places balances precision with readability
- Crucial for equations involving:
  - Square roots of non-perfect squares (√2, √3, √5)
  - Golden ratio φ = (1 + √5)/2 ≈ 1.618033988750
  - Irrational coefficients and transcendental numbers
  - Scientific calculations requiring high accuracy

### Complex Number Formatting

- Pure real: `2.000000000000`
- Pure imaginary: `1.000000000000i`
- Complex: `3.000000000000 +2.000000000000i`

### Edge Cases Handled

- ✅ Degenerate equations (a=0, b=0)
- ✅ Linear equations (a=0, b≠0)
- ✅ Negative discriminants (complex roots)
- ✅ Zero discriminant (repeated roots)
- ✅ Invalid input parsing

## 🎨 UI Features

### Theme Switching

The application supports both dark and light themes, switchable via the theme toggle button at the bottom of the window.

### Keyboard Shortcuts

- **Enter**: Compute results from any input field
- **Tab**: Navigate between input fields

### Visual Indicators

- Color-coded discriminant values
- Root type classification display
- Error messages with warning symbols
- Example buttons for quick testing

## 🛠️ Building for Release

For optimized performance:

```bash
cargo build --release
```

The executable will be in `target/release/quad_complex_gui.exe` (Windows) or `target/release/quad_complex_gui` (Linux/macOS).

## 📝 License

This project is licensed under the MIT License - see the LICENSE file for details.

## 🤝 Contributing

Contributions are welcome! Please feel free to submit a Pull Request.

### Ideas for Future Enhancements

- Copy results to clipboard
- Graph visualization of the parabola
- Support for higher-degree polynomials
- Save/load equation history
- Export results to file
- Multi-language support

## 👨‍💻 Author

Created with ❤️ using Rust and egui

## 🙏 Acknowledgments

- Built with [egui](https://github.com/emilk/egui) - an immediate mode GUI library
- Complex number support via [num-complex](https://github.com/rust-num/num-complex)
- Rust community for excellent tooling and documentation

---

**Made with Rust 🦀**
