
use chrono::prelude::*;
use chrono::*;
use failure::Error;
use clap::{Arg, App, SubCommand};

fn main() -> Result<(), Error> {
    let matches = App::new("calculate your work perforce improve")
        .version("0.1.0")
        .author("damody <t1238142000@gmail.com>")
        .about("for cht")
        .arg(Arg::with_name("Date1")
            .required(true)
            .takes_value(true)
            .index(1)
            .help("2019-04-02"))
        .arg(Arg::with_name("Date2")
            .required(true)
            .takes_value(true)
            .index(2)
            .help("2019-10-31"))
        .arg(Arg::with_name("Improve Days")
            .required(true)
            .takes_value(true)
            .index(3)
            .help("14"))
        .arg(Arg::with_name("Improve Speed")
            .required(true)
            .takes_value(true)
            .index(4)
            .help("0.99"))
        .get_matches();
    let dstr1 = matches.value_of("Date1").unwrap();
    let dstr2 = matches.value_of("Date2").unwrap();
    let improve_speed = matches.value_of("Improve Speed").unwrap().parse::<f64>()?;
    let improve_days = matches.value_of("Improve Days").unwrap().parse::<i64>()?;
    
    let mut d1 = NaiveDate::parse_from_str(dstr1, "%Y-%m-%d")?;
    let mut d2 = NaiveDate::parse_from_str(dstr2, "%Y-%m-%d")?;
    let start = d1.clone();
    
    let mut count = 0;
    let mut improve = 1.0 as f64;
    let dur1 = d2.signed_duration_since(d1);
    let origin_day = dur1.num_days();
    loop {
        let dur1 = d2.signed_duration_since(d1);
        if (dur1.num_seconds() <= 0) {
            break;
        }
        count += 1;
        improve *= improve_speed;
        let m99 = Duration::seconds((dur1.num_seconds() as f64*improve_speed) as i64);
        let d3 = d1.checked_add_signed(m99).unwrap();
        let dur2 = d3.signed_duration_since(d1);
        d1 = d1.checked_add_signed(Duration::days(improve_days)).unwrap();
        d2 = d3;
        //println!("Duration: {:?}, {:?}", dur1, d3);
        //println!("As whole days: {:?}, {:?}", dur1.num_days(), dur2.num_days());
        //println!("Date: {:?}, {:?}", d1, d2);
    }
    let dur2 = d2.signed_duration_since(start);
    let final_day = dur2.num_days();
    println!("start from {:?}", start);
    println!("end on from {:?}", d2);
    println!("improve count {:?}", count);
    println!("improve from {} days to {} days", origin_day, final_day);
    println!("final performance {:?}", improve);
    Ok(())
}
