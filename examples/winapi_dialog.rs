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

    let mut heads = vec![];

    macro_rules! h {
        () => {heads.push((template_buffer.as_slice().len(), line!()))};
    }

    println!("{:?}", std::mem::size_of::<DLGITEMTEMPLATE>());
    h!(); template_buffer.push_item_explicit_align(DLGTEMPLATE {
        style: WS_POPUP | WS_BORDER | WS_SYSMENU | DS_MODALFRAME | WS_CAPTION,
        dwExtendedStyle: 0,
        cdit: 3,
        x: 10, y: 10,
        cx: 100, cy: 100,
    }, 4);
    h!(); template_buffer.push_item::<WORD>(0); // no menu
    h!(); template_buffer.push_item::<WORD>(0); // predefined dialog box class (by default)
    h!(); template_buffer.push_iter::<WORD, _>(OsStr::new("My Dialog\0").encode_wide());

    h!(); template_buffer.push_item_explicit_align(DLGITEMTEMPLATE {
        x: 10, y: 70,
        cx: 80, cy: 20,
        id: ID_OK,
        style: WS_CHILD | WS_VISIBLE | BS_DEFPUSHBUTTON,
        dwExtendedStyle: 0,
    }, 4);
    h!(); template_buffer.push_item::<[WORD; 2]>([0xFFFF, 0x0080]);
    h!(); template_buffer.push_iter::<WORD, _>(OsStr::new("OK\0").encode_wide());
    h!(); template_buffer.push_item::<WORD>(0); // no creation data

    h!(); template_buffer.push_item_explicit_align(DLGITEMTEMPLATE {
        x: 55, y: 10,
        cx: 40, cy: 20,
        id: ID_HELP,
        style: WS_CHILD | WS_VISIBLE | BS_PUSHBUTTON,
        dwExtendedStyle: 0,
    }, 4);
    h!(); template_buffer.push_item::<[WORD; 2]>([0xFFFF, 0x0080]);
    h!(); template_buffer.push_iter::<WORD, _>(OsStr::new("Help\0").encode_wide());
    h!(); template_buffer.push_item::<WORD>(0); // no creation data

    h!(); template_buffer.push_item_explicit_align(DLGITEMTEMPLATE {
        x: 10, y: 10,
        cx: 40, cy: 20,
        id: ID_TEXT,
        style: WS_CHILD | WS_VISIBLE | SS_LEFT,
        dwExtendedStyle: 0,
    }, 4);
    h!(); template_buffer.push_item::<[WORD; 2]>([0xFFFF, 0x0082]);
    h!(); template_buffer.push_iter::<WORD, _>(OsStr::new("hello\0").encode_wide());
    h!(); template_buffer.push_item::<WORD>(0); // no creation data

    println!("{:?}", heads);

    let template: &[u8] = &[
        0x80, 0x00, 0xC8, 0x80, 0x00, 0x00, 0x00, 0x00, 0x03, 0x00, 0x0A, 0x00, 0x0A, 0x00, 0x64, 0x00, 0x64, 0x00, 0x00, 0x00, 0x00, 0x00, 0x4D, 0x00, 0x79, 0x00, 0x20, 0x00, 0x44, 0x00, 0x69, 0x00,
        0x61, 0x00, 0x6C, 0x00, 0x6F, 0x00, 0x67, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01, 0x00, 0x00, 0x50, 0x00, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x46, 0x00, 0x50, 0x00, 0x14, 0x00, 0x01, 0x00, 0xFF, 0xFF,
        0x80, 0x00, 0x4F, 0x00, 0x4B, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x50, 0x00, 0x00, 0x00, 0x00, 0x37, 0x00, 0x0A, 0x00, 0x28, 0x00, 0x14, 0x00, 0x96, 0x00, 0xFF, 0xFF,
        0x80, 0x00, 0x48, 0x00, 0x65, 0x00, 0x6C, 0x00, 0x70, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x50, 0x00, 0x00, 0x00, 0x00, 0x0A, 0x00, 0x0A, 0x00, 0x28, 0x00, 0x14, 0x00,
        0xC8, 0x00, 0xFF, 0xFF, 0x82, 0x00, 0x68, 0x00, 0x65, 0x00, 0x6C, 0x00, 0x6C, 0x00, 0x6F, 0x00, 0x00,
    ];

    let mut iter = template.iter().cloned();
    let mut gen_iter = template_buffer.as_slice().iter().map(|m| unsafe{ *m.as_ptr() });
    let mut i = 0;
    loop {
        let a = iter.next();
        let b = gen_iter.next();

        let mut head_iter = heads.iter().cloned();
        let head = loop {
            let (head, line) = match head_iter.next() {
                Some(h) => h,
                None => break None,
            };
            if head == i {
                break Some(line);
            }
        };

        print!("{:3} |", i);

        match head {
            Some(line) => print!("{:3} ", line),
            None       => print!("    "),
        }

        match a {
            Some(i) => print!("{:2x} ", i),
            None => print!("__ "),
        }
        match b {
            Some(i) => print!("{:2x} ", i),
            None => print!("__ "),
        }

        if a != b {
            print!("<<< DIFF");
        }
        println!();

        if a.is_none() && b.is_none() {
            break;
        }
        i += 1;
    }

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
