	.arch armv8-a
	.file	"test.c"
	.text
	.section	.rodata
	.align	3
.LC0:
	.string	"Hello world!"
	.text
	.align	2
	.global	main
	.type	main, %function
main:
.LFB0:
	.cfi_startproc
	stp	x29, x30, [sp, -48]!
	.cfi_def_cfa_offset 48
	.cfi_offset 29, -48
	.cfi_offset 30, -40
	mov	x29, sp
	adrp	x0, .LC0
	add	x0, x0, :lo12:.LC0
	str	x0, [sp, 32]
	ldr	x0, [sp, 32]
	str	x0, [sp, 40]
	mov	w0, 12
	str	w0, [sp, 28]
	b	.L2
.L3:
	ldr	x0, [sp, 40]
	ldrb	w0, [x0]
	bl	putchar
	ldr	x0, [sp, 40]
	add	x0, x0, 1
	str	x0, [sp, 40]
.L2:
	ldr	x1, [sp, 40]
	ldr	x0, [sp, 32]
	sub	x1, x1, x0
	ldrsw	x0, [sp, 28]
	cmp	x1, x0
	blt	.L3
	mov	w0, 10
	bl	putchar
	mov	w0, 0
	ldp	x29, x30, [sp], 48
	.cfi_restore 30
	.cfi_restore 29
	.cfi_def_cfa_offset 0
	ret
	.cfi_endproc
.LFE0:
	.size	main, .-main
	.ident	"GCC: (Debian 10.2.1-6) 10.2.1 20210110"
	.section	.note.GNU-stack,"",@progbits
