fn main() {
    slint_build::compile("ui/main_window.slint").expect("failed to compile Slint UI");

    #[cfg(target_os = "windows")]
    winresource::WindowsResource::new()
        .set_icon("assets/calculator.ico")
        .compile()
        .expect("failed to embed Windows application icon");
}
