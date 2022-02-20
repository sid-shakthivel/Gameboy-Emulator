# Gameboy Emulator

Rustboy is a GameBoy emulator written in rust. 
All official opcodes are supported, and most of Blargg's tests pass.
However, this emulator doesn't support MBC1 or any form of banking so only simple games such as tetris actually run.

## Supported Games
- Tetris

## Building 
```
git clone https://github.com/sid-shakthivel/Rustboy.git
cd Rustboy
cargo build --release
```

## Running

In development
```
cargo run  [rom_file.gb]
```

In release
```
cargo build --release
./target/release/rustboy [rom_file.gb]
```

**All joypad controls are mapped to their exact keys**

## Supported Platforms
- MacOS

## Blargg's instruction tests
|#|name|state|
|---|---|---|
|01|special|&check;|
|02|interrupts|&cross;|
|03|op sp,hl|&check;|
|04|op r,imm|&check;|
|05|op rp|&check;|
|06|ld r,r|&check;|
|07|jr,jp,call,ret,rst|&check;|
|08|misc instrs|&check;|
|09|op r,r|&check;|
|10|bit ops|&check;|
|11|op a,(hl)|&check;|