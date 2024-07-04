use rkaiq::prelude::*;
use rkaiq::types::{OpMode, WorkingMode};
use std::thread;
use std::time::Duration;

fn main() {
    //let sne_ent_name = "m01_b_imx415 4-001a";
    // let sne_ent_name = "/dev/v4l-subdev3";
    //let sne_ent_name = "m00_b_ov5695 4-0036-1";
    let sne_ent_name = "m00_b_imx415 7-001a";
    let ctx = Context::new(sne_ent_name, "/etc/iqfiles").unwrap();
    println!("Context ready!");
    ctx.set_exp_mode(OpMode::Manual).unwrap();
    ctx.set_wb_mode(OpMode::Manual).unwrap();
    ctx.prepare(2592, 1944, WorkingMode::Normal).unwrap();
    ctx.start().unwrap();
    println!("Wait for 10 secs!");
    thread::sleep(Duration::from_secs(10));
    println!("Wait stopping ...");
    ctx.stop(false).unwrap();
    println!("Done!");
}
