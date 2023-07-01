use image::DynamicImage;


fn main() {
    // Challenge: If you're feeling really ambitious, you could delete this code
    // and use the "clap" library instead: https://docs.rs/clap/2.32.0/clap/
    let mut args: Vec<String> = std::env::args().skip(1).collect(); // std::env::args() is a function provided by STL to real CL arguments
    if args.is_empty() {
        println!("No arguments!!!");
        print_usage_and_exit();
    }
    let mut blur_amt: f32 = 2.0; // default blur value
    let subcommand = args.remove(0); // args.remove(0) is used to retrieve and remove the first element from the args vector.
    match subcommand.as_str() {
        "blur" => {
            if args.len() < 2 {
                print_usage_and_exit();
            }
            // Taking an image as input, processing it and returning it as output
            let infile = args.remove(0);
            let outfile = args.remove(0);
            
            if !args.is_empty() {
                blur_amt = args.remove(0).parse().expect("Invalid blur amount");
            }

            blur(infile, outfile, blur_amt)
        }

        "brighten" => {
            if args.len() < 2 {
                print_usage_and_exit();
            }

            let infile = args.remove(0);
            let outfile = args.remove(0);
            let value = args.remove(0);

            brighten(infile, outfile, value.parse().unwrap());
        }

        "crop" => {
            if args.len() < 2 {
                print_usage_and_exit();
            }

            let infile = args.remove(0);
            let outfile = args.remove(0);
            let x: u32 = args.remove(0).parse().unwrap();
            let y: u32 = args.remove(0).parse().unwrap();
            let width: u32 = args.remove(0).parse().unwrap();
            let height: u32 = args.remove(0).parse().unwrap();

            crop(infile, outfile, x, y, width, height);
        }
        
        "rotate" => {
            if args.len() < 2 {
                print_usage_and_exit();
            }

            let infile = args.remove(0);
            let outfile = args.remove(0);
            let rotation_value: u32 = args.remove(0).parse().unwrap();

            rotate(infile, outfile, rotation_value);
        }

        "invert" => {
            if args.len() < 2 {
                print_usage_and_exit();
            }

            let infile = args.remove(0);
            let outfile = args.remove(0);
            
            invert(infile, outfile);
        }
        
        "grayscale" => {
            if args.len() < 2 {
                print_usage_and_exit();
            }

            let infile = args.remove(0);
            let outfile = args.remove(0);
            
            grayscale(infile, outfile);
        }

        // A VERY DIFFERENT EXAMPLE...a really fun one. :-)
        "fractal" => {
            if args.len() != 1 {
                print_usage_and_exit();
            }
            let outfile = args.remove(0);
            fractal(outfile);
        }

        // **OPTION**
        // Generate -- see the generate() function below -- this should be sort of like "fractal()"!

        // For everything else...
        _ => {
            print_usage_and_exit();
        }
    }
}

fn print_usage_and_exit() {
    println!("USAGE (when in doubt, use a .png extension on your filenames)");
    println!("blur INFILE OUTFILE");
    println!("fractal OUTFILE");
    // **OPTION**
    // Print useful information about what subcommands and arguments you can use
    // println!("...");
    std::process::exit(-1);
}

fn blur(_infile: String, _outfile: String, _blur_amt: f32) {
    let img = image::open(_infile).expect("Failed to open INFILE.");
    let img2 = img.blur(_blur_amt);
  
    img2.save(_outfile).expect("Failed writing OUTFILE.");
}

fn brighten(_infile: String, _outfile: String, _value: i32) {
    let img = image::open(_infile).expect("Couldn't open image");
    let img2 = img.brighten(_value);

    img2.save(_outfile).expect("Couldn't save image");
}

fn crop(_infile: String, _outfile: String, _x: u32, _y: u32, _width: u32, _height: u32) {
    let mut img = image::open(_infile).expect("Can't open image");
    let img2 = img.crop(_x, _y, _width, _height);

    img2.save(_outfile).unwrap();
}

fn rotate(_infile: String, _outfile: String, _rotation_value: u32) {
    let img = image::open(_infile).expect("Can't open image");
    let mut img2: DynamicImage;

    if _rotation_value == 90 {
       img2 = img.rotate90();
    } else if _rotation_value == 180 {
        img2 = img.rotate180();
    } else if _rotation_value == 270 {
        img2 = img.rotate270();
    } else {
        println!("Invalid rotation value ( choose either 90 or 180 or 270 )");
        std::process::exit(1);
    }

    img2.save(_outfile).expect("Couldn't save image");
}

fn invert(_infile: String, _outfile: String) {
    let mut img = image::open(_infile).expect("Can't open image");

    img.invert();
    img.save(_outfile).unwrap();
}

fn grayscale(_infile: String, _outfile: String) {
    // See blur() for an example of how to open an image.
    let mut img = image::open(_infile).expect("Can't open image");

    img.grayscale();

    img.save(_outfile);
}

fn generate(outfile: String) {
    // Create an ImageBuffer -- see fractal() for an example

    // Iterate over the coordinates and pixels of the image -- see fractal() for an example

    // Set the image to some solid color. -- see fractal() for an example

    // Challenge: parse some color data from the command-line, pass it through
    // to this function to use for the solid color.

    // Challenge 2: Generate something more interesting!

    // See blur() for an example of how to save the image
}

// This code was adapted from https://github.com/PistonDevelopers/image
fn fractal(_outfile: String) {
    let width = 800;
    let height = 800;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    let scale_x = 3.0 / width as f32;
    let scale_y = 3.0 / height as f32;

    // Iterate over the coordinates and pixels of the image
    for (x, y, pixel) in imgbuf.enumerate_pixels_mut() {
        // Use red and blue to be a pretty gradient background
        let red = (0.3 * x as f32) as u8;
        let blue = (0.3 * y as f32) as u8;

        // Use green as the fractal foreground (here is the fractal math part)
        let cx = y as f32 * scale_x - 1.5;
        let cy = x as f32 * scale_y - 1.5;

        let c = num_complex::Complex::new(-0.4, 0.6);
        let mut z = num_complex::Complex::new(cx, cy);

        let mut green = 0;
        while green < 255 && z.norm() <= 2.0 {
            z = z * z + c;
            green += 1;
        }

        // Actually set the pixel. red, green, and blue are u8 values!
        *pixel = image::Rgb([red, green, blue]);
    }

    imgbuf.save(_outfile).unwrap();
}

// **SUPER CHALLENGE FOR LATER** - Let's face it, you don't have time for this during class.
//
// Make all of the subcommands stackable!
//
// For example, if you run:
//
//   cargo run infile.png outfile.png blur 2.5 invert rotate 180 brighten 10
//
// ...then your program would:
// - read infile.png
// - apply a blur of 2.5
// - invert the colors
// - rotate the image 180 degrees clockwise
// - brighten the image by 10
// - and write the result to outfile.png
//
// Good luck!