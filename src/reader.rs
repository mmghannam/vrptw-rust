use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Customer {
    number: usize,
    x: usize,
    y: usize,
    demand: usize,
    ready_time: usize,
    due_date: usize,
    service_time: usize,
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Instance {
    n_vehicles: usize,
    capacity: usize,
    customers: Vec<Customer>,
    start_depot: usize,
    end_depot: usize,
}

pub fn read_solomon(path: &str) -> Result<Instance, io::Error> {
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
                let number: usize = parts[0].parse().unwrap();
                let x: usize = parts[1].parse().unwrap();
                let y: usize = parts[2].parse().unwrap();
                let demand: usize = parts[3].parse().unwrap();
                let ready_time: usize = parts[4].parse().unwrap();
                let due_date: usize = parts[5].parse().unwrap();
                let service_time: usize = parts[6].parse().unwrap();
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

    let mut end_depot_cust = customers[0].clone(); // end_depot
    end_depot_cust.number = customers.len();
    customers.push(end_depot_cust); 

    let start_depot = customers[0].number;
    let end_depot = customers[customers.len() - 1].number;

    Ok(Instance {
        n_vehicles,
        capacity,
        customers,
        start_depot,
        end_depot,
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
        assert_eq!(instance.customers.len(), 102);
        assert_eq!(instance.start_depot, instance.customers.first().unwrap().number);
        assert_eq!(instance.end_depot, instance.customers.last().unwrap().number);
    }
}
