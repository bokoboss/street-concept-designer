use std::{env, fs, path::PathBuf};

const BUILD_ICON: &[u8] = &[
    0, 0, 1, 0, 1, 0, 1, 1, 0, 0, 1, 0, 32, 0, 48, 0, 0, 0, 22, 0, 0, 0, 40, 0, 0, 0, 1, 0, 0, 0,
    2, 0, 0, 0, 1, 0, 32, 0, 0, 0, 4, 0, 0, 0, 4, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0,
    0, 32, 96, 160, 255, 0, 0, 0, 0,
];

fn main() {
    let icon_path =
        PathBuf::from(env::var_os("OUT_DIR").expect("OUT_DIR is required")).join("icon.ico");
    fs::write(&icon_path, BUILD_ICON).expect("write build icon");
    let inline_icon_path = icon_path.to_string_lossy().replace('\\', "\\\\");
    let mut config = env::var("TAURI_CONFIG").unwrap_or_else(|_| "{}".to_owned());
    config = config.trim().to_owned();
    if config == "{}" {
        config = format!("{{\"bundle\":{{\"icon\":[\"{inline_icon_path}\"]}}}}");
    } else {
        config.pop();
        config.push_str(&format!(
            ",\"bundle\":{{\"icon\":[\"{inline_icon_path}\"]}}}}"
        ));
    }
    println!("cargo:rustc-env=TAURI_CONFIG={config}");

    let windows = tauri_build::WindowsAttributes::new().window_icon_path(&icon_path);
    tauri_build::try_build(tauri_build::Attributes::new().windows_attributes(windows))
        .expect("failed to run tauri build script");
}
