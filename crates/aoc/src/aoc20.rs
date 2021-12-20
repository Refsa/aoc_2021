use std::u8;

use crate::runner::Runner;

#[derive(Default)]
pub struct AOC20 {
    lookup: Vec<u8>,
    image: Vec<Vec<u8>>,
    width: usize,
    height: usize,
}

impl Runner for AOC20 {
    fn parse(&mut self, input: &Vec<String>) {
        self.lookup = input[0].chars().map(pixel_to_bit).collect();

        self.width = input[2].len();
        self.height = input.len() - 2;
        self.image = input[2..]
            .iter()
            .map(|e| e.chars().map(pixel_to_bit).collect())
            .collect();
    }

    fn run_p1(&self) -> usize {
        println!();

        let mut img = Image {
            width: self.width,
            height: self.height,
            data: self.image.clone(),
        };

        let growth = Index(8, 8);
        img.grow(growth);
        for _ in 0..2 {
            img = filter_image(&img, &self.lookup);
        }
        img.resize(Index(self.width + 4, self.height + 4));

        // img.draw();
        img.count_lit()
    }

    fn run_p2(&self) -> usize {
        let mut img = Image {
            width: self.width,
            height: self.height,
            data: self.image.clone(),
        };

        let growth = Index(4 * 50, 4 * 50);
        img.grow(growth);
        for _ in 0..50 {
            img = filter_image(&img, &self.lookup);
        }
        img.resize(Index(self.width + 2 * 50, self.height + 2 * 50));

        // img.draw();
        img.count_lit()
    }
}

fn filter_image(img: &Image, filter: &Vec<u8>) -> Image {
    let mut new_img = Image {
        width: img.width,
        height: img.height,
        data: vec![vec![0u8; img.width]; img.height],
    };

    for y in 1..img.height - 1 {
        for x in 1..img.width - 1 {
            let conv = img.conv(Point(x as isize, y as isize));
            let pix = filter[conv];
            new_img.set_pixel(Index(x, y), pix);
        }
    }

    new_img
}

struct Image {
    width: usize,
    height: usize,
    data: Vec<Vec<u8>>,
}

impl Image {
    fn draw(&self) {
        for y in 0..self.height {
            for x in 0..self.width {
                print!("{}", bit_to_pixel(&self.get_pixel(Index(x, y))));
            }

            println!();
        }
    }

    fn draw_conv(&self, point: Point) {
        for y in -1..=1 {
            for x in -1..=1 {
                let p = point + Point(x, y);
                print!("{}", bit_to_pixel(&self.get_pixel(p.into())));
            }
            println!();
        }
    }

    fn get_pixel(&self, idx: Index) -> u8 {
        self.data[idx.1][idx.0]
    }

    fn set_pixel(&mut self, idx: Index, val: u8) {
        self.data[idx.1][idx.0] = val;
    }

    fn count_lit(&self) -> usize {
        self.data.iter().flatten().map(|&e| e as usize).sum()
    }

    fn grow(&mut self, by: Index) {
        let new_w = by.0 + self.width;
        let new_h = by.1 + self.height;
        let mut new_data = vec![vec![0u8; new_w]; new_h];

        let move_x = new_w / 2 - self.width / 2;
        let move_y = new_h / 2 - self.height / 2;

        for x in 0..self.width {
            for y in 0..self.height {
                new_data[move_y + y][move_x + x] = self.data[y][x];
            }
        }

        self.data = new_data;
        self.width = new_w;
        self.height = new_h;
    }

    fn resize(&mut self, size: Index) {
        let new_w = size.0;
        let new_h = size.1;
        let mut new_data = vec![vec![0u8; new_w]; new_h];

        let move_x = self.width / 2 - new_w / 2;
        let move_y = self.height / 2 - new_h / 2;

        for x in 0..new_w {
            for y in 0..new_h {
                new_data[y][x] = self.data[move_y + y][move_x + x];
            }
        }

        self.data = new_data;
        self.width = new_w;
        self.height = new_h;
    }

    fn conv(&self, point: Point) -> usize {
        let mut conv = [0u8; 9];

        let mut idx = 0;
        for y in -1..=1 {
            for x in -1..=1 {
                let p = point + Point(x, y);
                conv[idx] = self.get_pixel(p.into());

                idx += 1;
            }
        }

        bin_to_num(&conv[..], 0)
    }
}

fn pixel_to_bit(pixel: char) -> u8 {
    match pixel {
        '#' => 1,
        '.' => 0,
        _ => unreachable!(),
    }
}

fn bit_to_pixel(bit: &u8) -> char {
    match bit {
        0 => ' ',
        1 => '█',
        _ => unreachable!(),
    }
}

fn bin_to_num(bits: &[u8], offset: usize) -> usize {
    let mut num = 0;
    for i in 0..bits.len() {
        num += (bits[bits.len() - i - 1] as usize) << ((i + offset) as usize);
    }
    num
}

#[derive(Debug, PartialEq, Eq, Hash, Copy, Clone, PartialOrd, Ord)]
pub struct Point(pub isize, pub isize);

impl std::ops::Add for Point {
    type Output = Point;

    fn add(self, r: Point) -> <Self as std::ops::Add<Point>>::Output {
        Point(self.0 + r.0, self.1 + r.1)
    }
}

impl std::ops::Sub for Point {
    type Output = Point;

    fn sub(self, rhs: Self) -> Self::Output {
        Point(self.0 - rhs.0, self.1 - rhs.1)
    }
}

#[derive(PartialEq, Eq, Hash, Copy, Clone)]
struct Index(usize, usize);

impl Into<Index> for Point {
    fn into(self) -> Index {
        Index(self.0 as usize, self.1 as usize)
    }
}

impl Into<Point> for Index {
    fn into(self) -> Point {
        Point(self.0 as isize, self.1 as isize)
    }
}
