use wav_parser::StereoFrame;


// Using linear interpolation, repitch the array to desired hz
pub fn repitch(orig_freq: f64, new_freq: f64, mut samples: Vec<StereoFrame>) -> Vec<StereoFrame> {
    let repitch_factor = (orig_freq/new_freq) as f32;

    let mut repitched = Vec::new();
    for i in 0..(((samples.len()-1) as f32 * repitch_factor) as usize) {
        let mut new_frame = StereoFrame(0.0, 0.0);
        let mut s0 = 0.0;
        let mut s1 = 0.0;
        for j in 0..2 {
            let index = ((i as f32) / repitch_factor) as usize;
            match j {
                0 => {
                    s0 = samples[index].get_left();
                    s1 = samples[index + 1].get_left();
                },
                1 => {
                    s0 = samples[index].get_right();
                    s1 = samples[index + 1].get_right();
                },
                _ => (),
            }
            new_frame.set_value(j, s0);
        }
        repitched.push(new_frame);
    }
    repitched
}

#[cfg(test)]
mod tests {
    use wav_parser::parse_wav;
    use pitcher::repitch;

    #[test]
    fn test_parse_wav() {
        //println!("{:?}", parse_wav("E:\\Devel\\Repositories\\School\\Granulizor\\assets\\pads.wav").unwrap().len());
        println!("{:?}", repitch(1.0, 2.0, parse_wav("E:\\Devel\\Repositories\\School\\Granulizor\\assets\\pads.wav").unwrap()));
    }
}