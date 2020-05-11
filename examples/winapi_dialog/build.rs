use std::{
    fs,
    path::Path,
};
fn main() {
    let out_dir = std::env::var("OUT_DIR").unwrap();
    let out_dir = Path::new(&out_dir);
    let rc_path = out_dir.join("winapi_dialog.rc");
    let manifest_path = Path::new("./resources/manifest.xml");

    let rc_str = format!(r#"
        #pragma code_page(65001)
        1 24 "{}"
    "#, manifest_path.display());

    fs::write(&rc_path, rc_str).unwrap();

    embed_resource::compile(&rc_path);
}
