mod args;
use args::Args;
use std::vec;
use image::{GenericImageView, DynamicImage, ImageFormat, io::Reader, imageops::FilterType::Triangle };



fn main() -> Result<(), ImageDataErrors> {
    let args: Args = Args::new();

    let (image_1, image_1_format) = find_image_from_path(args.image_one);
    let (image_2, image_2_format) = find_image_from_path(args.image_two);

    if image_1_format != image_2_format {
        return Err(ImageDataErrors::DifferentImageFormats);
    } 

    let (image_1, image_2) = standardise_size(image_1, image_2);
    let mut output = FloatingImage::new(image_1.width(), image_1.height(), args.image_out);

    let combine_data = combine_images(image_1, image_2);
    output.set_data(combine_data)?;

    image::save_buffer_with_format(
        output.name,
        &output.data,
        output.width,
        output.height,
        image::ColorType::Rgba8,
        image_1_format
    ).unwrap(); 

    Ok(())
}


#[derive(Debug)]
enum ImageDataErrors {
    DifferentImageFormats,
    BufferTooSmall,
}


struct FloatingImage {
    width: u32,
    height: u32,
    data: Vec<u8>,
    name: String,
}


impl FloatingImage {
    fn new(width: u32, height: u32, name: String) -> Self {
        let buffer_capacity = width * height * 4;
        let buffer = Vec::with_capacity(buffer_capacity.try_into().unwrap());
        FloatingImage { width, height, data: buffer, name }
    }

    fn set_data (&mut self, data: Vec<u8>) -> Result<(), ImageDataErrors> {
        if data.len() > self.data.capacity()  {
            return Err(ImageDataErrors::BufferTooSmall);
        } self.data = data;
        Ok(())
    }
}


fn find_image_from_path(path: String) -> (DynamicImage, ImageFormat) {
    let image_reader = Reader::open(path).unwrap();
    let image_format = image_reader.format().unwrap();
    let image = image_reader.decode().unwrap();

    (image, image_format)
}


fn get_smallest_dimension (dimension_1: (u32,u32), dimension_2: (u32, u32)) -> (u32, u32) {
    let pix_1: u32 = dimension_1.0 * dimension_1.1;
    let pix_2: u32 = dimension_2.0 * dimension_2.1;

    return if pix_1 > pix_2 { dimension_1 } else { dimension_2 }
}


fn standardise_size (image_1: DynamicImage, image_2: DynamicImage) -> (DynamicImage, DynamicImage) {
    let (width, height) = get_smallest_dimension(image_1.dimensions(), image_2.dimensions());

    if image_2.dimensions() == (width, height) {
        (image_1.resize_exact(width, height, Triangle), image_2)
    } else {
        (image_1, image_2.resize_exact(width, height, Triangle))
    }

}


fn combine_images (image_1: DynamicImage, image_2: DynamicImage) -> Vec<u8> {
    let vec_1 = image_1.to_rgb8().into_vec();
    let vec_2 = image_2.to_rgb8().into_vec();

    alternate_pixels(vec_1,vec_2)
}


fn alternate_pixels(vec_1: Vec<u8>, vec_2: Vec<u8>) -> Vec<u8>{
    let mut data = vec![0u8; vec_1.len()];
    let mut pixel = 0;

    while pixel < vec_1.len() {
        if pixel % 8 == 0 {
            data.splice(pixel..=pixel + 3, set_rgba(&vec_1, pixel, pixel + 3));
        } else {
            data.splice(pixel..=pixel + 3, set_rgba(&vec_2, pixel, pixel + 3));
        }
        pixel += 4;
    }
    data
}


fn set_rgba(vec: &Vec<u8>, start: usize, end: usize) -> Vec<u8> {

    let mut rgba: Vec<u8> = Vec::new();

    for i in start..=end {
        let val =  match vec.get(i) {
            Some(d) => *d,
            None => panic!("OUT OF BOUNDS!"),
        };
        rgba.push(val);
    };
    rgba    
}