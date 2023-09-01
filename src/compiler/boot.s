# Boot start
.section .rodata
_BOOT_MESSAGE:
    .string "Boot complete\n"
_EXIT_MESSAGE:
    .string "The program exited with status %d\n"

.equ    _TEST_BASE, 0x00100000
.equ    _SHUTDOWN, 0x5555

.section .text.init
.global _start
_start:
.option push
.option norelax
    la      gp, _global_pointer
.option pop
    la      sp, _stack_end

    la      tp, _head_start

    call    init_uart
    #la      a0, _BOOT_MESSAGE
    #call    printf

    call    main

    mv      a1, a0
    la      a0, _EXIT_MESSAGE
    call    printf

    li      t0, _TEST_BASE
    li      t1, _SHUTDOWN
1:  sw      t1, 0(t0)
    j       1b
# Boot end

# Printf start
.global printf
printf:
    addi    sp, sp, -104
    sd      ra, 0(sp)
    sd      a1, 8(sp)
    sd      a2, 16(sp)
    sd      a3, 24(sp)
    sd      a4, 32(sp)
    sd      a5, 40(sp)
    sd      a6, 48(sp)
    sd      a7, 56(sp)
    sd      s0, 64(sp)
    sd      s1, 72(sp)
    sd      s2, 80(sp)
    sd      s3, 88(sp)
    sd      s4, 96(sp)

    mv      s0, a0
    addi    s1, sp, 8
    li      s2, 37
    li      s3, 100
    j       2f
1:
    addi    s0, s0, 1
2:
    lb      t0, 0(s0)
    beq     t0, s2, 3f
    beqz    t0, 8f
    mv      a0, t0
    call    write_char
    j       1b

3:
    addi    s0, s0, 1
    lb      t0, 0(s0)
    beq     t0, s3, 4f
    mv      s4, t0
    li      a0, 37
    call    write_char
    beqz    s4, 8f
    mv      a0, s4
    call    write_char
    j       1b

4:
    ld      t1, 0(s1)
    addi    s1, s1, 8
    li      t2, 10000000000000000000
    li      t3, 10
    j       6f
5:
    divu    t2, t2, t3
6:
    divu    t4, t1, t2
    beqz    t4, 5b
7:
    addi    a0, t4, 48
    call    write_char
    remu    t1, t1, t2
    divu    t2, t2, t3
    beqz    t2, 1b
    divu    t4, t1, t2
    j       7b

8:
    ld      ra, 0(sp)
    ld      s0, 64(sp)
    ld      s1, 72(sp)
    ld      s2, 80(sp)
    ld      s3, 88(sp)
    ld      s4, 96(sp)
    addi    sp, sp, 104
    ret
# Printf end

# write_char start
.equ    UART_BASE, 0x10000000
.equ    RBR, 0
.equ    THR, 0
.equ    IER, 1
.equ    DLL, 0
.equ    DLM, 1
.equ    IIR, 2
.equ    FCR, 2
.equ    LCR, 3
.equ    MCR, 4
.equ    LSR, 5
.equ    MSR, 6
.equ    SCR, 7

.global init_uart
init_uart:
        li      t0, UART_BASE

        # DLAB = 0, 8-bit, 1 stop bit, no parity, no break
        li      t1, 0b00000011
        sb      t1, LCR(t0)

        # Disable FIFO
        sb      zero, FCR(t0)

        # Disable interrupts
        sb      zero, IER(t0)
        ret

.global write_char
write_char:
        li      t0, UART_BASE
        sb      a0, THR(t0)
        ret
