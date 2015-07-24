//! Clipboard WinAPI
//!
//! This crate provide simple means to operate with Windows clipboard.
//!
//! # Example:
//! ```
//! extern crate clipboard_win;
//!
//! use clipboard_win::*;
//!
//! fn main() {
//!     println!("I set some clipboard text like a boss!");
//!     set_clipboard("for my waifu!");
//! }
//! ```

extern crate winapi;
extern crate user32;
extern crate kernel32;

//WinAPI
//functions
use kernel32::{FormatMessageW};

//wrapper functions
pub mod wrapper;
use wrapper::{get_clipboard_seq_num};

#[derive(Debug, Clone)]
///Represents Windows result.
pub struct WinResult(u32);

impl WinResult {
    ///Custom errors

    ///Constructs new error.
    pub fn new(errno: u32) -> WinResult {
        WinResult(errno)
    }

    #[inline(always)]
    ///Returns ```true``` if result is ok
    pub fn is_ok(&self) -> bool {
        self.0 == 0
    }

    #[inline(always)]
    ///Returns ```true``` if result is not ok
    pub fn is_err(&self) -> bool {
        !self.is_ok()
    }

    #[inline(always)]
    ///Returns extended error code. Should be used in case if result is not ok.
    pub fn errno(&self) -> u32 {
        self.0
    }

    ///Returns description of WinAPI error code.
    pub fn errno_desc(&self) -> String {
        let mut format_buff: [u16; 300] = [0; 300];
        let num_chars: u32 = unsafe { FormatMessageW(0x00000200 | 0x00001000 | 0x00002000,
                                                     std::ptr::null(), self.0,
                                                     0, format_buff.as_mut_ptr(),
                                                     200 as u32, std::ptr::null_mut()) };

        if num_chars == 0 {
            return "Unknown error".to_string();
        }

        String::from_utf16(&format_buff).unwrap_or("Unknown error".to_string())
    }
}

impl PartialEq for WinResult {
    fn eq(&self, right: &WinResult) -> bool {
        self.0 == right.0
    }

    fn ne(&self, right: &WinResult) -> bool {
        self.0 != right.0
    }
}

///Clipboard manager provides a primitive hack for console application to handle updates of
///clipboard. It uses ```get_clipboard_seq_num``` to determines whatever clipboard is updated or
///not. Due to that this manager is a bit hacky and not exactly right way to properly work with
///clipboard. On other hand it is the best and most easy option for console application as a proper
///window is required to be created to work with clipboard.
pub struct ClipboardManager {
    delay_ms: u32,
    ok_fn: fn(&String) -> (),
    err_fn: fn(&WinResult) -> (),
}

impl ClipboardManager {
    fn default_ok(text: &String) -> () { println!("Clipboard content: {}", &text); }
    fn default_err(err_code: &WinResult) -> () { println!("Failed to get clipboard. Reason:{}", err_code.errno_desc()); }
    ///Constructs new ClipboardManager with default settings
    pub fn new() -> ClipboardManager {
        ClipboardManager {
            delay_ms: 100,
            ok_fn: ClipboardManager::default_ok,
            err_fn: ClipboardManager::default_err,
        }
    }

    ///Configure manager's delay between accessing clipboard.
    pub fn delay(&mut self, delay_ms: u32) -> &mut ClipboardManager {
        self.delay_ms = delay_ms;
        self
    }

    ///Sets callback for successfully retrieved clipboard's text.
    pub fn ok_callback(&mut self, callback: fn(&String) -> ()) -> &mut ClipboardManager
     {
        self.ok_fn = callback;
        self
    }

    ///Sets callback for failed retrieval of clipboard's text.
    ///
    ///Error code is passed from ```get_clipboard_string()```
    pub fn err_callback(&mut self, callback: fn(&WinResult) -> ()) -> &mut ClipboardManager
     {
        self.err_fn = callback;
        self
    }

    ///Starts manager loop.
    ///
    ///It's infinitely running with delay and checking whatever clipboard is updated.
    ///In case if it is updated callbacks will be called. Depending on whatever clipboard's text
    ///was retrieved or not right callback will be called.
    pub fn run(&self) -> () {
        let mut clip_num: u32 = get_clipboard_seq_num().unwrap_or_else(|| panic!("Lacks sufficient rights to access clipboard(WINSTA_ACCESSCLIPBOARD)"));
        loop {
            // It is very unlikely that we would suddenly start to lack access rights.
            // So let's just skip this iteration. Maybe it is just Windows bug... ^_^
            let new_num = get_clipboard_seq_num().unwrap_or(0);
            if new_num != 0 && clip_num != new_num {
                clip_num = new_num;
                match get_clipboard_string() {
                    Ok(clip_text) => { (self.ok_fn)(&clip_text) },
                    Err(err_code) => { (self.err_fn)(&err_code) },
                }
            println!(">>>");
            }
            std::thread::sleep_ms(self.delay_ms);
        }
    }
}

///Set clipboard with text.
pub fn set_clipboard<T: ?Sized + AsRef<std::ffi::OsStr>>(text: &T) -> WinResult {
    let result = wrapper::open_clipboard();
    if result.is_err() { return result; }

    let result = wrapper::set_clipboard(text);
    wrapper::close_clipboard();
    result
}

///Rust variant of strlen.
///
///# Parameters:
///
///* ```buff_p``` Must be valid non-NULL pointer.
#[inline(always)]
pub unsafe fn rust_strlen(buff_p: *const u16) -> usize {
    let mut idx: isize = 0;
    while *buff_p.offset(idx) != 0 { idx += 1; }
    idx as usize
}

#[inline(always)]
///Extracts clipboard content in UTF16 and convert it to String.
///
///# Return result:
///
///* ```Ok``` Content of clipboard which is stored in ```String```.
///* ```Err``` Contains ```WinResult```.
pub fn get_clipboard_string() -> Result<String, WinResult> {
    get_clipboard(13)
}

///Extracts clipboard content and convert it to String.
///
///# Parameters:
///
///* ```format``` clipboard format code.
///
///# Return result:
///
///* ```Ok``` Content of clipboard which is stored in ```String```.
///* ```Err``` Contains ```WinResult```.
pub fn get_clipboard(format: u32) -> Result<String, WinResult> {
    let result = wrapper::open_clipboard();
    if result.is_err() { return Err(result); }

    let result = wrapper::get_clipboard(format);
    wrapper::close_clipboard();
    result
}

///Extracts available clipboard formats.
///
///# Return result:
///
///* ```Ok``` Vector of available formats.
///* ```Err``` Error description.
pub fn get_clipboard_formats() -> Result<Vec<u32>, WinResult> {
    let result = wrapper::open_clipboard();
    if result.is_err() { return Err(result); }

    let result = wrapper::get_clipboard_formats();
    wrapper::close_clipboard();
    result
}

///Returns format name based on it's code.
///
///# Note:
///It is not possible to retrieve name of predefined clipboard format.
///
///# Return result:
///
///* ```Some``` String which contains the format's name.
///* ```None``` If format name is incorrect or predefined.
pub fn get_format_name(format: u32) -> Option<String> {
    let result = wrapper::open_clipboard();
    if result.is_err() { return None; }

    let result = wrapper::get_format_name(format);
    wrapper::close_clipboard();
    result
}