__entry = 0x8000;

ENTRY(__entry)

SECTIONS {
    . = 0x8000;

    .text : {
        *(.init)
        *(.text)
        *(.text.*)
    }

    /DISCARD/ : { *(.comment*) }
}
