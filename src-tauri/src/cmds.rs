// type CmdResult<T = ()> = Result<T, String>;

#[tauri::command]
pub fn hello(name: String) -> String {
    format!("Hello, {}!", name)
}
