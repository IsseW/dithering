use clap::Parser;

#[derive(Parser, Debug)]
#[clap(author, version, about="Generate a dithering texture atlas")]
struct Args {
    #[clap(short, long, value_parser, default_value_t = String::from("atlas.png"))]
    output: String,
    
    #[clap(short, long, value_parser, default_value_t = 8)]
    size: u32,
    #[clap(short, long, value_parser, default_value_t = 8)]
    tiles: u32,
}

fn main() {
    let args = Args::parse();

    let mut img = image::ImageBuffer::new(args.size * args.tiles, args.size);


    for i in 0..args.tiles {
        let t = i as f32 / (args.tiles - 1) as f32;

        let mut x = 0.0;
        let mut y = 0;
        loop {
            x += 1.0 / t;
            y += x as u32 / args.size;
            x = x % args.size as f32;
            if y >= args.size {
                break;
            }
            *img.get_pixel_mut(i * args.size + x as u32, y) = image::Rgb::<u8>([0xFF, 0xFF, 0xFF]);
        }
    }
    
    img.save(args.output).unwrap();
}
