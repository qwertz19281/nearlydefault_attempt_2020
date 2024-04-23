use super::*;
use image::Rgba;

pub fn process(inp: &Image, o: &Opt) -> anyhow::Result<Image> {
    /// distance of 2d point from zero
    #[inline(always)] fn distance0(x: i32, y: i32) -> f32 {
        ((x.abs().pow(2) + y.abs().pow(2)) as f32).sqrt()
    }

    #[inline(always)] fn getpixel(inp: &Image, wrap: bool, x: i32,y: i32) -> Rgba<u8> {
        let w = inp.width() as i32;
        let h = inp.height() as i32;
        if wrap {
            let x = (x+w)%w;
            let y = (y+h)%h;
            *inp.get_pixel(x as u32, y as u32)
        }else{
            if x<0 || y<0 || x>=w || y>=h {
                Rgba([127,127,127,0])
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

    let mut v = vec![0u8;vclen*4];

    let (vr,vv) = v.split_at_mut(vclen);
    let (vg,vv) = vv.split_at_mut(vclen);
    let (vb,va) = vv.split_at_mut(vclen);

    for (x,y,p) in dest.enumerate_pixels_mut() {
        let (xx,yy) = (x as i32, y as i32);

        let mut i = 0usize;

        for oy in (-(o.factor as i32))..=o.factor as i32 {
            for ox in (-(o.factor as i32))..=o.factor as i32 {
                let sqokstate = (distance0(ox,oy)-o.circular_mod)<=o.factor as f32;//sqok[ ( (oy.abs() as usize) * (rad as usize+1) ) + (ox.abs() as usize) ];
                if sqokstate {
                    let Rgba([r,g,b,a]) = getpixel(inp,o.wrap, xx+ox,yy+oy);
                    vr[i] = r;
                    vg[i] = g;
                    vb[i] = b;
                    va[i] = a;
                    i+=1;
                }
            }
        }

        vr.sort_unstable();
        vg.sort_unstable();
        vb.sort_unstable();
        va.sort_unstable();
        let mitte=vclen/2;
        //println!("DEBUG {} {} {} {}",vr.len(),vg.len(),vb.len(),va.len());
        *p = Rgba([vr[mitte],vg[mitte],vb[mitte],va[mitte]]);//.min(min_alpha));
    }

    Ok(dest)
}
