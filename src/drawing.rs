use crate::detection::Detection;
use image::Pixel;
use image::Rgba;
use image::RgbaImage;
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};
use std::collections::{HashMap, HashSet};

pub fn draw(im: &mut RgbaImage, detection: Detection) {
    let labels: HashSet<String> = detection
        .predictions
        .iter()
        .map(|p| p.label.clone())
        .collect();

    let mut colors = HashMap::new();

    for label in labels {
        colors.entry(label).or_insert_with(|| {
            Rgba([
                rand::random::<u8>(),
                rand::random::<u8>(),
                rand::random::<u8>(),
                64u8,
            ])
        });
    }

    for prediction in detection.predictions {
        let bbox = prediction.bbox;
        let label = prediction.label;

        for x in bbox.x1..bbox.x2 + 1 {
            for y in bbox.y1..bbox.y2 + 1 {
                let pixel = im.get_pixel_mut(x as u32, y as u32);
                pixel.blend(&colors[&label]);
            }
        }

        draw_text_mut(
            im,
            colors[&label],
            bbox.x1.into(),
            (bbox.y1 - 16).into(),
            Scale::uniform(20.0),
            &Font::try_from_bytes(include_bytes!("../font/Montserrat-Regular.ttf")).unwrap(),
            label.as_str(),
        );
    }
}
