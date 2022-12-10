/* 
    Using a hash map and vectors, create a text interface to allow a user to add employee names
    to a department in a company. For example, “Add Sally to Engineering” or “Add Amir to Sales.”
    Then let the user retrieve a list of all people in a department or all people in the company by
    department, sorted alphabetically.
*/

/* 
    > Add Employee Name To A Department?
        Format: "Add Sally to Engineering"
        :- {INPUT}
      Query
        > By Department? 
            :- {INPUT}
          By Employee Name?
            :- {INPUT}
*/


use std::collections::HashMap;
use dialoguer::{theme::ColorfulTheme, Select, Input, console::Style};

fn add_data(database: &mut HashMap<String, Vec<String>>, input: String) 
{
        let string_to_vec: Vec<&str> = input.split_whitespace().collect();
        // dbg!(&string_to_vec);

        let name = string_to_vec[1].to_owned();
        let department = string_to_vec[3].to_owned();
        database.entry(department).or_insert(Vec::new()).push(name);
} 

fn display_cli_menu() {
        let theme = ColorfulTheme {
                values_style: Style::new().cyan().bright(),
                ..Default::default()
        };

        let init_selections = vec![
                "Add Employee Name To A Department?",
                "Query"
        ];

        let query_selections = vec![
                "By Department?",
                "By Name"
        ];

        let init_selection = Select::with_theme(&theme)
                .with_prompt("Simple CLI program to add or query a simple database")
                .default(0)
                .items(&init_selections[..])
                .interact()
                .unwrap();

        if init_selection == 0 {
                let input: String = Input::with_theme(&theme)
                        .with_prompt("Format: \"Add Sally to Engineering\"\n:-")
                        .interact_text()
                        .unwrap();
        } else {
            let query_selection = Select::with_theme(&theme)
            .default(0)
            .items(&query_selections[..])
            .interact()
            .unwrap();
        }
}

fn main() 
{
        let mut database: HashMap<String, Vec<String>> = HashMap::new();
        let s = String::from("Add Sally to Sales");


        add_data(&mut database, s);
        display_cli_menu();
}
