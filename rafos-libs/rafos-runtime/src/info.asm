.align 2
.section .module_info
.global smodule_info
.global emodule_info
smodule_info:
    .incbin "/home/zfl/workspace/rafos/rafos-libs/rafos-runtime/info.txt"
emodule_info:
