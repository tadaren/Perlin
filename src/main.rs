use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand::rngs::StdRng;
use image::{ImageBuffer, Rgb};

struct Perlin {
    permutation: Vec<usize>,
}

impl Perlin {
    pub fn new(seed: [u8; 32]) -> Perlin {
        // println!("seed");
        let p = Perlin::generate_random_permutation(seed);
        let permutation = [&p[..], &p[..]].concat();
        // println!("{:?}", permutation);
        Perlin{ permutation}
    }

    fn fade(t: f64) -> f64 {
        return ((6.0 * t - 15.0) * t + 10.0) * t * t * t
    }

    fn lerp(a: f64, b: f64, x: f64) -> f64 {
        return a + x * (b - a)
    }

    fn grad(hash: usize, x: f64, y: f64, z: f64) -> f64 {
        return match hash & 0xF {
            0x0 =>  x + y,
            0x1 => -x + y,
            0x2 =>  x - y,
            0x3 => -x - y,
            0x4 =>  x + z,
            0x5 => -x + z,
            0x6 =>  x - z,
            0x7 => -x - z,
            0x8 =>  y + z,
            0x9 => -y + z,
            0xa =>  y - z,
            0xb => -y - z,
            0xc =>  y + x,
            0xd => -y + z,
            0xe =>  y - x,
            0xf => -y - z,
            _ => 0.0
        }
    }

    pub fn get(&self, x: f64, y: f64, z: f64) -> f64 {
        // println!("{} {} {}", x, y, z);
        let xf = x - x.floor();
        let yf = y - y.floor();
        let zf = z - z.floor();
        // println!("{} {} {}", xf, yf, zf);
        let xi = (x as i64 & 255) as usize;
        let yi = (y as i64 & 255) as usize;
        let zi = (z as i64 & 255) as usize;
        // println!("{} {} {}", xi, yi, zi);

        let u = Perlin::fade(xf);
        let v = Perlin::fade(yf);
        let w = Perlin::fade(zf);
        // println!("{} {} {}", u, v, w);

        let p = &self.permutation;
        let aaa = p[p[p[xi  ]+yi  ]+zi  ];
        let aab = p[p[p[xi  ]+yi  ]+zi+1];
        let aba = p[p[p[xi  ]+yi+1]+zi  ];
        let abb = p[p[p[xi  ]+yi+1]+zi+1];
        let baa = p[p[p[xi+1]+yi  ]+zi  ];
        let bab = p[p[p[xi+1]+yi  ]+zi+1];
        let bba = p[p[p[xi+1]+yi+1]+zi  ];
        let bbb = p[p[p[xi+1]+yi+1]+zi+1];

        let x1 = Perlin::lerp(Perlin::grad(aaa, xf, yf, zf), Perlin::grad(baa, xf-1.0, yf, zf), u);
        let x2 = Perlin::lerp(Perlin::grad(aba, xf, yf-1.0, zf), Perlin::grad(bba, xf-1.0, yf-1.0, zf), u);
        let y1 = Perlin::lerp(x1, x2, v);
        let x1 = Perlin::lerp(Perlin::grad(aab, xf, yf, zf-1.0), Perlin::grad(bab, xf-1.0, yf, zf-1.0), u);
        let x2 = Perlin::lerp(Perlin::grad(abb, xf, yf-1.0, zf-1.0), Perlin::grad(bbb, xf-1.0, yf-1.0, zf-1.0), u);
        let y2 = Perlin::lerp(x1, x2, v);
        let z = Perlin::lerp(y1, y2, w);
        return z
    }

    pub fn noise(&self, x: f64, y: f64, z: f64) -> f64 {
        // println!("{} {} {} {:?}", x, y, z, self.get(x, y, z));
        return (self.get(x, y, z) + 1.0) / 2.0
    }

    pub fn octave_noise(&self, octaves: u8, x: f64, y: f64, z: f64) -> f64 {
        let mut value = 0.0;
        let mut amp = 1.0;

        for _ in 0..octaves {
            value += amp * self.get(x / amp, y / amp, z / amp);
            amp *= 0.5;
        }
        return (value + 1.0) / 2.0
    }

    fn generate_random_permutation(seed: [u8; 32]) -> Vec<usize> {
        let mut p: Vec<usize> = (0..256).collect();
        // let mut rng: StdRng = SeedableRng::from_seed(seed);
        let mut rng = rand::thread_rng();
        p.shuffle(&mut rng);
        // println!("{:?}", p);
        return p;
    }
}

fn color(value: u8) -> Rgb<u8> {
    match value {
        0 => Rgb([0, 6, 180]),
        1 => Rgb([0, 6, 180]),
        2 => Rgb([0, 66, 210]),
        3 => Rgb([0, 185, 255]),
        4 => Rgb([0, 234, 255]),
        5 => Rgb([255, 239, 163]),
        6 => Rgb([166, 255, 125]),
        7 => Rgb([166, 255, 125]),
        _ => Rgb([0, 0, 0])
    }
}

fn main() {
    let seed = [1; 32];
    let perlin = Perlin::new(seed);
    // let seed2 = 1u64.to_be_bytes();
    // let p2 = Perlin::new(seed2);

    let width = 1000;
    let height = 1000;
    let scale = 5.0;
    for i in 0..21 {
        let mut img = ImageBuffer::new(width, height);
        for (x, y, pixel) in img.enumerate_pixels_mut() {
            // println!("{:?}", perlin.noise(x as f64, y as f64, 0f64));
            // *pixel = image::Luma([(perlin.noise(x as f64 / width as f64, y as f64 / height as f64, 0f64) * 255f64) as u8]);
            // *pixel = image::Luma([(perlin.noise(4.0 * x as f64 / width as f64, 4.0* y as f64 / height as f64, 0.0) * 255.0) as u8]);
            // *pixel = image::Luma([(perlin.noise(x as f64, y as f64, 0f64) * 255f64) as u8]);
            // let v = ((perlin.octave_noise(
            //     i,
            //     x as f64 / width as f64,
            //     y as f64 / height as f64,
            //     0.0) * 8.0).floor() * 32.0) as u8;
            let v = (perlin.octave_noise(
                9,
                scale * x as f64 / width as f64,
                scale * y as f64 / height as f64,
                i as f64 / 10.0) * 8.0).floor() as u8;
            *pixel = color(v)
            // *pixel = image::Luma([v]);
        }
        img.save(format!("noise_{}.png", i)).unwrap();
    }
}
