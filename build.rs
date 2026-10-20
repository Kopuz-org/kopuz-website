// static_loader! and leptos_fluent! embed the locale files at compile time,
// so cargo has to know a translation change is a source change.
fn main() {
    println!("cargo:rerun-if-changed=locales");
}
