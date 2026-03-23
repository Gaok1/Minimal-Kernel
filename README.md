# Minimal Kernel

A minimal x86_64 kernel written in Rust, built following [Philipp Oppermann's "Writing an OS in Rust"](https://os.phil-opp.com/) blog series.

## What's implemented

- **VGA Text Mode** — driver for writing to the VGA buffer (0xb8000), with color support and `print!`/`println!`/`eprintln!` macros
- **GDT (Global Descriptor Table)** — kernel code segment and TSS configured via the `x86_64` crate
- **TSS (Task State Segment)** — dedicated stack for double faults via IST (Interrupt Stack Table)
- **IDT (Interrupt Descriptor Table)** — interrupt table with the following handlers:
  - `Breakpoint` — prints the stack frame
  - `Double Fault` — uses a separate IST stack to prevent triple faults
  - `Stack Segment Fault` — prints the stack frame
  - `Page Fault` — prints the accessed address and error code
  - `Timer (PIC IRQ0)` — prints `.` on every tick
  - `Keyboard (PIC IRQ1)` — reads scancodes and prints characters via `pc-keyboard`
- **Chained PIC 8259** — IRQ remapping to avoid conflicts with CPU exceptions
- **HLT loop** — kernel halts after initialization to avoid burning CPU cycles

## Structure

```
src/
├── main.rs                        # Entry point (_start), init, panic handler
├── gdt.rs                         # GDT + TSS
├── vga/
│   ├── mod.rs
│   ├── buffer.rs                  # VGA buffer mapping
│   ├── color.rs                   # Color enum
│   └── writer.rs                  # Writer + print!/println! macros
└── interrupt_handler/
    ├── mod.rs
    └── interrupts.rs              # IDT, PIC, exception and IRQ handlers

target_set/
└── x86-bare_metal.json            # Custom target: x86_64, no OS, no red zone, no SSE
```

## Dependencies

| Crate | Purpose |
|---|---|
| `bootloader 0.9` | Loads the kernel and sets up the initial environment |
| `x86_64` | Abstractions for GDT, IDT, TSS, paging, and registers |
| `pic8259` | Driver for the chained PIC 8259 |
| `pc-keyboard` | PS/2 keyboard scancode decoding |
| `spin` | OS-free spinlock mutex |
| `lazy_static` | Lazy initialization of statics with `spin_no_std` |

## Building and running

### Prerequisites

```bash
# Rust nightly
rustup override set nightly

# Required components
rustup component add rust-src llvm-tools-preview

# Tool to create a bootable image
cargo install bootimage
```

### Build

```bash
cargo build
```

### Run with QEMU

```bash
cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86-bare_metal/debug/bootimage-minimal-kernel.bin
```

## Custom target

The `target_set/x86-bare_metal.json` file defines a bare-metal x86_64 target with:

- `os: none` — no operating system
- `disable-redzone: true` — required for interrupt handlers
- `-mmx,-sse,+soft-float` — SSE disabled (the kernel does not save XMM registers)
- Linker: `rust-lld` (LLD)
- Panic strategy: `abort`

## Reference

- [Writing an OS in Rust — Philipp Oppermann](https://os.phil-opp.com/)
