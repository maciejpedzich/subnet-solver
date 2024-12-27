use regex::Regex;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::env::args;
use std::fmt::{Display, Formatter};
use std::net::Ipv4Addr;
use std::process::exit;
use std::str::FromStr;

fn main() {
    println!("(c) 2024 Maciej Pędzich. Released under the GNU General Public License v3.0");
    println!();

    let arguments = args().collect::<Vec<String>>();

    if arguments.len() != 4 {
        eprintln!("Please enter 3 arguments:");
        eprintln!();
        eprintln!("1. Main subnet CIDR, eg. 12.34.56.78/9");
        eprintln!(
            "2. Comma-separated list of subnets with their names and minimum number of hosts, \
            eg. \"(A,12), (B,34), (C,56)\""
        );
        eprintln!(
            "3. A-Z to order subnets with the same sizes alphabetically, \
            or Z-A to use reverse alphabetical order"
        );
        exit(1);
    }

    let input_cidr = arguments[1].split_once("/").unwrap();
    let input_ip = Ipv4Addr::from_str(input_cidr.0).unwrap();
    let input_num_subnet_bits = input_cidr.1.parse::<u32>().unwrap();
    let input_num_host_bits = Ipv4Addr::BITS - input_num_subnet_bits;

    let input_subnet_mask = Ipv4Addr::from(
        ((1 << input_num_subnet_bits) - 1) << input_num_host_bits,
    );
    let input_broadcast_mask = Ipv4Addr::from((1 << input_num_host_bits) - 1);
    let input_subnet_base_ip = input_ip & input_subnet_mask;
    let input_subnet_broadcast_ip = input_subnet_base_ip | input_broadcast_mask;

    println!("Input subnet's base IP: {}", input_subnet_base_ip);
    println!("Input subnet's broadcast IP: {}", input_subnet_broadcast_ip);
    println!();

    let subnet_pattern = Regex::new(r"\((\w),(\d+)\)").unwrap();
    let ordered_subnets = subnet_pattern
        .captures_iter(&arguments[2])
        .map(|capture| {
            let [name, min_num_hosts] = capture.extract::<2>().1;
            let min_num_ips = min_num_hosts.parse::<u32>().unwrap() + 2;

            Subnet::new(name, min_num_ips)
        })
        .collect::<BTreeSet<Subnet>>();

    let mut current_subnet_base_ip = input_subnet_base_ip;

    for subnet in ordered_subnets {
        print!("{}, ", subnet);
        print!("Base IP: {}, ", current_subnet_base_ip);

        let num_subnet_bits = (subnet.size - 1).leading_zeros();
        let num_host_bits = Ipv4Addr::BITS - num_subnet_bits;
        let subnet_mask =
            Ipv4Addr::from(((1 << num_subnet_bits) - 1) << num_host_bits);

        print!("Subnet mask: {}/{}, ", subnet_mask, num_subnet_bits);

        current_subnet_base_ip =
            Ipv4Addr::from(current_subnet_base_ip.to_bits() + subnet.size);

        println!(
            "Broadcast IP: {}",
            Ipv4Addr::from(current_subnet_base_ip.to_bits() - 1)
        );
    }

    let num_input_subnet_ips = 1 << input_num_host_bits;
    let total_num_subnet_ips =
        current_subnet_base_ip.to_bits() - input_subnet_base_ip.to_bits();

    println!();
    println!("Number of available IPs: {}", num_input_subnet_ips);
    println!("Number of IPs used by all subnets: {}", total_num_subnet_ips);

    if total_num_subnet_ips > num_input_subnet_ips {
        eprintln!("ERROR: Not enough available IPs");
        exit(1);
    }
}

struct Subnet {
    name: String,
    size: u32,
}

impl Subnet {
    fn new(name: &str, min_num_ips: u32) -> Self {
        let num_host_bits = Ipv4Addr::BITS - (min_num_ips - 1).leading_zeros();

        Subnet { name: name.to_string(), size: 1 << num_host_bits }
    }
}

impl PartialEq for Subnet {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && self.size == other.size
    }
}

impl Eq for Subnet {}

impl Ord for Subnet {
    fn cmp(&self, other: &Self) -> Ordering {
        other.size.cmp(&self.size).then_with(|| {
            let name_order = args().nth(3).unwrap();

            if name_order == "A-Z" {
                self.name.cmp(&other.name)
            } else {
                other.name.cmp(&self.name)
            }
        })
    }
}

impl PartialOrd<Self> for Subnet {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Display for Subnet {
    fn fmt(&self, f: &mut Formatter) -> std::fmt::Result {
        write!(f, "{}) Size: {}", self.name, self.size)
    }
}
