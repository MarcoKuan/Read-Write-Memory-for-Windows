# Read/Write Process Memory for Windows

A terminal project that was made using "[Rust for Windows](https://github.com/microsoft/windows-rs)". This program will run a terminal. The user will run the program and the person running the terminal will input one of three commands that will be perform certain functions.

|                              |           |    Format            |                                               |             Example 1         |          Example 2            |
|------------------------------|-----------|----------------------|-----------------------------------------------|-------------------------------|-------------------------------|
|           Name               |  \[cmd\]  |      \[addr\]        |                 \[w/r bytes\]                 |                               |                               |
|       (P)rocess Attach       |     P     | [process in decimal] |                                               | P 103                         |                               |
|       Process (R)ead         |     W     | [Address in dec/hex] |       [How many bytes to read in dec]         | R 0xDEADBEEF 5                | R 3735928559 5                |
|       Process (W)rite        |     R     | [Address in dec/hex] |  [String of byte code instructions to write]  | W 0xDEADBEEF \xEF\xBE\xAD\xDE | W 3735928559 \xEF\xBE\xAD\xDE |

[Note]: To terminate the program, please use Ctrl-C to terminate the program.

Documentation is provided in the source code. A video will be provided on how to use this program. 
