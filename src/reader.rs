use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Customer {
    number: u32,
    x: u32,
    y: u32,
    demand: u32,
    ready_time: u32,
    due_date: u32,
    service_time: u32,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct SolomonInstance {
    n_vehicles: usize,
    capacity: usize,
    customers: Vec<Customer>,
}

pub fn read_solomon(path: &str) -> Result<SolomonInstance, io::Error> {
    let mut n_vehicles: usize = 0;
    let mut capacity: usize = 0;
    let mut customers = Vec::new();

    // Open the file
    let path = Path::new(path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    // Flags to identify the sections in the file
    let mut is_vehicle_section = false;
    let mut is_customer_section = false;

    // Read the file line by line
    for line in reader.lines() {
        let line = line?;
        let line = line.trim(); // Trim any leading/trailing whitespace

        if line.is_empty() {
            continue;
        }

        if line == "VEHICLE" {
            is_vehicle_section = true;
            is_customer_section = false;
            continue;
        }

        if line == "CUSTOMER" {
            is_customer_section = true;
            is_vehicle_section = false;
            continue;
        }

        if is_vehicle_section {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 2 {
                if parts[0] == "NUMBER" {
                    continue;
                }
                n_vehicles = parts[0].parse().unwrap();
                capacity = parts[1].parse().unwrap();
                is_vehicle_section = false;
            }
        }

        if is_customer_section {
            let parts: Vec<&str> = line.split_whitespace().collect();
            if parts.len() == 7 {
                let number: u32 = parts[0].parse().unwrap();
                let x: u32 = parts[1].parse().unwrap();
                let y: u32 = parts[2].parse().unwrap();
                let demand: u32 = parts[3].parse().unwrap();
                let ready_time: u32 = parts[4].parse().unwrap();
                let due_date: u32 = parts[5].parse().unwrap();
                let service_time: u32 = parts[6].parse().unwrap();
                customers.push(Customer {
                    number,
                    x,
                    y,
                    demand,
                    ready_time,
                    due_date,
                    service_time,
                });
            }
        }
    }

    // Check that the data was parsed correctly
    assert!(n_vehicles > 0);
    assert!(capacity > 0);
    assert!(customers.len() > 0);

    Ok(SolomonInstance {
        n_vehicles,
        capacity,
        customers: customers[1..].to_vec(),
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn can_read_solomon() {
        let instance = read_solomon("data/solomon/C101.100.txt").unwrap();
        assert_eq!(instance.n_vehicles, 25);
        assert_eq!(instance.capacity, 200);
        assert_eq!(instance.customers.len(), 100);
    }
}
