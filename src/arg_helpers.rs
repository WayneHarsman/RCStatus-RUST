pub fn get_path_as_string() -> String {
    let path = std::env::current_dir().unwrap().into_os_string().into_string().unwrap();
    path
}