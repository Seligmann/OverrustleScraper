use std::io;
use std::fs::File;
use std::collections::HashMap;
use select::document::Document;
use select::predicate::Name;
use reqwest;

fn main() {
    scrape_overrustle();
}

fn scrape_overrustle() {
    let bad_hrefs = vec!["userlogs", "broadcaster", "subscribers", "bans", "top100"];
    let months = vec![
        "January",
        "February",
        "March",
        "April",
        "May",
        "June",
        "July",
        "August",
        "September",
        "October",
        "November",
        "December"
    ];
    let months_map = HashMap::from([
        ("January", "01"),
        ("February", "02"),
        ("March", "03"),
        ("April", "04"),
        ("May", "05"),
        ("June", "06"),
        ("July", "07"),
        ("August", "08"),
        ("September", "09"),
        ("October", "10"),
        ("November", "11"),
        ("December", "12"),
    ]);

    let mut years: Vec<i32> = Vec::new();
    for n in 2013..2070 {
        years.push(n);
    }

    let url = "https://overrustlelogs.net/Destinygg%20chatlog";
    let chatlog = "/Destinygg chatlog/";
    let resp = reqwest::blocking::get(url).unwrap();
    assert!(resp.status().is_success());

    Document::from_read(resp)
        .unwrap()
        .find(Name("a"))
        .filter_map(|n| n.attr("href"))
        .for_each(|link| {
            if link.contains(chatlog) {
                for month in months.iter() {
                    if link.contains(month) {
                        for year in &years {
                            if link.contains(&year.to_string()) {
                                // Get each text file
                                let mut url: String = "https://overrustlelogs.net/Destinygg%20chatlog".to_owned();
                                let month_year: String = "/".to_owned() + *month + &*"%20".to_owned() + &*year.to_string();
                                url.push_str(&month_year);

                                // get request to specific month and year
                                let resp = reqwest::blocking::get(url).unwrap();
                                assert!(resp.status().is_success());

                                Document::from_read(resp)
                                    .unwrap()
                                    .find(Name("a"))
                                    .filter_map(|n| n.attr("href"))
                                    .for_each(|link|{
                                        if link.contains(chatlog) {
                                            // check if link is for the logs of a single day
                                            let mut flag = false;
                                            for bad_href in bad_hrefs.iter() {
                                                if link.contains(bad_href) { flag = true };
                                            }
                                            if !flag {
                                                let len = link.len();
                                                let day = &link[len-2..];
                                                let mut url: String = "https://overrustlelogs.net/Destinygg%20chatlog".to_owned();
                                                let month_year: String = "/".to_owned() + *month + &*"%20".to_owned() + &*year.to_string();
                                                let day_month_year: String = "/".to_owned() + &*year.to_string() + &"-".to_owned() + months_map[*month] + &"-".to_owned() + day + &".txt".to_string();

                                                url.push_str(&month_year);
                                                url.push_str(&day_month_year);


                                                let mut yymmdd: String = year.clone().to_string().to_owned();
                                                let month: String = "-".to_owned() + months_map[*month];
                                                let day: String = "-".to_owned() + day + &".txt".to_string();
                                                yymmdd.push_str(&month);
                                                yymmdd.push_str(&day);

                                                download(&url, &yymmdd);
                                            }
                                        }
                                    })
                            }
                        }
                    }
                }
            }
        });
}

fn download(url: &str, name: &str) {
    let url = String::from(url);
    let resp = reqwest::blocking::get(url).expect("Failed to get url");
    let body = resp.text().expect("Body is invalid");
    let mut out = File::create(name).expect("Failed to create file");
    io::copy(&mut body.as_bytes(), &mut out).expect("Failed to copy content");
}
