use std::fs::File;
use csv::{ByteRecord, StringRecord};
use std::io::Write;


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
    result.push_str(format!("Медіана: {}\n", filtered.get(filtered.len() / 2).unwrap().get(7).unwrap()).as_str());
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
                b.push_field(format!("{}", (y.parse::<f32>().unwrap() - min as f32)/ (max-min) as f32).as_str());
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
}
