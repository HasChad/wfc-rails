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
