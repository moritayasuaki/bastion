//! Serial output and a small framebuffer console for a VPS's web/VNC console.
use crate::{input, out};
use bastion_core::console::OutputQueue;
static mut INTERACTIVE: bool = false;
static mut RX_ERROR: bool = false;
static mut OUTPUT: OutputQueue<4096> = OutputQueue::new();
use core::fmt::{self, Write};

#[repr(C)]
struct Framebuffer {
    address: *mut u32,
    width: u64,
    height: u64,
    pitch: u64,
    bpp: u16,
    model: u8,
    red_size: u8,
    red_shift: u8,
    green_size: u8,
    green_shift: u8,
    blue_size: u8,
    blue_shift: u8,
}
static mut FB: Option<&'static Framebuffer> = None;
static mut X: usize = 16;
static mut Y: usize = 16;

pub unsafe fn init(response: *const u64) {
    out(0x3f9, 0);
    out(0x3fb, 0x80);
    out(0x3f8, 1);
    out(0x3f9, 0);
    out(0x3fb, 3);
    out(0x3fa, 0xc7);
    out(0x3fc, 0x0b);
    if !response.is_null() && *response.add(1) > 0 {
        let list = *response.add(2) as *const *const Framebuffer;
        let fb = &**list;
        if fb.bpp == 32
            && fb.model == 1
            && fb.width >= 640
            && fb.height >= 480
            && fb.red_size == 8
            && fb.green_size == 8
            && fb.blue_size == 8
        {
            FB = Some(fb);
            for y in 0..fb.height as usize {
                for x in 0..fb.width as usize {
                    pixel(fb, x, y, 0x0b, 0x12, 0x20);
                }
            }
        }
    }
}

unsafe fn pixel(fb: &Framebuffer, x: usize, y: usize, r: u32, g: u32, b: u32) {
    let color = (r << fb.red_shift) | (g << fb.green_shift) | (b << fb.blue_shift);
    core::ptr::write_volatile(fb.address.add(y * (fb.pitch as usize / 4) + x), color);
}

struct Console;
impl Write for Console {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            unsafe {
                if INTERACTIVE {
                    let bytes = if byte == b'\n' {
                        &b"\r\n"[..]
                    } else {
                        core::slice::from_ref(&byte)
                    };
                    let _ = OUTPUT.push(bytes);
                    continue;
                }
                if byte == b'\n' {
                    serial_byte(b'\r');
                }
                serial_byte(byte);

                if let Some(fb) = FB {
                    if byte == b'\n' {
                        X = 16;
                        Y += 18;
                        continue;
                    }
                    if X + 12 >= fb.width as usize {
                        X = 16;
                        Y += 18;
                    }
                    if Y + 16 >= fb.height as usize {
                        continue;
                    }
                    let glyph = glyph(byte.to_ascii_uppercase());
                    for (row, bits) in glyph.iter().enumerate() {
                        for col in 0..5 {
                            for dy in 0..2 {
                                for dx in 0..2 {
                                    let on = bits & (1 << (4 - col)) != 0;
                                    let (r, g, b) = if on {
                                        (0xc8, 0xe6, 0xff)
                                    } else {
                                        (0x0b, 0x12, 0x20)
                                    };
                                    pixel(fb, X + col * 2 + dx, Y + row * 2 + dy, r, g, b);
                                }
                            }
                        }
                    }
                    X += 12;
                }
            }
        }
        Ok(())
    }
}
pub fn print(args: fmt::Arguments<'_>) {
    let _ = Console.write_fmt(args);
}

// All access is on one CPU with interrupts masked, as with console output.
unsafe fn line_status() -> u8 {
    let status = input(0x3fd);
    // Reading LSR clears UART errors; retain them even when checking TX readiness.
    if status != 0xff && status & 0x1e != 0 {
        RX_ERROR = true;
    }
    status
}
unsafe fn serial_byte(byte: u8) {
    for _ in 0..10000 {
        if line_status() & 0x20 != 0 {
            out(0x3f8, byte);
            return;
        }
    }
}
pub fn begin_interactive() {
    unsafe {
        INTERACTIVE = true;
    }
}
pub fn output_available() -> usize {
    unsafe { OUTPUT.available() }
}
pub fn drain() {
    unsafe {
        // Never wait for the UART from the timer interrupt.
        for _ in 0..64 {
            if line_status() & 0x20 == 0 {
                break;
            }
            let Some(byte) = OUTPUT.pop() else {
                break;
            };
            out(0x3f8, byte);
        }
    }
}
pub fn read_byte() -> Option<Result<u8, ()>> {
    unsafe {
        let status = line_status();
        if status == 0xff {
            return None;
        }
        let byte = if status & 1 != 0 {
            Some(input(0x3f8))
        } else {
            None
        };
        if RX_ERROR {
            RX_ERROR = false;
            Some(Err(()))
        } else {
            byte.map(Ok)
        }
    }
}
pub fn emergency_mode() {
    unsafe {
        INTERACTIVE = false;
        while let Some(byte) = OUTPUT.pop() {
            serial_byte(byte);
        }
    }
}

// Original 5x7 glyph patterns, used only by the diagnostic console.
fn glyph(c: u8) -> [u8; 7] {
    match c {
        b'A' => [14, 17, 17, 31, 17, 17, 17],
        b'B' => [30, 17, 17, 30, 17, 17, 30],
        b'C' => [14, 17, 16, 16, 16, 17, 14],
        b'D' => [30, 17, 17, 17, 17, 17, 30],
        b'E' => [31, 16, 16, 30, 16, 16, 31],
        b'F' => [31, 16, 16, 30, 16, 16, 16],
        b'G' => [14, 17, 16, 23, 17, 17, 15],
        b'H' => [17, 17, 17, 31, 17, 17, 17],
        b'I' => [14, 4, 4, 4, 4, 4, 14],
        b'J' => [7, 2, 2, 2, 2, 18, 12],
        b'K' => [17, 18, 20, 24, 20, 18, 17],
        b'L' => [16, 16, 16, 16, 16, 16, 31],
        b'M' => [17, 27, 21, 21, 17, 17, 17],
        b'N' => [17, 25, 21, 19, 17, 17, 17],
        b'O' => [14, 17, 17, 17, 17, 17, 14],
        b'P' => [30, 17, 17, 30, 16, 16, 16],
        b'Q' => [14, 17, 17, 17, 21, 18, 13],
        b'R' => [30, 17, 17, 30, 20, 18, 17],
        b'S' => [15, 16, 16, 14, 1, 1, 30],
        b'T' => [31, 4, 4, 4, 4, 4, 4],
        b'U' => [17, 17, 17, 17, 17, 17, 14],
        b'V' => [17, 17, 17, 17, 17, 10, 4],
        b'W' => [17, 17, 17, 21, 21, 21, 10],
        b'X' => [17, 17, 10, 4, 10, 17, 17],
        b'Y' => [17, 17, 10, 4, 4, 4, 4],
        b'Z' => [31, 1, 2, 4, 8, 16, 31],
        b'0' => [14, 17, 19, 21, 25, 17, 14],
        b'1' => [4, 12, 4, 4, 4, 4, 14],
        b'2' => [14, 17, 1, 2, 4, 8, 31],
        b'3' => [30, 1, 1, 14, 1, 1, 30],
        b'4' => [2, 6, 10, 18, 31, 2, 2],
        b'5' => [31, 16, 16, 30, 1, 1, 30],
        b'6' => [14, 16, 16, 30, 17, 17, 14],
        b'7' => [31, 1, 2, 4, 8, 8, 8],
        b'8' => [14, 17, 17, 14, 17, 17, 14],
        b'9' => [14, 17, 17, 15, 1, 1, 14],
        b'-' => [0, 0, 0, 31, 0, 0, 0],
        b'/' => [1, 2, 2, 4, 8, 8, 16],
        b':' => [0, 4, 4, 0, 4, 4, 0],
        b'+' => [0, 4, 4, 31, 4, 4, 0],
        b'.' => [0, 0, 0, 0, 0, 6, 6],
        b'=' => [0, 0, 31, 0, 31, 0, 0],
        b' ' => [0; 7],
        _ => [31, 17, 1, 2, 4, 0, 4],
    }
}
