  .section .text.__hard_fault_trampoline
  .global __hard_fault_trampoline
  .thumb_func
__hard_fault_trampoline:
  mrs r0, MSP
  bl hard_fault
