# woah its another NES emulator

Yep

# Features

-   Native frontend using `iced`
-   WASM frontend
-   One save-state slot
-   Runs at 60FPS

## CPU

-   All of the official opcodes are correctly implemented and tested

## PPU

-   Fully implemented
-   Mostly accurate

## APU

-   Has square channels #1 and #2, triangle channel, and noise channel
-   DMC emulation eventually
-   Native frontend no longer makes funky popping noises
-   WASM frontend needs a bit of work

## Mapper list

-   NROM (0)
-   MMC1 (1)
-   UxROM (2)
-   AxROM (7)
