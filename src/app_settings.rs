use macroquad::prelude::*;
use miniquad::conf::Icon;

fn load_img(bytes: &'static [u8]) -> Image {
    Image::from_file_with_format(bytes, Some(ImageFormat::Png)).unwrap()
}

fn populate_array(img: Image, array: &mut [u8]) {
    let mut index: usize = 0;
    for pixel in img.get_image_data() {
        for value in pixel.iter() {
            array[index] = *value;
            index += 1;
        }
    }
}

fn set_icon() -> Icon {
    let mut array_small: [u8; 1024] = [0; 1024];
    let mut array_medium: [u8; 4096] = [0; 4096];
    let mut array_big: [u8; 16384] = [0; 16384];

    populate_array(
        load_img(include_bytes!("../assets/icons/icon-16.png")),
        &mut array_small,
    );
    populate_array(
        load_img(include_bytes!("../assets/icons/icon-32.png")),
        &mut array_medium,
    );
    populate_array(
        load_img(include_bytes!("../assets/icons/icon-64.png")),
        &mut array_big,
    );

    Icon {
        small: array_small,
        medium: array_medium,
        big: array_big,
    }
}

pub fn window_conf() -> Conf {
    Conf {
        window_title: "WFC-Rails".into(),
        icon: Some(set_icon()),
        window_width: 1280,
        window_height: 720,
        ..Default::default()
    }
}

pub const ZOOM_DEFAULT: f32 = 2.0; // change this if you want different starting zoom level
pub const ZOOM_VALUE: f32 = 0.2; // change this if you want to zoom in/out faster or slower

pub fn camera_controller(camera: &mut Camera2D, zoomer: &mut f32) {
    // resize camera width and height to window width and height
    camera.zoom = vec2(*zoomer / screen_width(), *zoomer / screen_height());

    // set min window width and heigh
    if screen_width() < 320. {
        request_new_screen_size(320., screen_height());
    }

    if screen_height() < 240. {
        request_new_screen_size(screen_width(), 240.);
    }

    // zoom in
    if mouse_wheel().1 > 0. {
        // way to always get precise .2 accuracy for floating point operation
        *zoomer = (*zoomer * 10.).round() / 10.;

        *zoomer += ZOOM_VALUE;

    // zoom out
    } else if mouse_wheel().1 < 0. && *zoomer > ZOOM_VALUE {
        // way to always get precise .2 accuracy for floating point operation
        *zoomer = (*zoomer * 10.).round() / 10.;

        *zoomer -= ZOOM_VALUE;

        if *zoomer < ZOOM_VALUE {
            *zoomer = ZOOM_VALUE;
        }
    }

    // move camera
    if is_mouse_button_down(MouseButton::Right) {
        let mouse_pos = mouse_delta_position();

        // you can change the add and sub assigments if you plan to use different camera movement
        camera.target.x += mouse_pos.x * screen_width() / *zoomer;
        camera.target.y += mouse_pos.y * screen_height() / *zoomer;
    }

    // reset position and zoom
    if is_key_pressed(KeyCode::Space) {
        camera.target = Vec2::ZERO;
        *zoomer = ZOOM_DEFAULT;
    }
}
