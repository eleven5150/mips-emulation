LC0:
        .ascii  "The latest prime number:\0"
LC1:
        .ascii  "%d\0"
main:
        addiu   $sp,$sp,-48
        sw      $31,44($sp)
        sw      $fp,40($sp)
        move    $fp,$sp
        sw      $4,48($fp)
        sw      $5,52($fp)
        li      $2,10                 
        sw      $2,24($fp)
        sw      $0,28($fp)
        b       L2
        nop

L6:
        lw      $2,28($fp)
        nop
        addiu   $2,$2,1
        sw      $2,28($fp)
        sw      $0,32($fp)
        li      $2,1                      
        sw      $2,36($fp)
        b       L3
        nop

L5:
        lw      $3,28($fp)
        lw      $2,36($fp)
        nop
        div     $0,$3,$2
        bne     $2,$0,1f
        nop
        break   7
        mfhi    $2
        bne     $2,$0,L4
        nop

        lw      $2,32($fp)
        nop
        addiu   $2,$2,1
        sw      $2,32($fp)
L4:
        lw      $2,36($fp)
        nop
        addiu   $2,$2,1
        sw      $2,36($fp)
L3:
        lw      $3,36($fp)
        lw      $2,28($fp)
        nop
        slt     $2,$2,$3
        beq     $2,$0,L5
        nop

LBE3 = .
        lw      $3,32($fp)
        li      $2,2                        # 0x2
        bne     $3,$2,L2
        nop

        lw      $2,24($fp)
        nop
        addiu   $2,$2,-1
        sw      $2,24($fp)
L2:
LBE2 = .
        lw      $2,24($fp)
        nop
        bgtz    $2,L6
        nop

        lui     $2,%hi(LC0)
        addiu   $4,$2,%lo(LC0)
        jal     puts
        nop

        lw      $5,28($fp)
        lui     $2,%hi(LC1)
        addiu   $4,$2,%lo(LC1)
        jal     printf
        nop

        move    $2,$0
        move    $sp,$fp
        lw      $31,44($sp)
        lw      $fp,40($sp)
        addiu   $sp,$sp,48
        jr      $31
        nop