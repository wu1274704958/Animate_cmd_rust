use std::thread::sleep;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::time::Duration;
use std::io::Error;

extern crate cgmath;
extern crate console;

use cgmath::prelude::*;
use cgmath::{Vector4, ortho,perspective, Vector3, Rotation3, Rad, Matrix3, Basis3, Point3};
use cgmath::Matrix4;
use cgmath::Deg;

use std::io::Write;
use std::str::from_utf8;

use console::{style,Term};


struct Canvas {
    pub data: Vec<u8>,
    pub zb : Vec<i8>,
    pub w: u32,
    pub h: u32,
}

fn _max(x:i32, y:i32, z:i32) ->i32
{
    let v = if x > y { x }else{y};
    if v > z { v }else { z }
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Canvas {
        let mut c = Canvas {
            data: vec![b' '; ((width + 1) * height) as usize],
            zb : vec![-128;(width * height) as usize],
            w: width,
            h: height,
        };
        c.init();
        c
    }
    pub fn init(&mut self) {
        self.data.iter_mut().for_each(|it| { *it = b' ' });
        for i in 0..self.h {
            self.data[(i * (self.w + 1) + self.w) as usize] = b'\n';
        }
        self.zb.iter_mut().for_each(|it|{*it = -128});
    }
    pub fn setPixel(&mut self, x: u32, y: u32 ,z:i32)
    {
        //print!("{}\n",z);
        let mut p : u8;
        p = match z {
            -10 => b'\'',
            -9 => b'\'',
            -8 => b'`',
            -7 => b':',
            -6 => b';',
            -5 => b'-',
            -4 => b'~',
            -3 => b'=',
            -2 => b'|',
            -1 => b'\\',
            0  => b'\\',
             1 => b'!',
             2 => b'I',
             3 => b'J',
             4 => b'L',
             5 => b'E',
             6 => b'P',
             7 => b'R',
             8 => b'$',
             9 => b'#',
             10 => b'@',
            _ => b'#'
        };
        if self.zb[(y * self.w + x) as usize] < (z as i8)
        {
            self.data[(y * (self.w + 1) + x) as usize] = p;
            self.zb[(y * self.w + x) as usize] = z as i8;
        }
    }
    pub fn inBound(&self, x: i32, y: i32) -> bool {
        x >= 0 && x < self.w as i32 && y >= 0 && y < self.h as i32
    }

    fn drawLine(&mut self, p1: &Vector4<f32>, p2: &Vector4<f32>)
    {
        let mut x0 = p1.x as i32;
        let mut y0 = p1.y as i32;
        let mut z0 = p1.z as i32;
        let x1 = p2.x as i32;
        let y1 = p2.y as i32;
        let z1 = p2.z as i32;
        let dx = ((x1 - x0) as f32).abs() as i32;
        let sx = if x0<x1 { 1 } else{ -1};
        let dy = ((y1 - y0) as f32).abs() as i32;
        let sy = if y0<y1  {1} else { -1 };
        let dz = ((z1 - z0) as f32).abs() as i32;
        let sz = if z0<z1 { 1 }else {  -1 };
        let dm = _max(dx, dy, dz);
        let mut i = dm; /* maximum difference */
        let mut z1 = dm / 2;
        let mut y1 = z1;
        let mut x1 = y1; /* error offset */

        loop{  /* loop */
            if self.inBound(x0,y0) {
                self.setPixel(x0 as u32, y0 as u32, z0);
            }

            if i == 0 {break;}
            i-=1;
            x1 -= dx; if x1 < 0 { x1 += dm; x0 += sx; }
            y1 -= dy; if y1 < 0 { y1 += dm; y0 += sy; }
            z1 -= dz; if z1 < 0 { z1 += dm; z0 += sz; }
        }
    }
}

fn main() {
    let mat: Matrix4<f32> = perspective(Deg(60f32),  1.0f32, 0.3f32, 1000.0f32);
    let translate = Matrix4::<f32>::from_translation(Vector3::new(20f32, 20f32, -4f32));
    let scale = Matrix4::<f32>::from_scale(0.5f32);
    let rot_x = Matrix4::<f32>::from_angle_x(Rad(0.4));

    let v: Vector4<f32> =   Vector4::new(0.0,	    -20.0,	0.0,	1.0);
    let v2: Vector4<f32> =  Vector4::new(-20.0,	    20.0,	0.0,  1.0);
    let v3: Vector4<f32> =  Vector4::new(20.0,	    20.0,	0.0,  1.0);

    let v4:Vector4<f32> = Vector4::new(0.0, 20., 20., 1.);
    let v5:Vector4<f32> = Vector4::new(0.0, 20., -20., 1.);

    let mut angle = 0.0f32;

    let mut str = Canvas::new(80, 80);

    let mut stdout = Term::stdout();

    for i in 1..=3100 {
        str.init();
        //if angle >= std::f32::consts::PI {break;}
        stdout.move_cursor_up(80);
        let rot: Matrix4<f32> = Matrix4::from_axis_angle(Vector3::new(0.0f32, 1.0f32, 0f32), Rad(angle));
        let tv1 = mat * translate * rot_x * rot * scale * v;
        let tv2 = mat * translate * rot_x * rot * scale * v2;
        let tv3 = mat * translate * rot_x * rot * scale * v3;
        let tv4 = mat * translate * rot_x * rot * scale * v4;
        let tv5 = mat * translate * rot_x * rot * scale * v5;

        str.drawLine(&tv2, &tv4);
        str.drawLine(&tv4, &tv3);
        str.drawLine(&tv3, &tv5);
        str.drawLine(&tv5, &tv2);

        str.drawLine(&tv1, &tv2);

        str.drawLine(&tv3, &tv1);
        str.drawLine(&tv1, &tv4);
        str.drawLine(&tv1, &tv5);

        angle += 0.1;
//        stdout.write(str.data.as_ref());
        let s = from_utf8( str.data.as_slice() ).unwrap();
        print!("{}",style(s).cyan().on_black().bold());
        sleep(Duration::from_millis(28));
    }
}