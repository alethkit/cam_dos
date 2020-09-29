ENTRY(_wumbo)

SECTIONS {
    . = 0x80000;
    .text : {
        *(.text._wumbo)
        *(.text*)
    }
    .rodata : { *(.rodata*) }
    .data : { *(.data*) }
    .bss : 
    {
        . = ALIGN(8);
        __bss_start = .;
        *(.bss*);
        . = ALIGN(8);
        __bss_end = .;
    }
}
