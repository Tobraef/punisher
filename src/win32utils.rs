type Hwnd = *const std::ffi::c_void;
type Dword = u64;
type WinEventCallback = extern "C" fn(i32, Dword, Hwnd, i64, i64, Dword, Dword);

const EVENT_SYSTEM_FOREGROUND: Dword = 3;
const NULL: Hwnd = std::ptr::null();

fn default_window_change(_ :&str) {}
static mut G_WINDOW_CHANGE_CALLBACK: fn(&str) = default_window_change;
extern "C" fn window_change(_hook: i32, _event: Dword, handle: Hwnd, _id_o: i64, _id_c: i64, _id_e: Dword, _e_t: Dword) {
    println!("Called");
    let name = process_name_from_hwnd(handle).unwrap();
    unsafe { G_WINDOW_CHANGE_CALLBACK(&name); }
}

#[link(name = "user32")]
extern "system" {
    fn SetWinEventHook(
        eventMin :Dword, 
        eventMax :Dword,
        hmodWinEventProc :Hwnd, 
        pfnWinEventProc :WinEventCallback, 
        idProcess :Dword, 
        idThread :Dword, 
        dwFlags :Dword) -> Dword;
    fn GetWindowThreadProcessId(handle: Hwnd, lpdwProcessId: *const Dword) -> Dword;
    fn GetForegroundWindow() -> Hwnd;
    fn MessageBoxA(handle: Hwnd, text: *const u8, caption: *const u8, options: u32) -> i32;
}

#[deprecated(note = "Cannot subscribe with given function")]
pub fn subscribe_for_window_change<'a>(callback: fn(&str)) {
    unsafe { G_WINDOW_CHANGE_CALLBACK = callback; };
    let result = unsafe { SetWinEventHook(EVENT_SYSTEM_FOREGROUND, EVENT_SYSTEM_FOREGROUND, 
        NULL, window_change, 0, 0, 2) };
    println!("Received result {}", result); 
}

pub fn does_process_exist(process_name: &str) -> bool {
    true //panic!("not yet implemented")
}

fn process_name_from_hwnd(handle: Hwnd) -> Option<String> {
    let mut process_id: Dword = 0;
    let process_id_buffer = &process_id;
    let _ = unsafe { GetWindowThreadProcessId(handle, process_id_buffer) };
    if process_id == 0 { return None }
    let pid_arg = format!("{} {}", "pid eq", process_id);
    let output = std::process::Command::new("tasklist")
        .args(&["/fi", &pid_arg])
        .output()
        .expect("failed to execute process");
    if !output.stderr.is_empty() { 
        let message = String::from_utf8_lossy(&output.stderr);
        println!("{}", message);
        return None
    }
    let output = String::from_utf8_lossy(&output.stdout);
    let output_lines: Vec<&str> = output.split_terminator('\n').collect();
    if output_lines.is_empty() { return None }
    output_lines
        .last()
        .unwrap()
        .split_terminator(' ')
        .next()
        .map(|name| String::from(name))
}

pub fn current_process_name() -> Option<String> {
    let handle = unsafe { GetForegroundWindow() };
    process_name_from_hwnd(handle)
}

pub fn display_yes_no_window(caption: &str, text: &str) -> bool {
    let hwnd = unsafe { GetForegroundWindow() };
    let caption_ptr = caption.as_ptr();
    let text_ptr = text.as_ptr();
    let result = unsafe { MessageBoxA(hwnd, text_ptr, caption_ptr, 4) };
    result == 6
}

pub fn display_window(caption: &str, text: &str) {
    let hwnd = unsafe { GetForegroundWindow() };
    let caption_ptr = caption.as_ptr();
    let text_ptr = text.as_ptr();
    let _ = unsafe { MessageBoxA(hwnd, text_ptr, caption_ptr, 0) };
}