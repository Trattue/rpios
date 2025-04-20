// We will perform boot on one core (the master core) only. For this we put all
// cores except #0 into wait. To find out the core ID, we check the lower byte
// of the mpidr_el1 register.
mrs x4, mpidr_el1
and x4, x4, #0xFF
cbz x4, 1f
b 4f

// This code is only executed on the master core. Here we will initialize our
// Rust runtime by zero-ing the BSS section and setting the stack pointer.
1:
ldr x0, __bss_start
ldr x1, __bss_end
2:
cmp x0, x1
beq 3f
stp	xzr, xzr, [x0], #16
b 2b
3:
ldr x0, =__boot_core_stack_end
mov sp, x0
b _start_rust

4:
// End of assembly
