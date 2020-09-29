ENTRY(_wumbo)

SECTIONS {
    . = 0x80000;
    .text : {
        *(.text._wumbo)
        *(.text*)
    }
    . = ALIGN(0x1000);
    .rodata : { *(.rodata*) }
    . = ALIGN(0x1000);
    .data : { *(.data*) }
    . = ALIGN(0x1000);
    .bss : 
    {
        __bss_start = .;
        *(.bss*);
        . = ALIGN(0x1000);
        __bss_end = .;
    }
}
