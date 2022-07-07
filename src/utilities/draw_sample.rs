use std::{
    fs::File,
    io::{self, BufRead},
    path::Path,
};

pub fn sample_image(filename: &str) -> Vec<u8> {
    let path = Path::new("HDRIs/shanghai.png");
    let image = image::open(path)
        .map_err(|e| format!("Failed to read image from {:?}: {}", path, e))
        .unwrap();
    let mut image_v = image.as_bytes().to_vec();
    /*let mut image_v = vec![];
    for _ in 0..image.width()*image.height(){
        image_v.append(&mut vec![0,0,0,255]);
    }*/

    if let Ok(lines) = read_lines(filename) {
        // Consumes the iterator, returns an (Optional) String
        for line in lines {
            if let Ok(ip) = line {
                let uv: Vec<&str> = ip.split(|c| c == ',').collect();
                let index = uv[0].trim().parse::<usize>().unwrap()
                    + image.width() as usize * uv[1].trim().parse::<usize>().unwrap();
                image_v[4 * index..4 * index + 4].copy_from_slice(&[255, 0, 255, 255]);
            }
        }
    }

    image_v
}

fn read_lines<P>(filename: P) -> io::Result<io::Lines<io::BufReader<File>>>
where
    P: AsRef<Path>,
{
    let file = File::open(filename)?;
    Ok(io::BufReader::new(file).lines())
}
