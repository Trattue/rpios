// Put all cores except #0 into wait
mrs x4, mpidr_el1
and x4, x4, #0xFF
cbz x4, 1f
b 4f

// Master core: Zero BSS and set stack pointer
1:
ldr x0, __bss_start
ldr x1, __bss_end
2:
cmp x0, x1
beq 3f
stp	xzr, xzr, [x0], #16
b 2b
3:
ldr x0, __boot_core_stack_end
mov sp, x0
b _start_rust

4:
// End of assembly
