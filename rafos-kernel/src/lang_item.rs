use sbi_rt::*;


///
#[lang = "eh_personality"]
#[no_mangle]
pub fn rust_eh_personality() {}

#[no_mangle]
pub fn _Unwind_Resume() {}

/// not_kernel panic
#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    log::warn!("{info}");
    system_reset(Shutdown, SystemFailure);
    unreachable!()
}
