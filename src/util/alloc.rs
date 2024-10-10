use crate::println;
use crate::print;
use core::ptr::null_mut;
use core::{mem, assert};
use core::option::Option;
use bitfield_struct::bitfield;

const PAGE_SIZE: usize = 4096 + 2;

extern "C" {
    static _heap_start: usize;
    static _heap_end:   usize;
}

static mut KMEM_HEAD: *const usize = null_mut();
static mut KMEM_END: *const usize = null_mut();
static mut KMEM_NUM_PAGES: usize = 0;

pub struct Alloc;

#[bitfield(u16)]
pub struct Page {
    taken: bool,
    last: bool,
    #[bits(14)]
    num_reserved: usize
}

impl Alloc {
    pub fn init() {
        unsafe {
            let heap_size = (&_heap_end as *const usize as usize) - (&_heap_start as *const usize as usize);

            // Initialize static members.
            KMEM_HEAD = Alloc::get_heap_start();
            KMEM_END = Alloc::get_heap_end();
            KMEM_NUM_PAGES = heap_size / PAGE_SIZE;
            
            // Initializing pages.
            let mut temp_page = KMEM_HEAD as *mut Page;
            for i in 0..KMEM_NUM_PAGES {
                
                // Initialize fields.
                (*temp_page).set_taken(false);
                (*temp_page).set_num_reserved(0);
                
                // If at last page
                if i == KMEM_NUM_PAGES - 1 {
                    (*temp_page).set_last(true);
                } else {
                    (*temp_page).set_last(false);
                }

                // Go to the next page.
                temp_page = temp_page.offset(PAGE_SIZE as isize);
            }

            assert!(temp_page > KMEM_END as *mut Page, "Did not initialize all pages");
        }
    }

    pub fn get(num_requested: usize) -> Option<*mut Page> {
        let mut page;
        unsafe {
            page = KMEM_HEAD as *mut Page;
            let mut temp_page = page;
            let mut num_pages_curr = 1;
            while !(*temp_page).last() {
                // println!("Addr: {}", (*page).num_reserved() as isize);
                if (*page).taken() {
                    let num_pages_to_jump = (*page).num_reserved();
                    page = page.offset((PAGE_SIZE * num_pages_to_jump / 2) as isize);
                    temp_page = page;
                    continue;
                }

                // If the lead page is on a taken page move page then increment page after.
                if (*temp_page).taken() {
                    page = temp_page;
                    num_pages_curr = 1;
                    continue;
                }

                // If neither the head or tail are taken then you have a valid block.
                if num_pages_curr == num_requested {
                    (*page).set_taken(true);
                    (*page).set_num_reserved(num_requested);
                    return Some(page.offset(1));
                }

                // Increment the tail.
                temp_page = temp_page.offset((PAGE_SIZE / 2) as isize);
                num_pages_curr += 1;
            }
        }
        // No space.
        return None;
    }

    pub fn free<T>(ptr: *const T) {
        unsafe {
            let mut page = ptr as *mut Page;
            page = page.offset(-1);
            
            let num_pages_to_free = (*page).num_reserved();
            for _ in 0..num_pages_to_free {
                (*page).set_taken(false);
                (*page).set_num_reserved(0);
                page = page.offset((PAGE_SIZE / 2) as isize);
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