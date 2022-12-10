/* 
    Using a hash map and vectors, create a text interface to allow a user to add employee names
    to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.”
    Then let the user retrieve a list of all people in a department or all people in the company by
    department, sorted alphabetically.
*/

mod cli_interface;

use std::collections::HashMap;
use cli_interface::cli_interface;

fn add_data(database: &mut HashMap<String, Vec<String>>, input: String) {
        let string_to_vec: Vec<&str> = input.split_whitespace().collect();
        // dbg!(&string_to_vec);

        let name = string_to_vec[1].to_owned();
        let department = string_to_vec[3].to_owned();
        database.entry(department).or_insert(Vec::new()).push(name);
        dbg!(database);
} 

fn main() 
{
        let mut database: HashMap<String, Vec<String>> = HashMap::new();
        cli_interface();
}
