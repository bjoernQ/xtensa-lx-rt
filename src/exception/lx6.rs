use super::ExceptionCause;

/// State of the CPU saved when entering exception or interrupt
///
/// Must be aligned with assembly frame format in assembly_lx6
#[repr(C)]
#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Context {
    PC: u32,
    PS: u32,

    A0: u32,
    A1: u32,
    A2: u32,
    A3: u32,
    A4: u32,
    A5: u32,
    A6: u32,
    A7: u32,
    A8: u32,
    A9: u32,
    A10: u32,
    A11: u32,
    A12: u32,
    A13: u32,
    A14: u32,
    A15: u32,
    SAR: u32,
    EXCCAUSE: u32,
    EXCVADDR: u32,
    LBEG: u32,
    LEND: u32,
    LCOUNT: u32,
    THREADPTR: u32,
    SCOMPARE1: u32,
    BR: u32,
    ACCLO: u32,
    ACCHI: u32,
    M0: u32,
    M1: u32,
    M2: u32,
    M3: u32,
    F64R_LO: u32,
    F64R_HI: u32,
    F64S: u32,
    FCR: u32,
    FSR: u32,
    F0: u32,
    F1: u32,
    F2: u32,
    F3: u32,
    F4: u32,
    F5: u32,
    F6: u32,
    F7: u32,
    F8: u32,
    F9: u32,
    F10: u32,
    F11: u32,
    F12: u32,
    F13: u32,
    F14: u32,
    F15: u32,

    reserved: [u32; 7],
    BASESAVE: [u32; 4],
}

extern "Rust" {
    /// This symbol will be provided by the user via `#[exception]`
    fn __exception(cause: ExceptionCause);
    /// No attribute is supplied for this symbol as the double exception can hardly occur
    fn __double_exception(cause: ExceptionCause);

    /// This symbol will be provided by the user via `#[interrupt(1)]`
    fn __level_1_interrupt(level: u32);
    /// This symbol will be provided by the user via `#[interrupt(2)]`
    fn __level_2_interrupt(level: u32);
    /// This symbol will be provided by the user via `#[interrupt(3)]`
    fn __level_3_interrupt(level: u32);
    /// This symbol will be provided by the user via `#[interrupt(4)]`
    fn __level_4_interrupt(level: u32);
    /// This symbol will be provided by the user via `#[interrupt(5)]`
    fn __level_5_interrupt(level: u32);
    /// This symbol will be provided by the user via `#[interrupt(6)]`
    fn __level_6_interrupt(level: u32);
    /// This symbol will be provided by the user via `#[interrupt(7)]`
    fn __level_7_interrupt(level: u32);
}

#[no_mangle]
#[link_section = ".rwtext"]
extern "C" fn __default_exception(cause: ExceptionCause, save_frame: &Context) {
    panic!("Exception: {:?}, {:08x?}", cause, save_frame)
}

#[no_mangle]
#[link_section = ".rwtext"]
extern "C" fn __default_interrupt(level: u32, save_frame: &Context) {
    panic!("Interrupt: {:?}, {:08x?}", level, save_frame)
}

#[no_mangle]
#[link_section = ".rwtext"]
extern "C" fn __default_double_exception(cause: ExceptionCause, save_frame: &Context) {
    panic!("Double Exception: {:?}, {:08x?}", cause, save_frame)
}

// Raw vector handlers
//
// The interrupt handlers all use special return instructions.
// rust still generates a ret.w instruction, which will never be reached.
// generation of the ret.w can be prevented by using core::intrinsics::unreachable,
// but then a break 15,1 will be generated (which takes 3 bytes instead of 2) or a 'loop {}',
// but then a jump to own address will be generated which is also 3 bytes.
// No way found yet to prevent this generation altogether.

#[naked]
#[no_mangle]
#[link_section = ".KernelExceptionVector.text"]
unsafe extern "C" fn _KernelExceptionVector() {
    llvm_asm!(
        "
        wsr a0, EXCSAVE1 // preserve a0
        rsr a0, EXCCAUSE // get exception cause

        beqi a0, 5, .AllocAException

        call0 __naked_kernel_exception
        "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".UserExceptionVector.text"]
unsafe extern "C" fn _UserExceptionVector() {
    llvm_asm!(
        "
        wsr a0, EXCSAVE1 // preserve a0
        rsr a0, EXCCAUSE // get exception cause

        beqi a0, 5, .AllocAException

        call0 __naked_user_exception

        .AllocAException:
        call0  _AllocAException
        "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".DoubleExceptionVector.text"]
unsafe extern "C" fn _DoubleExceptionVector() {
    llvm_asm!(
        "
    wsr a0, EXCSAVE1                   // preserve a0 (EXCSAVE1 can be reused as long as there
                                       // is no double exception in the first exception until
                                       // EXCSAVE1 is stored to the stack.)
    call0 __naked_double_exception     // used as long jump
    "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".Level2InterruptVector.text"]
unsafe extern "C" fn _Level2InterruptVector() {
    llvm_asm!(
        "
    wsr a0, EXCSAVE2 // preserve a0
    call0 __naked_level_2_interrupt     // used as long jump
    "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".Level3InterruptVector.text"]
unsafe extern "C" fn _Level3InterruptVector() {
    llvm_asm!(
        "
    wsr a0, EXCSAVE3 // preserve a0
    call0 __naked_level_3_interrupt     // used as long jump
    "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".Level4InterruptVector.text"]
unsafe extern "C" fn _Level4InterruptVector() {
    llvm_asm!(
        "
    wsr a0, EXCSAVE4 // preserve a0
    call0 __naked_level_4_interrupt     // used as long jump
    "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".Level5InterruptVector.text"]
unsafe extern "C" fn _Level5InterruptVector() {
    llvm_asm!(
        "
    wsr a0, EXCSAVE5 // preserve a0
    call0 __naked_level_5_interrupt     // used as long jump
    "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".DebugExceptionVector.text"]
unsafe extern "C" fn _Level6InterruptVector() {
    llvm_asm!(
        "
    wsr a0, EXCSAVE6 // preserve a0
    call0 __naked_level_6_interrupt     // used as long jump
    "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".NMIExceptionVector.text"]
unsafe extern "C" fn _Level7InterruptVector() {
    llvm_asm!(
        "
    wsr a0, EXCSAVE7 // preserve a0
    call0 __naked_level_7_interrupt     // used as long jump
    "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".WindowOverflow4.text"]
unsafe extern "C" fn _WindowOverflow4() {
    llvm_asm!(
        "
        s32e    a0, a5, -16
        s32e    a1, a5, -12
        s32e    a2, a5,  -8
        s32e    a3, a5,  -4
        rfwo
    "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".WindowUnderflow4.text"]
unsafe extern "C" fn _WindowUnderflow4() {
    llvm_asm!(
        "
        l32e    a1, a5, -12
        l32e    a0, a5, -16
        l32e    a2, a5,  -8
        l32e    a3, a5,  -4
        rfwu

        // inline the _AllocAException saves on the ret.w for WindowUnderflow4
        // this makes that it just fits, which is needed for the bbci instructions

        .align 4
        _AllocAException:
        rsr     a0, WINDOWBASE  // grab WINDOWBASE before rotw changes it
        rotw    -1              // WINDOWBASE goes to a4, new a0-a3 are scratch
        rsr     a2, PS
        extui   a3, a2, 8, 4    // XCHAL_PS_OWB_SHIFT, XCHAL_PS_OWB_BITS
        xor     a3, a3, a4      // bits changed from old to current windowbase
        rsr     a4, EXCSAVE1    // restore original a0 (now in a4)
        slli    a3, a3, 8       // XCHAL_PS_OWB_SHIFT
        xor     a2, a2, a3      // flip changed bits in old window base
        wsr     a2, PS          // update PS.OWB to new window base
        rsync

        bbci    a4, 31, _WindowUnderflow4
        rotw    -1              // original a0 goes to a8
        bbci    a8, 30, _WindowUnderflow8
        rotw    -1
        j               _WindowUnderflow12

        "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".WindowOverflow8.text"]
unsafe extern "C" fn _WindowOverflow8() {
    llvm_asm!(
        "
        s32e    a0, a9, -16
        l32e    a0, a1, -12

        s32e    a1, a9, -12
        s32e    a2, a9,  -8
        s32e    a3, a9,  -4
        s32e    a4, a0, -32
        s32e    a5, a0, -28
        s32e    a6, a0, -24
        s32e    a7, a0, -20
        rfwo
    "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".WindowUnderflow8.text"]
unsafe extern "C" fn _WindowUnderflow8() {
    llvm_asm!(
        "
        l32e    a0, a9, -16
        l32e    a1, a9, -12
        l32e    a2, a9,  -8
        l32e    a7, a1, -12

        l32e    a3, a9,  -4
        l32e    a4, a7, -32
        l32e    a5, a7, -28
        l32e    a6, a7, -24
        l32e    a7, a7, -20
        rfwu
    "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".WindowOverflow12.text"]
unsafe extern "C" fn _WindowOverflow12() {
    llvm_asm!(
        "
        s32e    a0,  a13, -16
        l32e    a0,  a1,  -12

        s32e    a1,  a13, -12
        s32e    a2,  a13,  -8
        s32e    a3,  a13,  -4
        s32e    a4,  a0,  -48
        s32e    a5,  a0,  -44
        s32e    a6,  a0,  -40
        s32e    a7,  a0,  -36
        s32e    a8,  a0,  -32
        s32e    a9,  a0,  -28
        s32e    a10, a0,  -24
        s32e    a11, a0,  -20
        rfwo
    "
    );
}

#[naked]
#[no_mangle]
#[link_section = ".WindowUnderflow12.text"]
unsafe extern "C" fn _WindowUnderflow12() {
    llvm_asm!(
        "
        l32e    a0,  a13, -16
        l32e    a1,  a13, -12
        l32e    a2,  a13,  -8
        l32e    a11, a1,  -12

        l32e    a3,  a13,  -4
        l32e    a4,  a11, -48
        l32e    a5,  a11, -44
        l32e    a6,  a11, -40
        l32e    a7,  a11, -36
        l32e    a8,  a11, -32
        l32e    a9,  a11, -28
        l32e    a10, a11, -24
        l32e    a11, a11, -20
        rfwu
    "
    );
}
