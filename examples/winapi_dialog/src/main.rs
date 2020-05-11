use winapi::{
    um::winuser::*,
    um::libloaderapi::*,
    shared::basetsd::*,
    shared::windef::*,
    shared::minwindef::*,
};
use hetbuf::HetBuf;
use std::{
    ffi::OsStr,
    os::windows::ffi::OsStrExt,
    ptr,
};

fn main() {
    let mut template_buffer = HetBuf::<DWORD>::new();

    template_buffer.push_item_explicit_align(DLGTEMPLATE {
        style: WS_POPUP | WS_BORDER | WS_SYSMENU | DS_MODALFRAME | WS_CAPTION | DS_SETFONT,
        dwExtendedStyle: 0,
        cdit: 3,
        x: 10, y: 10,
        cx: 100, cy: 100,
    }, 4);
    template_buffer.push_item::<WORD>(0); // no menu
    template_buffer.push_item::<WORD>(0); // predefined dialog box class (by default)
    template_buffer.push_iter::<WORD, _>(OsStr::new("My Dialog\0").encode_wide());
    template_buffer.push_item::<WORD>(8);
    template_buffer.push_iter::<WORD, _>(OsStr::new("MS Shell Dlg\0").encode_wide());

    template_buffer.push_item_explicit_align(DLGITEMTEMPLATE {
        x: 10, y: 70,
        cx: 80, cy: 20,
        id: ID_OK,
        style: WS_CHILD | WS_VISIBLE | BS_DEFPUSHBUTTON,
        dwExtendedStyle: 0,
    }, 4);
    template_buffer.push_item::<[WORD; 2]>([0xFFFF, 0x0080]);
    template_buffer.push_iter::<WORD, _>(OsStr::new("OK\0").encode_wide());
    template_buffer.push_item::<WORD>(0); // no creation data

    template_buffer.push_item_explicit_align(DLGITEMTEMPLATE {
        x: 55, y: 10,
        cx: 40, cy: 20,
        id: ID_HELP,
        style: WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON,
        dwExtendedStyle: 0,
    }, 4);
    template_buffer.push_item::<[WORD; 2]>([0xFFFF, 0x0080]);
    template_buffer.push_iter::<WORD, _>(OsStr::new("Help\0").encode_wide());
    template_buffer.push_item::<WORD>(0); // no creation data

    template_buffer.push_item_explicit_align(DLGITEMTEMPLATE {
        x: 10, y: 10,
        cx: 40, cy: 20,
        id: ID_TEXT,
        style: WS_CHILD | WS_VISIBLE | SS_LEFT,
        dwExtendedStyle: 0,
    }, 4);
    template_buffer.push_item::<[WORD; 2]>([0xFFFF, 0x0082]);
    template_buffer.push_iter::<WORD, _>(OsStr::new("hello\0").encode_wide());
    template_buffer.push_item::<WORD>(0); // no creation data

    unsafe {
        let module_handle = GetModuleHandleW(ptr::null_mut());
        let result = DialogBoxIndirectParamW(
            module_handle,
            // template.as_ptr() as *const DLGTEMPLATE,
            template_buffer.as_slice().as_ptr() as *const DLGTEMPLATE,
            ptr::null_mut(),
            Some(dlg_proc),
            0,
        );
        println!("{}", result);
    }
}

const ID_OK: WORD = 100;
const ID_HELP: WORD = 150;
const ID_TEXT: WORD = 200;

unsafe extern "system" fn dlg_proc(
    hwnd: HWND,
    message: UINT,
    wparam: WPARAM,
    lparam: LPARAM) -> INT_PTR
{
    match message {
        WM_COMMAND => match LOWORD(wparam as _) {
            ID_OK |
            ID_HELP => {
                EndDialog(hwnd, wparam as _);
                return 1
            },
            _ => ()
        },
        _ => (),
    }
    return 0;
}
