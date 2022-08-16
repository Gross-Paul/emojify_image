use image::{io::Reader as ImageReader, DynamicImage, GenericImageView, RgbaImage};
use indicatif::ProgressIterator;
use std::{fs, i64::MAX};

const EMOJI_WIDTH: usize = 16;
const EMOJI_HEIGHT: usize = EMOJI_WIDTH;

fn main() {
    let img = ImageReader::open("input.jpg").unwrap().decode().unwrap();

    let img_width = img.width();
    let img_height = img.height();

    let mut img_out = RgbaImage::new(img_width, img_height);

    let mut emoji_images: Vec<DynamicImage> = vec![];

    let paths = fs::read_dir("./images/160x160/").unwrap();

    for path in paths {
        let path = path.unwrap().path();
        let result = path.display();
        let emoji_img = ImageReader::open(result.to_string())
            .expect("First")
            .decode()
            .expect(format!("{}", result).as_str());

        emoji_images.push(emoji_img);
    }

    for x in ((0..img_width).step_by(EMOJI_WIDTH)).progress() {
        for y in (0..img_height).step_by(EMOJI_HEIGHT) {
            let sub_image = img.view(
                x,
                y,
                std::cmp::min(EMOJI_WIDTH as u32, img_width - x),
                std::cmp::min(EMOJI_HEIGHT as u32, img_height - y),
            );

            let mut best_emoji_score = MAX;
            let mut best_emoji_index: usize = 0;

            for (i, emoji_img) in emoji_images.iter().enumerate() {
                let mut score: i64 = 0;

                for (x, y, sub_img_color) in sub_image.pixels() {
                    let new_emoji_color = emoji_img.get_pixel(x as u32, y as u32);
                    //let emoji_color = emoji_img.get_pixel(x as u32, y as u32);
                    let vec_sub_img = sub_img_color.0;
                    let vec_emoji_color = new_emoji_color.0;

                    for i in 0..4 {
                        let sub_img_color: i32 = vec_sub_img[i] as i32;
                        let emoji_color: i32 = vec_emoji_color[i] as i32;

                        score += sub_img_color.abs_diff(emoji_color) as i64;
                    }
                }

                if score < best_emoji_score.into() {
                    best_emoji_score = score;
                    best_emoji_index = i;
                }
            }

            for (i, j, sub_img_color) in emoji_images[best_emoji_index].pixels() {
                if i + x >= img_width || j + y >= img_height {
                    continue;
                }

                img_out.put_pixel(x as u32 + i, y as u32 + j, sub_img_color)
            }
        }
    }

    img_out.save("output.png").unwrap();
}
