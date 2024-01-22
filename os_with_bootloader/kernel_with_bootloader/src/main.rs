#![no_std]
#![no_main]

use bootloader_api::config::Mapping;
use x86_64::instructions::hlt;


pub static BOOTLOADER_CONFIG: bootloader_api::BootloaderConfig = {let mut config = bootloader_api::BootloaderConfig::new_default();
    config.mappings.physical_memory = Some(Mapping::Dynamic);
    config.kernel_stack_size = 100 * 1024;
    config
};
bootloader_api::entry_point!(my_entry_point, config = 
&BOOTLOADER_CONFIG);


fn my_entry_point(boot_info: &'static mut bootloader_api::BootInfo) -> ! {
    loop {
    hlt();//stop x86_64 from being unnecessarily busy while looping
    }
    }


    #[panic_handler]
    fn panic(_info: &core::panic::PanicInfo) -> ! {
        loop {
        hlt();
        }
        }