use lazy_static::lazy_static;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable, SegmentSelector};
use x86_64::structures::tss::TaskStateSegment;
use x86_64::VirtAddr;

// we want to switch to the stack at the TOP of the IST when a double fault occurs (index 0)
pub const DOUBLE_FAULT_IST_INDEX: u16 = 0;

lazy_static! {
    static ref TSS: TaskStateSegment = {
        let mut tss = TaskStateSegment::new();
        // set the top stack to have a stack size of 4096 * 5 = 20,480 (??)
        tss.interrupt_stack_table[DOUBLE_FAULT_IST_INDEX as usize] = {
            const STACK_SIZE: usize = 4096 * 5;
            // fill it with emptiness
            static mut STACK: [u8; STACK_SIZE] = [0; STACK_SIZE];

            let stack_start = VirtAddr::from_ptr(unsafe { &STACK });

            stack_start + STACK_SIZE
        };
        tss
    };
}
// wrapper struct
// since we change the GDT,
//  we'll need this struct to keep track of our code/tss selectors to reload them after changing the GDT
struct Selectors {
    code_selector: SegmentSelector,
    tss_selector: SegmentSelector,
}

lazy_static! {
    //
    static ref GDT: (GlobalDescriptorTable, Selectors) = {
        let mut gdt = GlobalDescriptorTable::new();
        // add selectors or some shit, idk (??)
        // additionally, save them so we can reload them elsewhere
        let code_selector = gdt.add_entry(Descriptor::kernel_code_segment());
        let tss_selector = gdt.add_entry(Descriptor::tss_segment(&TSS));
        (
            gdt,
            Selectors {
                code_selector,
                tss_selector,
            },
        )
    };
}

pub fn init() {
    use x86_64::instructions::{
        segmentation::{Segment, CS},
        tables::load_tss,
    };
    GDT.0.load(); // load our own global descriptor table
    unsafe {
        CS::set_reg(GDT.1.code_selector); // reloads `cs` - that is, the code segment register
        load_tss(GDT.1.tss_selector); // we have a new TSS so uh yeah lets use that too
    }
}
