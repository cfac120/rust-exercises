use std::collections::HashMap;
use std::io;
fn main() {
    let mut company = HashMap::new();
    let mut employee_name  = String::from("");
    let mut employee_department = String::from("");
    let department_a = String::from("Enginerering");
    let department_b = String::from("Sales");
    let department_c = String::from("Finance");
    let department_d = String::from("Support");
    'add_name :loop {
        println!("Please type the name of new worker(Alphabet only!)");
        io::stdin().read_line(&mut employee_name).unwrap();
        let employeename = employee_name.clone();
        if company.contains_key(&employeename) {
            employee_name.clear();
            continue;
        }else {
            loop {
                println!("Which department that the new employee will add in?(A/B/C/D)");
                println!("A for {department_a},B for {department_b}, C for {department_c}, D for {department_d}");
                io::stdin().read_line(&mut employee_department).unwrap();
                let departmet_option = employee_department.trim().to_uppercase();
                match departmet_option.as_str(){
                  "A" => company.insert(employeename, &department_a),
                  "B" => company.insert(employeename, &department_b),
                  "C" => company.insert(employeename, &department_c),
                  "D" => company.insert(employeename, &department_d),
                   _ => {
                       println!("Please input correct option, only A/B/C/D!");
                       employee_department.clear();
                       continue;
                     }
                };
                print!("You added {}", employee_name);
                print!("to the {} department", company.get(&employee_name).unwrap());
                employee_department.clear();
                employee_name.clear();
                break;
               }
        }
            loop {
                println!("Are you want to add another one,Y/N?");
                let mut y_or_n= String::from("");
                io::stdin().read_line(&mut y_or_n).unwrap();
                let continue_option = y_or_n.trim().to_uppercase();
                match continue_option.as_str() {
                   "Y" => {continue 'add_name;}
                   "N" => {break 'add_name;}
                    _ => {
                       println!("Please input correct option, only Y/N!");
                       continue;
                     }
               }  
            }
            
        }
    let mut company_to_vec = Vec::new();
    for (x,y) in company.iter() {
        company_to_vec.push((x, y));
    }
    company_to_vec.sort_unstable();
    company_to_vec.sort_by_key(|(_, k)|*k);
    for (a,b) in company_to_vec {
        println!("{a}\r: {b}");
     }
}