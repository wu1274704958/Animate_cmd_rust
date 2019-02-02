use std::thread::sleep;
use std::ffi::OsStr;
use std::ffi::OsString;
use std::time::Duration;
use std::io::Error;

extern crate cgmath;

mod canvas;

use canvas::{Canvas};

extern crate console;

use cgmath::prelude::*;
use cgmath::{Vector4, ortho,perspective, Vector3, Rotation3, Rad, Matrix3, Basis3, Point3};
use cgmath::Matrix4;
use cgmath::Deg;

use std::mem::transmute;
use std::io::Write;
use std::str::from_utf8;
use std::convert::From;
use console::{style,Term};

impl<T> From<Vector4<T>> for canvas::Vector4<T>
{
    fn from(kind:Vector4<T>) ->canvas::Vector4<T> {
        canvas::Vector4::<T>{
            x: kind.x,
            y: kind.y,
            z: kind.z,
            w: kind.w
        }
    }
}

impl<'a,T> From<&'a Vector4<T>> for &'a canvas::Vector4<T>
{
    fn from(kind:&'a Vector4<T>) ->&'a canvas::Vector4<T> {
        unsafe{ transmute(kind) }
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

        str.drawLine(&tv2.into(), &tv4.into());
        str.drawLine(&tv4.into(), &tv3.into());
        str.drawLine(&tv3.into(), &tv5.into());
        str.drawLine(&tv5.into(), &tv2.into());

        str.drawLine(&tv1.into(), &tv2.into());

        str.drawLine(&tv3.into(), &tv1.into());
        str.drawLine(&tv1.into(), &tv4.into());
        str.drawLine(&tv1.into(), &tv5.into());

        angle += 0.1;
//        stdout.write(str.data.as_ref());
        let s = from_utf8( str.data.as_slice() ).unwrap();
        print!("{}",style(s).cyan().on_black().bold());
        sleep(Duration::from_millis(28));
    }
}
