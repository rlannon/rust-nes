# rust-nes

An NES emulator, written entirely in Rust.

## How It Works

This emulator uses interpretation, not dynamic recompilation. While the latter method is preferred for newer consoles (as interpretation is simply too slow), interpretation should yield OK performance with a console like the NES on a modern machine. Plus, it is incredibly accurate to the original, something that matters if you're doing any sort of homebrew. I also intend on reproducing all of the original known bugs/quirks of the design, as although they may be bugs, they matter for an accurate emulator.

While this processor uses interpretation, I do not intend on making it entirely cycle-accurate, as I'm not emulating the microcode -- at least not for the time being.

## Getting Started

This system will use the emulator-standard `.nes` file format, so all of your favorite ROMs should run in it. The goal of the emulator is to be well-suited for homebrew development, so I intend on including a variety of debugging tools. Note this repo does not include an assembler, tough a variety of others (such as [NESasm](https://github.com/camsaul/nesasm)) are publicly available, so that shouldn't be an issue.

Given this project uses Rust, I suggest using `cargo` to build it.
