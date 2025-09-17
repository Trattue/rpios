__binary_entry = 0x80000;
__boot_core_stack_start = 0x0;

ENTRY(__binary_entry)

/* Flags:
 * 4 == R
 * 5 == RX
 * 6 == RW
 */
PHDRS {
    segment_boot_core_stack PT_LOAD FLAGS(6);
    segment_code PT_LOAD FLAGS(5);
    segment_data PT_LOAD FLAGS(6);
}

SECTIONS {
    /*
     * Segment for stack
     */
    . = __boot_core_stack_start;
    .boot_core_stack (NOLOAD) : {
        . += __binary_entry;
        __boot_core_stack_end = .;
    } :segment_boot_core_stack

    /*
     * Code
     */
    .text : {
        *(.init)
        *(.text)
        *(.text.*)
    } :segment_code

    /*
     * Data
     */
    .rodata : ALIGN(8) {
        *(.rodata*)
    } :segment_code

    .data : {
        *(.data*)
    } :segment_data

    .bss (NOLOAD) : ALIGN(16) {
        __bss_start = .;
        *(.bss*);
        . = ALIGN(16);
        __bss_end = .;
    } :segment_data

    /*
     * Stuff
     */
    /DISCARD/ : { *(.comment*) }
}
