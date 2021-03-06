//!Provides direct wrappers to WinAPI functions.
//!
//!These functions omit calls to ```OpenClipboard``` and ```CloseClipboard```.
//!Due to that most functions have requirements for them to be called.

extern crate user32;
extern crate kernel32;

use winapi::minwindef::{HGLOBAL, UINT};
use winapi::winnt::HANDLE;
use winapi::basetsd::SIZE_T;

use kernel32::{GlobalFree, GlobalAlloc, GlobalLock, GlobalUnlock, GetLastError};
use user32::{RegisterClipboardFormatW, CountClipboardFormats, IsClipboardFormatAvailable, GetClipboardFormatNameW, EnumClipboardFormats, GetClipboardSequenceNumber, SetClipboardData, EmptyClipboard, OpenClipboard, GetClipboardData, CloseClipboard};

use std;
use std::os::windows::ffi::OsStrExt;

use super::WindowsError;
use super::clipboard_formats::*;

///Wrapper around ```GetLastError```.
///
///# Return result:
///
///* ```WindowsError``` with last windows error
#[inline]
pub fn get_last_error() -> WindowsError {
    WindowsError(unsafe { GetLastError() })
}

#[inline]
///Generic strlen
///
///# Note:
///
///```Default``` is used instead of ```Zero```
unsafe fn rust_strlen<T: Default + PartialEq>(buff_p: *const T) -> usize {
    let zero = T::default();
    let mut idx: isize = 0;

    while *buff_p.offset(idx) != zero {
        idx += 1;
    }

    idx as usize
}

#[inline]
///Wrapper around ```GetClipboardSequenceNumber```.
///
///# Return result:
///
///* ```Some``` Contains return value of ```GetClipboardSequenceNumber```.
///* ```None``` In case if you do not have access. It means that zero is returned by system.
pub fn get_clipboard_seq_num() -> Option<u32> {
    let result: u32 = unsafe { GetClipboardSequenceNumber() };

    if result == 0 {
        return None;
    }

    Some(result)
}

#[inline]
///Wrapper around ```OpenClipboard```.
///
///This function MUST be called only once until the clipboard is closed again with
///```close_clipboard```.
pub fn open_clipboard() -> Result<(), WindowsError> {
    unsafe {
        if OpenClipboard(std::ptr::null_mut()) == 0 {
            return Err(get_last_error());
        }
    }

    Ok(())
}

#[inline]
///Wrapper around ```CloseClipboard```.
///
///This function MUST be called after a successful call of ```open_clipboard```.
pub fn close_clipboard() -> Result<(), WindowsError> {
    unsafe {
        if CloseClipboard() == 0 {
            return Err(get_last_error());
        }
    }

    Ok(())
}

#[inline]
///Wrapper around ```EmptyClipboard```.
///
///This function MUST be called after a successful call of ```open_clipboard```.
pub fn empty_clipboard() -> Result<(), WindowsError> {
    unsafe {
        if EmptyClipboard() == 0 {
            return Err(get_last_error());
        }
    }

    Ok(())
}

///Wrapper around ```SetClipboardData``` to set unicode text.
///
///This function MUST be called after a successful call of ```open_clipboard```.
pub fn set_clipboard<T: ?Sized + AsRef<std::ffi::OsStr>>(text: &T) -> Result<(), WindowsError> {
    const FLAG_GHND: UINT = 66;
    let text = text.as_ref();
    unsafe {
        //allocate buffer and copy string to it.
        let utf16_buff: Vec<u16> = text.encode_wide().collect();
        let len: usize = (utf16_buff.len()+1) * 2;
        let handler: HGLOBAL = GlobalAlloc(FLAG_GHND, len as SIZE_T);
        if handler.is_null() {
            return Err(get_last_error());
        }
        else {
            let lock = GlobalLock(handler) as *mut u16;

            let len: usize = (len - 1) / 2;
                                          //src,               dest, len
            std::ptr::copy_nonoverlapping(utf16_buff.as_ptr(), lock, len);
            drop(utf16_buff);
            *lock.offset(len as isize) = 0;

            GlobalUnlock(handler);

            //Set new clipboard text.
            EmptyClipboard();
            if SetClipboardData(CF_UNICODETEXT, handler).is_null() {
                let result = Err(get_last_error());
                GlobalFree(handler);
                return result;
            }
        }
    }
    Ok(())
}

///Wrapper around ```SetClipboardData``` to set raw data.
///
///# Parameters:
///
///* ```data``` buffer with raw data to be copied.
///* ```format``` clipboard format code.
///
///This function MUST be called after a successful call of ```open_clipboard```.
pub fn set_clipboard_raw(data: &[u8], format: u32) -> Result<(), WindowsError> {
    const FLAG_GHND: UINT = 66;
    unsafe {
        //allocate buffer and copy string to it.
        let len: usize = data.len() + 1;
        let handler: HGLOBAL = GlobalAlloc(FLAG_GHND, len as SIZE_T);

        if handler.is_null() {
            return Err(get_last_error());
        }
        else {
            let lock = GlobalLock(handler) as *mut u8;

            let len: usize = len - 1;
                                          //src,         dest, len
            std::ptr::copy_nonoverlapping(data.as_ptr(), lock, len);
            *lock.offset(len as isize) = 0;

            GlobalUnlock(handler);

            //Set new clipboard text.
            EmptyClipboard();
            if SetClipboardData(format, handler).is_null() {
                let result = Err(get_last_error());
                GlobalFree(handler);
                return result;
            }
        }
    }
    Ok(())
}

///Wrapper around ```GetClipboardData``` with hardcoded UTF16 format.
///
///This function MUST be called after a successful call of ```open_clipboard```.
///
///# Return result:
///
///* ```Ok``` Content of clipboard which is stored in ```String```.
///* ```Err``` Contains ```WindowsError```.
pub fn get_clipboard_string() -> Result<String, WindowsError> {
    let result: Result<String, WindowsError>;
    unsafe {
        let text_handler: HANDLE = GetClipboardData(CF_UNICODETEXT as u32);

        if text_handler.is_null() {
            result = Err(get_last_error());
        }
        else {
            let text_p = GlobalLock(text_handler) as *const u16;
            let text_s = std::slice::from_raw_parts(text_p, rust_strlen(text_p));

            result = Ok(String::from_utf16_lossy(text_s));
            GlobalUnlock(text_handler);
        }
    }

    result
}

///Wrapper around ```GetClipboardData```.
///
///This function MUST be called after a successful call of ```open_clipboard```.
///
///# Parameters:
///
///* ```format``` clipboard format code.
///
///# Return result:
///
///* ```Ok``` Contains buffer with raw data.
///* ```Err``` Contains ```WindowsError```.
pub fn get_clipboard(format: u32) -> Result<Vec<u8>, WindowsError> {
    let result: Result<Vec<u8>, WindowsError>;
    unsafe {
        let text_handler: HANDLE = GetClipboardData(format as UINT);

        if text_handler.is_null() {
            result = Err(get_last_error());
        }
        else {
            let text_p = GlobalLock(text_handler) as *const u8;
            let text_vec: Vec<u8> = std::slice::from_raw_parts(text_p, rust_strlen(text_p)).to_vec();

            result = Ok(text_vec);
            GlobalUnlock(text_handler);
        }
    }

    result
}

///Extracts available clipboard formats.
///
///# Return result:
///
///* ```Ok``` Vector of available formats.
///* ```Err``` Contains ```WindowsError```.
pub fn get_clipboard_formats() -> Result<Vec<u32>, WindowsError> {
    let mut result: Vec<u32> = vec![];
    unsafe {
        let mut clip_format: u32 = EnumClipboardFormats(0);

        while clip_format != 0 {
            result.push(clip_format);
            clip_format = EnumClipboardFormats(clip_format);
        }

        let error = GetLastError();

        if error != 0 {
            return Err(WindowsError(error));
        }
    }

    Ok(result)
}

///Returns format name based on it's code.
///
///# Note:
///It is not possible to retrieve name of predefined clipboard format.
///
///# Parameters:
///
///* ```format``` clipboard format code.
///
///# Return result:
///
///* ```Some``` String which contains the format's name.
///* ```None``` If format name is incorrect or predefined.
pub fn get_format_name(format: u32) -> Option<String> {
    let format_buff: [u16; 30] = [0; 30];
    unsafe {
        let buff_p = format_buff.as_ptr() as *mut u16;

        if GetClipboardFormatNameW(format, buff_p, 30) == 0 {
            return None;
        }

    }

    Some(String::from_utf16_lossy(&format_buff))
}

#[inline(always)]
///Determines whenever provided clipboard format is available on clipboard or not.
///
///# Return result:
///
///* ```true``` Such format exists.
///* ```false``` No such format.
pub fn is_format_avail(format: u32) -> bool {
    unsafe { IsClipboardFormatAvailable(format) != 0 }
}

#[inline]
///Retrieves number of currently available formats on clipboard.
///
///# Return result:
///
///* ```Ok``` Contains number of formats.
///* ```Err``` Contains ```WindowsError```.
pub fn count_formats() -> Result<i32, WindowsError> {
    let result = unsafe { CountClipboardFormats() };

    if result == 0 {
        Err(get_last_error())
    }
    else {
        Ok(result)
    }
}

///Registers a new clipboard format with specified name.
///
///# Return result:
///
///* ```Ok``` Contains a ```u32``` number of newly created format.
///* ```Err``` Contains ```WindowsError```.
pub fn register_format<T: ?Sized + AsRef<std::ffi::OsStr>>(text: &T) -> Result<u32, WindowsError> {
    let mut utf16_buff: Vec<u16> = text.as_ref().encode_wide().collect();
    utf16_buff.push(0);

    let result = unsafe { RegisterClipboardFormatW(utf16_buff.as_ptr()) };

    if result == 0 {
        Err(get_last_error())
    }
    else {
        Ok(result)
    }
}
