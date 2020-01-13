mod c {
    extern "C" {
        pub fn draw_pixel(x: u16, y: u16, color: u16);
        pub fn draw_background(color: u16);

        pub fn draw_square_size(x: u16, y: u16, height: u16, width: u16, color: u16);
        pub fn draw_square_coords(x1: u16, y1: u16, x2: u16, y2: u16, color: u16);

        pub fn draw_line(x1: u16, y1: u16, color: u16, thickness: u8);
        pub fn draw_triangle(x1: u16, y1: u16, x2: u16, y2: u16, x3: u16, y3: u16, color: u16, thickness: u8);

        pub fn outline_circle(xc: u16, yc: u16, x: u16, y: u16, color: u16);
        pub fn draw_circle(xc: u16, yc: u16, radius: u16, color: u16);

        pub fn draw_char(character: i8, x: u16, y: u16, color: u16);
    }
}

pub fn draw_pixel(x: u16, y: u16, color: u16) { unsafe { c::draw_pixel(x, y, color); } }
pub fn draw_background(color: u16) { unsafe { c::draw_background(color); } }

pub fn draw_char(character: u8, x: u16, y: u16, color: u16) {
    unsafe { c::draw_char(character as i8, x, y, color); }
}

pub fn draw_str(string: &str, x: u16, y: u16, color: u16) {
    let mut i: u16 = 0;
    for byte in string.bytes() {
        draw_char(byte, x + (i * 8), y, color);
        i += 1;
    }
}
