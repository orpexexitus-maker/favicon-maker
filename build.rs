fn main() {
    let mut res = winres::WindowsResource::new();
    res.set_icon("assets/app.ico");
    res.set("ProductName", "Favicon Maker");
    res.set("FileDescription", "Favicon Maker");
    res.set("OriginalFilename", "favicon-maker.exe");
    res.compile().expect("failed to compile resources");
}
