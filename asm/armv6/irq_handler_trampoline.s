  .section .text.__irq_handler_trampoline
  .global __irq_handler_trampoline
  .thumb_func
__irq_handler_trampoline:
  mrs r0, IPSR
  lsr r0, r0, #4
  bl irq_handler
