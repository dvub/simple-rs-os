use crate::{gdt, print, println};
use lazy_static::lazy_static;
use pic8259::ChainedPics;
use x86_64::structures::idt::{InterruptDescriptorTable, InterruptStackFrame};

#[derive(Debug, Clone, Copy)]
#[repr(u8)]
pub enum InterruptIndex {
    Timer = PIC_1_OFFSET,
}
impl InterruptIndex {
    fn as_u8(self) -> u8 {
        self as u8
    }
    fn as_size(self) -> usize {
        usize::from(self.as_u8())
    }
}

pub const PIC_1_OFFSET: u8 = 32;
pub const PIC_2_OFFSET: u8 = PIC_1_OFFSET + 8;

pub static PICS: spin::Mutex<ChainedPics> =
    spin::Mutex::new(unsafe { ChainedPics::new(PIC_1_OFFSET, PIC_2_OFFSET) });

// shit is getting tricky here
// IDT = interrupt descriptor table, which is just a set of instructions that say what to do when a certain interrupt occurs
// mostly we're just handling simple shit right now
lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();
        idt.breakpoint.set_handler_fn(breakpoint_handler); // breakpoints are related to debugging, kind of neat

        // 2 things are important here
        // of course, the df handler so that we know what to do when a double fault occurs
        // and ALSO the double fault ist index
        // ist = interrupt stack table
        // basically, we want to CHANGE STACKS right before a handler is called
        // so that we can "recover from corrupt stack pointers"
        unsafe {
            idt.double_fault
                .set_handler_fn(double_fault_handler)
                .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX);
        }
        idt[InterruptIndex::Timer.as_size()].set_handler_fn(timer_interrupt_handler);
        idt
    };
}
// to be totally honest, i don't know why we wrap this (??)
pub fn init_idt() {
    IDT.load();
}

// `extern x86-interrupt` tells us to use a specific type of calling convention
extern "x86-interrupt" fn breakpoint_handler(stack_frame: InterruptStackFrame) {
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
}

extern "x86-interrupt" fn timer_interrupt_handler(_stack_frame: InterruptStackFrame) {
    print!(".");
    // we must explicitly "say" that we are done processing the timer interrupt
    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Timer.as_u8());
    }
}

// EXTREMELY IMPORTANT!!!
// we DEFINITELY VERY MUCH NEED TO HANDLE THIS INTERRUPT

extern "x86-interrupt" fn double_fault_handler(
    stack_frame: InterruptStackFrame,
    _error_code: u64,
) -> ! {
    panic!("EXCEPTION: DOUBLE FAULT\n{:#?}", stack_frame);
}
