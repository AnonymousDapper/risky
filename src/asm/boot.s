# # MIT License

# # Copyright (c) 2021 AnonmousDapper

# # bootloader for risky

.option norvc

.section .text.init

.globl _start
_start:

.option push
.option norelax
    la gp, _global_pointer
.option pop

    csrw satp, zero

    csrr t0, mhartid
    bnez t0, 3f

    la   a0, _bss_start
    la   a1, _bss_end
    bgeu a0, a1, 2f
1:
    sd   zero, (a0)
    addi a0, a0, 8
    bltu a0, a1, 1b

2:
    la sp, _stack_end
    
    li   t0, 0b11 << 11 | (1 << 7) | (1 << 3)
    csrw mstatus, t0

    li   t2, (1 << 3) | (1 << 7) | (1 << 11)
    csrw mie, t2

    la   t3, asm_trap_vector
    csrw mtvec, t3

    la   t1, kmain
    csrw mepc, t1

    la   ra, park

    mret # VTEC engage

3:


park:
        wfi

        j park
