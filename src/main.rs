use std::collections::HashMap;
use std::fs::File;
use csv::{ByteRecord, StringRecord};
use std::io::Write;
use plotters::prelude::*;


fn main() {
    let mut reader = csv::Reader::from_path("amazon_prime_titles.csv").unwrap();
    let mut result = String::new();
    let filtered = reader.records().into_iter().map(|x| x.unwrap())
        .filter(|x| { x.get(7).is_some() })
        .collect::<Vec<StringRecord>>();
    // for record in filetered {
    //     i+=1;
    //     println!("{:?}", record);
    // }
    // println!("{i}");
    let mut mm = filtered.iter().map(|x| x.get(7).unwrap().parse::<u64>().unwrap()).collect::<Vec<u64>>();
    mm.sort();
    result.push_str(format!("Медіана: {}\n", mm.get(filtered.len() / 2).unwrap()).as_str());
    let mut ser: u64 = 0;
    filtered.iter().for_each(|f| ser += f.get(7).unwrap().parse::<u64>().unwrap());
    ser /= filtered.len() as u64;
    result.push_str(format!("Середнє: {}\n", ser).as_str());
    let t = 5;
    let us = &filtered[t..filtered.len() - t];
    let mut kl: u64 = 0;
    for im in us {
        kl += im.get(7).unwrap().parse::<u64>().unwrap();
    }
    kl /= us.len() as u64;
    result.push_str(format!("Усічене середнє при t={t}: {}\n", kl).as_str());
    kl = 0;
    // filtered.iter().for_each(|f| kl += ((f.get(7).unwrap().parse::<i64>().unwrap() - ser as i64) * ( (f.get(7).unwrap().parse::<i64>().unwrap()))) as u64);
    filtered.iter().for_each(|f| {
        let mut d = f.get(7).unwrap().parse::<i64>().unwrap() - (ser as i64);
        d = d * d;
        kl = kl + d as u64;
    });
    kl = kl / (filtered.len() - 1) as u64;
    result.push_str(format!("Дисперсія: {}\n", kl).as_str());
    result.push_str(format!("Стандартне відхилення: {}\n", (kl as f32).sqrt()).as_str());
    let stan_vi = (kl as f32).sqrt();
    kl = 0;
    filtered.iter().for_each(|f| {
        kl = kl + (f.get(7).unwrap().parse::<i64>().unwrap() - (ser as i64)).abs() as u64;
    });
    kl = kl / (filtered.len()) as u64;
    result.push_str(format!("Середнє абсолютне відхилення: {}\n", kl).as_str());


    let m = 10;
    let lk = filtered.iter().map(|x| (x.get(7).unwrap().parse::<i64>().unwrap() - m).abs() as u64).collect::<Vec<u64>>();
    result.push_str(format!("Медіанне абсолютне відхилення: {}\n\n", lk.get(lk.len() / 2).unwrap()).as_str());
    let min = filtered.iter().map(|f| f.get(7).unwrap().parse::<u64>().unwrap()).min().unwrap();
    let max = filtered.iter().map(|f| f.get(7).unwrap().parse::<u64>().unwrap()).max().unwrap();
    // let mut min_norm: Vec<StringRecord> = Vec::with_capacity(filtered.len());
    let mut file = File::create("results.txt").unwrap();

    // Write a &str in the file (ignoring the result).
    writeln!(&mut file, "{}", result).unwrap();
    let min_normal = filtered.iter().map(|x| {
        let mut b: StringRecord = StringRecord::new();
        let mut i: u8 = 0;
        for y in x {
            if i == 7 {
                b.push_field(format!("{}", (y.parse::<f32>().unwrap() - min as f32) / (max - min) as f32).as_str());
                break;
            }
            i += 1;
            b.push_field(y);
        }
        b
    }).collect::<Vec<StringRecord>>();
    let ser_normal = filtered.iter().map(|x| {
        let mut b: StringRecord = StringRecord::new();
        let mut i: u8 = 0;
        for y in x {
            if i == 7 {
                b.push_field(format!("{}", (y.parse::<f32>().unwrap() - ser as f32) / stan_vi).as_str());
                break;
            }
            i += 1;
            b.push_field(y);
        }
        b
    }).collect::<Vec<StringRecord>>();
    let mut kkkk = csv::Writer::from_path("minMaxNorm.csv").unwrap();
    min_normal.iter().for_each(|x| kkkk.write_record(x).unwrap());
    kkkk.flush().unwrap();
    kkkk = csv::Writer::from_path("serNorm.csv").unwrap();
    ser_normal.iter().for_each(|x| kkkk.write_record(x).unwrap());
    kkkk.flush().unwrap();
    kkkk = csv::Writer::from_path("filtered.csv").unwrap();
    filtered.iter().for_each(|x| kkkk.write_record(x).unwrap());
    kkkk.flush().unwrap();

    let mut root_area = BitMapBackend::new("1.png", (600, 400)).into_drawing_area();
    root_area.fill(&WHITE).unwrap();
    let mut ctx = ChartBuilder::on(&root_area)
        .set_label_area_size(LabelAreaPosition::Left, 40.0)
        .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
        .set_label_area_size(LabelAreaPosition::Right, 40.0)
        .set_label_area_size(LabelAreaPosition::Top, 40.0)
        .caption("Кількість фільмів та серіалів за рік", ("sans-serif", 40.0))
        .build_cartesian_2d(1900.0..2050.0, 0.0..1600.0)
        .unwrap();
    let mut map: HashMap<u64, u64> = HashMap::new();
    filtered.iter().for_each(|x| {
        match map.get_mut(&x.get(7).unwrap().parse::<u64>().unwrap()) {
            None => { map.insert(x.get(7).unwrap().parse::<u64>().unwrap(), 1u64); }
            Some(x) => { *x += 1; }
        }
        // map.entry(x.get(7).unwrap().parse::<u32>().unwrap())
    });
    ctx.configure_mesh().draw().unwrap();
    ctx.draw_series(
        map.iter().map(|(b, f)| Circle::new((*b as f64, *f as f64), 2.0f64, &BLUE)),
    ).unwrap();

    {
        let mut root_area1 = BitMapBackend::new("2.png", (600, 400)).into_drawing_area();
        root_area1.fill(&WHITE).unwrap();
        let mut ctx = ChartBuilder::on(&root_area1)
            .set_label_area_size(LabelAreaPosition::Left, 40.0)
            .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
            .set_label_area_size(LabelAreaPosition::Right, 40.0)
            .set_label_area_size(LabelAreaPosition::Top, 40.0)
            .caption("Кількість фільмів  за рік", ("sans-serif", 40.0))
            .build_cartesian_2d(1900.0..2050.0, 0.0..1600.0)
            .unwrap();
        let mut map: HashMap<u64, u64> = HashMap::new();
        filtered.iter().for_each(|x| {
            if x.get(1).unwrap().eq("Movie") {
                match map.get_mut(&x.get(7).unwrap().parse::<u64>().unwrap()) {
                    None => { map.insert(x.get(7).unwrap().parse::<u64>().unwrap(), 1u64); }
                    Some(x) => { *x += 1; }
                }
                // map.entry(x.get(7).unwrap().parse::<u32>().unwrap())
            }
        });
        ctx.configure_mesh().draw().unwrap();
        ctx.draw_series(
            map.iter().map(|(b, f)| Circle::new((*b as f64, *f as f64), 2.0f64, &BLUE)),
        ).unwrap();
    }
    {
        let mut root_area1 = BitMapBackend::new("3.png", (600, 400)).into_drawing_area();
        root_area1.fill(&WHITE).unwrap();
        let mut ctx = ChartBuilder::on(&root_area1)
            .set_label_area_size(LabelAreaPosition::Left, 40.0)
            .set_label_area_size(LabelAreaPosition::Bottom, 40.0)
            .set_label_area_size(LabelAreaPosition::Right, 40.0)
            .set_label_area_size(LabelAreaPosition::Top, 40.0)
            .caption("Дитячий контент на амазоні щорічно", ("sans-serif", 40.0))
            .build_cartesian_2d(1900.0..2050.0, 0.0..1600.0)
            .unwrap();
        let mut map: HashMap<u64, u64> = HashMap::new();
        filtered.iter().for_each(|x| {
            if x.get(10).unwrap().contains("Kids") {
                match map.get_mut(&x.get(7).unwrap().parse::<u64>().unwrap()) {
                    None => { map.insert(x.get(7).unwrap().parse::<u64>().unwrap(), 1u64); }
                    Some(x) => { *x += 1; }
                }
                // map.entry(x.get(7).unwrap().parse::<u32>().unwrap())
            }
        });
        ctx.configure_mesh().draw().unwrap();
        ctx.draw_series(
            map.iter().map(|(b, f)| Circle::new((*b as f64, *f as f64), 2.0f64, &BLUE)),
        ).unwrap();
    }
}
