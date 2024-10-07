use crate::println;
use crate::print;
use core::ptr::null_mut;
use bitfield_struct::bitfield;

const PAGE_SIZE: usize = 4096;

extern "C" {
    static _heap_start: usize;
    static _heap_end:   usize;
}

static mut KMEM_HEAD: *const usize = null_mut();
static mut KMEM_END: *const usize = null_mut();
static mut KMEM_NUM_PAGES: usize = 0;

pub struct Alloc {
    start: *const usize,
    end:   *const usize,
    num_pages:    usize,
}

#[bitfield(u8)]
struct Page {
    taken: bool,
    last: bool,
    #[bits(6)]
    padding: usize
}

impl Alloc {

    pub fn init() {
        unsafe {
            let heap_size = (&_heap_end as *const usize as usize) - (&_heap_start as *const usize as usize);

            KMEM_HEAD = Alloc::get_heap_start();
            KMEM_END = Alloc::get_heap_end();
            KMEM_NUM_PAGES = heap_size / PAGE_SIZE;

            // Initializing pages.
            let mut temp_page = KMEM_HEAD as *mut Page;
            for i in 0..KMEM_NUM_PAGES {
                (*temp_page).set_taken(false);
                // If at last page
                if i == KMEM_NUM_PAGES - 1 {
                    (*temp_page).set_last(true);
                } else {
                    (*temp_page).set_last(false);
                }
                temp_page = temp_page.offset(PAGE_SIZE as isize);
            }
            if temp_page != KMEM_END as *mut Page {
                println!("SOMETHING WENT WRONG");
            }
        }
    }

    fn get_heap_start() -> *const usize {
        unsafe {
            return &_heap_start as *const usize;
        }
    }

    fn get_heap_end() -> *const usize {
        unsafe {
            return &_heap_end as *const usize;
        }
    }
}