# Minimal Kernel

Um kernel x86_64 mínimo escrito em Rust, desenvolvido seguindo o post do [Oppermann (Writing an OS in Rust)](https://os.phil-opp.com/).

## O que está implementado

- **VGA Text Mode** — driver para escrita no buffer VGA (0xb8000), com suporte a cores e macros `print!`/`println!`/`eprintln!`
- **GDT (Global Descriptor Table)** — segmentos de código do kernel e TSS configurados via `x86_64`
- **TSS (Task State Segment)** — pilha dedicada para double faults via IST (Interrupt Stack Table)
- **IDT (Interrupt Descriptor Table)** — tabela de interrupções com os seguintes handlers:
  - `Breakpoint` — imprime o stack frame
  - `Double Fault` — pilha separada via IST para evitar triple fault
  - `Stack Segment Fault` — imprime o stack frame
  - `Page Fault` — imprime endereço acessado e error code
  - `Timer (PIC IRQ0)` — imprime `.` a cada tick
  - `Keyboard (PIC IRQ1)` — lê scancodes e imprime caracteres via `pc-keyboard`
- **PIC 8259 encadeado** — remapeamento das IRQs para evitar conflito com as exceções da CPU
- **HLT loop** — o kernel entra em halt após a inicialização para não desperdiçar CPU

## Estrutura

```
src/
├── main.rs                        # Entry point (_start), init, panic handler
├── gdt.rs                         # GDT + TSS
├── vga/
│   ├── mod.rs
│   ├── buffer.rs                  # Mapeamento do buffer VGA
│   ├── color.rs                   # Enum de cores
│   └── writer.rs                  # Writer + macros print!/println!
└── interrupt_handler/
    ├── mod.rs
    └── interrupts.rs              # IDT, PIC, handlers de exceções e IRQs

target_set/
└── x86-bare_metal.json            # Target customizado: x86_64, sem OS, sem red zone, sem SSE
```

## Dependências

| Crate | Uso |
|---|---|
| `bootloader 0.9` | Carrega o kernel e configura o ambiente inicial |
| `x86_64` | Abstrações para GDT, IDT, TSS, paginação, registradores |
| `pic8259` | Driver para o PIC 8259 encadeado |
| `pc-keyboard` | Decodificação de scancodes de teclado PS/2 |
| `spin` | Mutex sem OS (spinlock) |
| `lazy_static` | Inicialização lazy de statics com `spin_no_std` |

## Como compilar e rodar

### Pré-requisitos

```bash
# Rust nightly
rustup override set nightly

# Componentes necessários
rustup component add rust-src llvm-tools-preview

# Ferramenta para criar imagem bootável
cargo install bootimage
```

### Build

```bash
cargo build
```

### Rodar com QEMU

```bash
cargo bootimage
qemu-system-x86_64 -drive format=raw,file=target/x86-bare_metal/debug/bootimage-minimal-kernel.bin
```

## Target customizado

O arquivo `target_set/x86-bare_metal.json` define um target `x86_64` bare-metal com:

- `os: none` — sem sistema operacional
- `disable-redzone: true` — necessário para handlers de interrupção
- `-mmx,-sse,+soft-float` — SSE desabilitado (o kernel não salva registradores XMM)
- Linker: `rust-lld` (LLD)
- Panic strategy: `abort`

## Referência

- [Writing an OS in Rust — Philipp Oppermann](https://os.phil-opp.com/)
