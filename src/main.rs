use std::ptr::read_volatile;

fn main(){
    let pru_start = (0x4A30_0000 + 0x200/4) as *mut u32;
    unsafe{ println!("{:#08x}", read_volatile(pru_start));}
}
