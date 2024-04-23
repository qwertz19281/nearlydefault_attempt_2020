use super::*;
use image::Rgba;

pub fn process_alt(inp: &Image, o: &Opt) -> anyhow::Result<Image> {
    /// distance of 2d point from zero
    #[inline(always)] fn distance0(x: i32, y: i32) -> f32 {
        (((x.abs().pow(2))+(y.abs().pow(2))) as f32).sqrt()
    }

    #[inline(always)] fn getpixel(inp: &Image, wrap: bool, x: i32,y: i32) -> Rgba<u8> {
        let w = inp.width() as i32;
        let h = inp.height() as i32;
        if wrap {
            let x = (x+w)%w;
            let y = (x+h)%h;
            *inp.get_pixel(x as u32, y as u32)
        }else{
            if x<0 || y<0 || x>=w || y>=h {
                Rgba([0,0,0,0])
            }else{
                *inp.get_pixel(x as u32, y as u32)
            }
        }
    }

    let mut dest = Image::new(inp.width(),inp.height());
    //an now the median alg
    let antirad=(o.factor*2+1) as usize;

    let vclen = {
        let mut i = 0usize;
        for oy in (-(o.factor as i32))..=o.factor as i32 {
            for ox in (-(o.factor as i32))..=o.factor as i32 {
                let sqokstate = (distance0(ox,oy)-o.circular_mod)<=o.factor as f32;//sqok[ ( (oy.abs() as usize) * (rad as usize+1) ) + (ox.abs() as usize) ];
                if sqokstate {
                    i+=1;
                }
            }
        }
        i
    };

    anyhow::ensure!(vclen < 256);

    for (x,y,p) in dest.enumerate_pixels_mut() {
        let mut vr = MiniSort::new();
        let mut vg = MiniSort::new();
        let mut vb = MiniSort::new();
        let mut va = MiniSort::new();

        let (xx,yy) = (x as i32, y as i32);

        for oy in (-(o.factor as i32))..=o.factor as i32 {
            for ox in (-(o.factor as i32))..=o.factor as i32 {
                let sqokstate = (distance0(ox,oy)-o.circular_mod)<=o.factor as f32;//sqok[ ( (oy.abs() as usize) * (rad as usize+1) ) + (ox.abs() as usize) ];
                if sqokstate {
                    let Rgba([r,g,b,a]) = getpixel(inp,o.wrap, xx+ox,yy+oy);
                    vr.push(r);vg.push(g);vb.push(b);va.push(a);
                }
            }
        }
        let mitte=(vclen/2) as u8;
        //println!("DEBUG {} {} {} {}",vr.len(),vg.len(),vb.len(),va.len());
        *p = Rgba([vr.at(mitte),vg.at(mitte),vb.at(mitte),va.at(mitte)]);//.min(min_alpha));
    }

    Ok(dest)
}

pub struct MiniSort {
    stor: [u8;256],
}

impl MiniSort {
    pub fn new() -> Self {
        Self{
            stor: [0;256],
        }
    }

    pub fn push(&mut self, v: u8) {
        self.stor[v as usize]+=1;
        //self.amount+=1;
    }

    /*pub fn median(&self) -> u8 {
        self.at_unchecked(self.amount/2)
    }*/

    pub fn at(&self, index: u8) -> u8 {
        let mut amount = 0u8;

        for (i,v) in self.stor.iter().enumerate() {
            amount += v;
            if amount > index {
                return i as u8;
            }
        }

        unreachable!()
    }
}