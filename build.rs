fn main() {
    // panic!("BUILD SCRIPT IS RUNNING");
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/app2.ico");
    res.compile().unwrap();
}
