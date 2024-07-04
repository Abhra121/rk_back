
use gst::prelude::*;
use gst_video::VideoMeta;
use rkaiq::prelude::*;
use rkaiq::types::{OpMode, WorkingMode,CprocAttrib, WbScene, WbGain, Sync};
use std::thread;
use std::time::Duration;
//use rkaiq::types::rk_aiq_uapi_sync_e::RK_AIQ_UAPI_MODE_DEFAULT;

//use rkaiq_sys::rk_aiq_wb_mwb_mode_e::RK_AIQ_MWB_MODE_SCENE;
//use rkaiq::awb::WbOpMode;

fn delay_ms(ms: u64) {
    thread::sleep(Duration::from_millis(ms));
}

fn main() {
    gst::init().unwrap();

    // 初始化 AIQ 上下文
    let sne_ent_name = "m01_b_imx415 4-001a";
    //let sne_ent_name = "m00_b_ov5695 4-0036-1";
    //let sne_ent_name = "m00_b_imx415 7-001a";
    let ctx = Context::new(sne_ent_name, "/etc/iqfiles/").unwrap();
    ctx.set_exp_mode(OpMode::Auto).unwrap();
    //ctx.set_wb_mode(MWBalanceMode::Mode_Scene).unwrap();
    ctx.set_wb_mode(OpMode::Auto).unwrap();
    ctx.prepare(3864, 2192, WorkingMode::Normal).unwrap();

    // 创建 GStreamer 管线
    let pipeline_cmd =
        "v4l2src io-mode=mmap name=vsrc ! video/x-raw,format=NV12,width=1920,height=1080 ! waylandsink";
    let mut context = gst::ParseContext::new();
    let pipeline =
        gst::parse_launch_full(&pipeline_cmd, Some(&mut context), gst::ParseFlags::empty())
            .unwrap();
    let pipeline = pipeline.dynamic_cast::<gst::Pipeline>().unwrap();

    let elm = pipeline.by_name("vsrc").unwrap();
    let pad = elm.static_pad("src").unwrap();
    pad.add_probe(gst::PadProbeType::BUFFER, move |pad, probe_info| {
        if let Some(gst::PadProbeData::Buffer(ref buffer)) = probe_info.data {
            //println!("{:?}", buffer);
            if let Some(meta) = buffer.meta::<VideoMeta>() {
            //    //println!("{:?}", meta);
            } else if let Some(ref caps) = pad.current_caps() {
            //    println!("{:?}", caps);
            } else {
            }
        }
        gst::PadProbeReturn::Ok
    });

    let gst_bus = pipeline.bus().unwrap();

    // 将视频管线放入单独线程中执行
    let gst_thread = {
        let bus = gst_bus.clone();
        thread::spawn(move || {
            pipeline
                .set_state(gst::State::Playing)
                .expect("Unable to set the pipeline to the `Playing` state");

            for msg in bus.iter_timed(gst::ClockTime::NONE) {
                use gst::MessageView;

                match msg.view() {
                    MessageView::Eos(..) => break,
                    MessageView::Error(err) => {
                        println!("{}", err.error());
                        break;
                    }
                    _ => (),
                }
            }

            pipeline
                .set_state(gst::State::Null)
                .expect("Unable to set the pipeline to the `Null` state");
        })
    };

    // 启动 AIQ 控制系统
    ctx.start().unwrap();

    // 1 秒钟后设置对比度为 0
    delay_ms(3000);
    //ctx.set_saturation(0).unwrap();
     //ctx.set_sharpness(0).unwrap();
    // 5 秒钟后设置对比度为 100
    let out4 = ctx.get_wb_mode();
    println!("{:?}", out4);
    
    ctx.set_wb_mode(OpMode::Manual).unwrap();
    //println!("{}", out);
    //println!("{:?}", out);
    delay_ms(1000);

    let  out1=ctx.get_wb_mode();
    println!("{:?}", out1);
    delay_ms(1000);
    let out7= ctx.set_mwb_scene(WbScene::RK_AIQ_WBCT_DAYLIGHT);
    //let out7= ctx.get_mwb_ct();
    println!("{:?}", out7);
    delay_ms(1000);
    let out11=ctx.get_mwb_scene();
    println!("{:?}", out11);
    let out3= ctx.get_mwb_gain();
    println!("{:?}", out3);
    delay_ms(1000);
    
    //ctx.set_sharpness(100).unwrap();
    //ctx.set_saturation(100).unwrap();
    let out2= ctx.unlock_awb();
    println!("{:?} unlock awb", out2);
    delay_ms(1000);
    let out9=ctx.get_cproc_attrib();
    println!("{:?}", out9);
    for m in out9 {
        println!("Found Brightness: {}\n{:#?}", m.brightness, m);
        println!("Found sync: {:?}", m.sync);
        let out99=m.sync;
        ctx.set_cproc_attrib(CprocAttrib{sync:(m.sync) , brightness: 250, contrast: 128, saturation: 128, hue: 128 });
        //for n in out99 {
        //    println!("Found sync_mode: {:?}", n.sync_mode);
        //}
    }
         
    //let out12= ctx.get_gamma_coef();
    //println!("{:?}", out12);
    //let out8= ctx.set_mwb_gain(WbGain.bri.125);
    //let out81= ctx.set_cproc_attrib(out9);
    let out8= ctx.set_mwb_gain(WbGain{ rgain: 2.99993, grgain: 1.0, gbgain: 1.11, bgain: 2.8367 });
    //let out81= ctx.set_cproc_attrib(CprocAttrib{out99 , brightness: 250, contrast: 128, saturation: 128, hue: 128 });
    delay_ms(2000);// {sync: rk_aiq_uapi_sync_s { sync_mode: RK_AIQ_UAPI_MODE_DEFAULT, done: false}
    //let out82= ctx.set_cproc_attrib(CprocAttrib{sync: rk_aiq_uapi_sync_s { sync_mode: RK_AIQ_UAPI_MODE_DEFAULT, done: false},brightness: 50, contrast: 128, saturation: 128, hue: 128 }); 
    let out6= ctx.lock_awb();
    println!("{:?} lock awb", out6);

    let out91=ctx.get_cproc_attrib();
    println!("{:?}", out91);
    // 5 秒钟后设置对比度为 255
    delay_ms(5000);
    let out5= ctx.get_mwb_gain();
    for m in out5 {
        println!("Found rgain: {}\n{:#?}", m.rgain, m);
        //ctx.set_mwb_gain(m.rgain::139);
    }
    
    //println!("{:?}", out5);
    
    //let out3= ctx.get_mwb_gain();
    //println!("{:?}", out3);
    //delay_ms(1000);
    //ctx.set_mwb_gain(gain::Rgain).unwrap();
    //ctx.lock_awb().unwrap();
    //let out6= ctx.lock_awb();
    //println!("{:?} lock awb", out6);
    //ctx.set_saturation(255).unwrap();
    //ctx.set_sharpness(0).unwrap();
    delay_ms(10000);

    // 5 秒钟后停止 GStreamer 管线
    //delay_ms(5000);
    //ctx.set_sharpness(25).unwrap();
    //delay_ms(5000);

    gst_bus.post(gst::message::Eos::new()).unwrap();
    gst_thread.join().unwrap();

    // 停止 AIQ 控制系统
    ctx.stop(false).unwrap();
}
