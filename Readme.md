# Objgrep

A tool to disasemble binarys

## Installation

To install `objgrep` run:
```bash
cargo install objgrep
```

## Usage

To dump a object file simply run:

```bash
objgrep test.o
```

Which showes all infos: (Output of dump for Hello World! C program):
```
test.o Elf X86_64-Little-endian

Sections:
   Name                 Size     Align  Kind
                        0x0      0x0     Metadata
   .text                0x23     0x1     Text
   .rela.text           0x30     0x8     Metadata
   .data                0x0      0x1     Data
   .bss                 0x0      0x1     Uninitialized Data
   .rodata              0xc      0x1     Read Only Data
   .comment             0x2c     0x1     Other String
   .note.GNU-stack      0x0      0x1     Other
   .note.gnu.property   0x20     0x8     Note
   .eh_frame            0x38     0x8     Read Only Data
   .rela.eh_frame       0x18     0x8     Metadata
   .symtab              0x90     0x8     Metadata
   .strtab              0x14     0x1     Metadata
   .shstrtab            0x74     0x1     Metadata
Syms:
   Bind     Typ          Symbol  Section
   Unknown   Null                 Undefined
   Private   File        test.c   
   Global    Func        main     .text
   Global    Unknown     printf   Undefined

entry: 0

main:
   0xF3 0xF 0x1E 0xFA             |  endbr64        
   0x55                           |  push rbp
   0x48 0x89 0xE5                 |  mov rbp, rsp
   0x48 0x8D 0x5 0x0 0x0 0x0 0x0  |  lea rax, [rip] # .rodata + 0x7
   0x48 0x89 0xC7                 |  mov rdi, rax
   0xB8 0x0 0x0 0x0 0x0           |  mov eax, 0
   0xE8 0x0 0x0 0x0 0x0           |  call 0x1c      # printf     
   0xB8 0x0 0x0 0x0 0x0           |  mov eax, 0
   0x5D                           |  pop rbp
   0xC3                           |  ret

```