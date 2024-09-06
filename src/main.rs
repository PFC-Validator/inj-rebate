use std::{
    collections::{HashMap, HashSet},
    env, fs,
    fs::File,
    io::{BufRead, BufReader, Write},
    ops::{Div, Mul},
    str::FromStr,
};

use rust_decimal::Decimal;
use serde::Deserialize;

#[derive(Deserialize)]
pub struct DelegationShares {
    pub delegator_address: String,
    pub validator_address: String,
    pub shares: String,
}
#[derive(Deserialize)]
pub struct Delegation {
    pub delegation: DelegationShares,
}
#[derive(Deserialize)]
pub struct Delegations {
    pub delegation_responses: Vec<Delegation>,
}
fn main() {
    //let min_inj: u128 = 10^18; // 1 inj
    //let max_inj: u128 = 1_000_*10^18; // 1k inj

    let mut in_the_cut: HashMap<String, Decimal> = Default::default();
    let args: Vec<String> = env::args().collect();
    if args.len() != 5 {
        println!("Usage: {} <whitelist> <previous> <delegation_directory> <output-file>", args[0]);
        return;
    }
    let white_list_file = File::open(&args[1]).expect("no such file");
    let buf = BufReader::new(white_list_file);
    let white_list: HashSet<String> =
        buf.lines().map(|l| l.expect("Could not parse line")).collect();

    let prior_list: Delegations =
        serde_json::from_reader(std::fs::File::open(&args[2]).unwrap()).unwrap();
    let prior: HashSet<String> = prior_list
        .delegation_responses
        .iter()
        .map(|x| x.delegation.delegator_address.clone())
        .collect();
    let mut first: bool = true;
    let divisor = Decimal::from(10_u64.pow(18));

    let paths = fs::read_dir(&args[3]).expect("Unable to read directory");
    println!("{:40}\t{:20}\tNew\tCut\t#", "File name", "Delegations");

    for path in paths {
        let delegation_file = path.expect("Expecting a file");
        let meta = delegation_file.metadata().expect("wanting meta");
        if meta.is_file() {
            //   println!("processing file {:?}", delegation_file.file_name());
            let filename = format!(
                "{}/{}",
                args[3],
                delegation_file.file_name().into_string().expect("wierd os string error")
            );
            let delegations: Delegations =
                serde_json::from_reader(std::fs::File::open(&filename).unwrap()).unwrap();
            //    println!("{} {}", filename, delegations.delegation_responses.len());
            // this file contains only 'new' delegators
            let this_file = delegations
                .delegation_responses
                .iter()
                .filter(|d| !prior.contains(&d.delegation.delegator_address))
                .map(|x| {
                    (
                        x.delegation.delegator_address.clone(),
                        Decimal::from_str(&x.delegation.shares).ok().unwrap_or_default(),
                    )
                })
                .collect::<HashMap<_, _>>();
            if first {
                first = false;
                in_the_cut = this_file.clone();
            } else {
                // reduce delegation amount if it happened
                for entry in &this_file {
                    if let Some(existing) = in_the_cut.get(entry.0) {
                        // if they reduced the amount. they only get credit for smaller amount
                        if entry.1.le(existing) {
                            in_the_cut.insert(entry.0.clone(), *existing);
                        }
                    }
                }
                // remove delegators no longer appearing
                in_the_cut =
                    in_the_cut.into_iter().filter(|x| this_file.contains_key(&x.0)).collect();
            }
            //println!("FileName\tDelegations\tNew\tCut");
            println!(
                "{:40}\t{:20}\t{}\t{}\t{:.0}",
                filename,
                &delegations.delegation_responses.len(),
                this_file.len(),
                in_the_cut.len(),
                this_file.iter().map(|x| x.1).sum::<Decimal>().div(divisor)
            );
        }
    }

    // only want whitelisted
    in_the_cut = in_the_cut.into_iter().filter(|x| white_list.contains(x.0.as_str())).collect();

    let sum = in_the_cut.iter().map(|x| x.1).sum::<Decimal>();
    println!("Total Delegated (via coupons):{:.3}", sum.div(divisor));
    let mut out_file = File::create(&args[4]).expect("Can't write output file");
    for entry in in_the_cut {
        let inj_amt = entry.1.div(divisor);
        writeln!(
            out_file,
            "{},{:.3},{:.3}%",
            entry.0,
            inj_amt,
            entry.1.mul(Decimal::from(100u64)) / sum
        )
        .expect("bad write");
    }
}
