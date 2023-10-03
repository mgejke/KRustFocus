use windres::Build;

fn main() {
    Build::new().compile("tray-resources.rc").unwrap();
}
