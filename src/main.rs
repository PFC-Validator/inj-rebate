use serde::Deserialize;
use std::{env, fs};
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Write;
use std::ops::Div;
use std::str::FromStr;
use rust_decimal::Decimal;

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
    if args.len() != 4 {
        println!("Usage: {} <previous> <delegation_directory> <output-file>", args[0]);
        return;
    }
    let prior_list: Delegations =
        serde_json::from_reader(std::fs::File::open(&args[1]).unwrap()).unwrap();
    let prior: HashSet<String> = prior_list.delegation_responses.iter().map(|x| x.delegation.delegator_address.clone()).collect();
    let mut first:bool = true;
    let divisor= Decimal::from(10_u64.pow(18));

    let paths = fs::read_dir(&args[2]).expect("Unable to read directory");
    println!("{:40}\t{:20}\tNew\tCut\t#","File name","Delegations");

    for path in paths {
        let delegation_file = path.expect("Expecting a file");
        let meta = delegation_file.metadata().expect("wanting meta");
        if meta.is_file() {
         //   println!("processing file {:?}", delegation_file.file_name());
            let filename = format!("{}/{}", args[2], delegation_file.file_name().into_string().expect("wierd os string error"));
            let delegations: Delegations =
                serde_json::from_reader(std::fs::File::open(&filename).unwrap()).unwrap();
        //    println!("{} {}", filename, delegations.delegation_responses.len());
            // this file contains only 'new' delegators
            let  this_file = delegations.delegation_responses.iter()
                .filter(|d| !prior.contains(&d.delegation.delegator_address))
                .map(|x| (x.delegation.delegator_address.clone(), Decimal::from_str(&x.delegation.shares).ok().unwrap_or_default())).collect::<HashMap<_,_>>();
               if first {
                first = false ;
                in_the_cut = this_file.clone();
            } else {
                // reduce delegation amount if it happened
                for entry in &this_file {
                    if let Some(existing) = in_the_cut.get(entry.0) {
                        // if they reduced the amount. they only get credit for smaller amount
                        if entry.1.le(existing ) {
                            in_the_cut.insert(entry.0.clone(),existing.clone());
                        }
                    }
                }
                // remove delegators no longer appearing
                in_the_cut = in_the_cut.into_iter().filter(|x| this_file.contains_key(&x.0)).collect();
            }
            //println!("FileName\tDelegations\tNew\tCut");
            println!("{:40}\t{:20}\t{}\t{}\t{:.0}", filename, &delegations.delegation_responses.len(), this_file.len(), in_the_cut.len(),this_file.iter().map(|x|x.1).sum::<Decimal>().div(divisor));

        }
    }
    let sum = in_the_cut.iter().map(|x|x.1).sum::<Decimal>();
    println!("Sum:{:0}", sum.div(divisor));
    let mut out_file = File::create(&args[3]).expect("Can't write output file");
    for entry in in_the_cut {
        writeln!(out_file,"{},{},{}",entry.0,entry.1, entry.1/sum).expect("bad write");
    }


}
