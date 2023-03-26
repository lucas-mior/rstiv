struct Options {
    w: u32,
    h: u32,
    H: i32,
    x: u32,
    y: u32,
    preview: bool,
    clear: bool,
    print_dim: bool,
}

struct Image {
    filename: Option<String>,
    path: Option<String>,
    w: u32,
    h: u32,
}

const HEIGHT_SHELL: u32 = 11;

fn main() {
    let mut opt = Options {
        w: 100,
        h: HEIGHT_SHELL,
        H: -1,
        x: 0,
        y: 1,
        preview: true,
        clear: false,
        print_dim: true,
    };

    let mut img = Image {
        filename: None,
        path: None,
        w: 0,
        h: 0,
    };

    img.filename = Some(std::env::args().nth(1).expect("No image filename provided"));

    // Rest of the program...
}
