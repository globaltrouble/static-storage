# Static storage
The package provide a way to store data in separate memory section during device software resets (no power loss).

Can be used to cache some settings between device reboots to speedup its startup time on reboot or to store debug information for hardware panic/exception. 

### WARNING the data will be lost in case of power loss because all data is stored in RAM.


### Usage:
    * Declare static variable and put it into the separate segment (`.static`):
    ```
    #[link_section = ".state"]
    static mut COUNTER_STORAGE: StaticStorage<u32> = StaticStorage::<u32>::empty();
    ```

    * Add deffered initialization for this var:
    ```
    unsafe { COUNTER_STORAGE.init(|| 0) };
    ```

    * Safe and use reference to static storage:
    ```
    let counter = unsafe { COUNTER_STORAGE.as_mut() }.expect("No static storage!");
    if *counter == 0 {
        // first boot
        // do whatever you want
    } else {
        // was rebooted
        // do whatever you want
    }
    *counter += 1;
    ```

    *  Add new segment and section to a linker script (`memory.x` file):
    ```
    MEMORY {
        /* Some code for you linker ...*/

        STATIC(rw) : ORIGIN = 0x20040000, LENGTH = 24K   /* add memory region, can be splitted from RAM */
    }

    /* Add new section with NOLOAD attribute to keep it AS IS in memory, whithout cleaning it on startup/reset.
    SECTIONS {
        .static ORIGIN(STATIC) (NOLOAD):
        {
            . = ALIGN(4);  /* alignment can be device dependent and can be changed for you device */
            KEEP(*(.static))
        } > STATIC
    } INSERT AFTER .text;
    ```