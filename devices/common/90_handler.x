
/* PROVIDE exception handlers */
PROVIDE(nmi = default_exception_handler);
PROVIDE(hard_fault = __hard_fault_trampoline);
PROVIDE(mem_manage = default_exception_handler);
PROVIDE(bus_fault = default_exception_handler);
PROVIDE(usage_fault = default_exception_handler);
PROVIDE(sv_call = default_exception_handler);
PROVIDE(debug_monitor = default_exception_handler);
PROVIDE(pend_sv = default_exception_handler);
PROVIDE(sys_tick = default_exception_handler);
PROVIDE(irq_handler = __irq_handler_trampoline);
