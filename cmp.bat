@echo off
nasm -f win64 -o %1.obj %1.asm && (
    gcc %1.obj -o %1.exe && (
        %1
    )
)
