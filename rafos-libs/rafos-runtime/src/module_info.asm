.align 3
.section .module_info
.global smodule_info
.global emodule_info
smodule_info:
    .incbin "/home/zfl/u-intr/rafos/rafos-libs/rafos-runtime/info.txt"
emodule_info:
