bits 64
default rel

segment .data
    msg db "Hello world!", 0xd, 0xa, 0
    // Explanation for parameters:
    // msg: The string to be printed
    // db: Define Byte
    // "Hello world!": The string to be printed
    // 0xd: Carriage return (equivaletn to '\r')
    // 0xa: Line feed (equivalent to '\n')
    // 0: Null terminator (end of string

segment .text
global main
extern ExitProcess

extern printf

main:
    push    rbp         // Save the base pointer (so we can restore it later)
    mov     rbp, rsp    // Set the base pointer to the current stack pointer
    sub     rsp, 32     // Allocate 32 bytes of space on the stack

    lea     rcx, [msg]  // Load the address of the message into rcx
    call    printf      // Call the printf function

    xor     rax, rax    // Set rax to 0 (return value)
    call    ExitProcess // Call the ExitProcess function