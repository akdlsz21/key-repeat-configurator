use winapi::shared::minwindef::{DWORD, UINT};
use winapi::um::winuser::{SystemParametersInfoW, SPI_SETFILTERKEYS};

#[repr(C)]
pub struct FILTERKEYS {
    pub cb_size: u32, // Size of the structure in bytes
    //https://learn.microsoft.com/en-us/windows/win32/api/winuser/ns-winuser-filterkeys
    pub dw_flags: u32,      // Flags to enable filter keys and other options
    pub i_wait_msec: u32,   // Wait time in milliseconds (not used in this context)
    pub i_delay_msec: u32,  // Delay before key repetition starts, in milliseconds
    pub i_repeat_msec: u32, // Repeat rate in milliseconds
    pub i_bounce_msec: u32, // Bounce time in milliseconds (not used in this context)
}

pub fn set_key_repeat_windows(delay: u64, rate: u64) {
    // Calculate repeat rate in milliseconds based on CPS (Characters Per Second)
    let rate_ms = if rate > 0 {
        1000 / rate // Convert CPS to milliseconds per character
    } else {
        eprintln!("Repeat rate must be greater than zero.");
        return;
    };

    let fkf_filterkeyson = 0x00000001;
    let fkf_available = 0x00000002;

    // Initialize the FILTERKEYS structure
    let mut keys = FILTERKEYS {
        cb_size: std::mem::size_of::<FILTERKEYS>() as u32,
        dw_flags: fkf_filterkeyson | fkf_available, // Enable filter keys
        i_wait_msec: 0,
        i_delay_msec: delay as u32,
        i_repeat_msec: rate_ms as u32,
        i_bounce_msec: 0,
    };

    unsafe {
        // Apply the keyboard settings using SystemParametersInfoW
        if SystemParametersInfoW(SPI_SETFILTERKEYS, 0, &mut keys as *mut _ as *mut _, 0) != 0 {
            println!(
                "Keyboard repeat settings updated successfully: delay = {} ms, rate = {} cps",
                delay, rate
            );
        } else {
            eprintln!("Failed to update keyboard repeat settings.");
        }
    }
}
