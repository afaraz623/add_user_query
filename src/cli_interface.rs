use dialoguer::{theme::ColorfulTheme, Select, Input, console::Style};

/*
 * menu_selections and user_inputs are only 
 * here to wrap the dialoguer libray to increase
 * code readibity and help minimize code repetition. 
 */
fn menu_selections(theme: &ColorfulTheme, menu_items: &Vec<&str>, prompt_msg: &String) -> usize
{
        let selec_index = Select::with_theme(theme)
                .with_prompt(prompt_msg)
                .default(0)
                .items(&menu_items[..])
                .interact()
                .unwrap();

        selec_index
}

fn user_inputs(theme: &ColorfulTheme, prompt_message: &String) -> String
{
        let input: String = Input::with_theme(theme)
        .with_prompt(prompt_message)
        .interact_text()
        .unwrap();
        
        input
}

/*
 * This is the main menu function from 
 * which the user can select what they 
 * want to do.
 */
fn init_menu_cli(theme: &ColorfulTheme) -> usize
{
        let init_selections = vec![
                "Add Employee Name To A Department?",
                "Query Database"
        ];

        let init_message = String::from("Add/Query data from a simple program");

        let menu_selec = menu_selections(&theme, &init_selections, &init_message);

        menu_selec
}

/*
 * Sub-menu function which is the result
 * of "Add Employee Name To A Department?"
 * and returns user entry
 */  
fn add_data_cli(theme: &ColorfulTheme)
{
        let entry_format_msg = String::from("Format: \"Add Sally to Engineering\"\n:-");
        loop 
        {
                let tmp = user_inputs(theme, &entry_format_msg);
                if tmp.to_lowercase() == "back" {break};
        }
}

/*
 * Sub-menu function which is the result
 * of "Query Database". It goes one level
 * down to help the user be more specfic 
 * when querying.
 */
fn query_data_cli(theme: &ColorfulTheme)
{
        let query_selections = vec![
                "By Department?",
                "By Name?",
                "Back"
        ];

        let query_selec_msg = String::from("Query data");

        loop 
        {
                let tmp = menu_selections(theme, &query_selections, &query_selec_msg);
                match tmp {
                        0 => query_depart_cli(theme),
                        1 => query_emply_cli(theme),
                        2 => break,
                        _ => println!("SOMETHING IS WRONG!!!")
                }
        }
}

/*
 * This function is one level below the
 * "By Department?" sub-menu and it returns
 * user entry  
 */
fn query_depart_cli(theme: &ColorfulTheme) 
{
        let query_depart_msg = String::from("Enter department name to see which employees are in it\n:-");
        
        loop 
        {
                let tmp = user_inputs(theme, &query_depart_msg);
                if tmp.to_lowercase() == "back" {break};
        }
}

/*
 * This function is one level below the
 * "By Name?" sub-menu and it returns
 * user entry  
 */
fn query_emply_cli(theme: &ColorfulTheme) 
{
        let query_emply_msg = String::from("Enter employee name to see which department they work in\n:-");
        
        loop 
        {
                let tmp = user_inputs(theme, &query_emply_msg);
                if tmp.to_lowercase() == "back" {break};
        }
}

/*
 * This function is called to start the
 * cli interface.
 *
 *  > Add Employee Name To A Department?
 *      Format: "Add Sally to Engineering"
 *      :- {INPUT}
 *    Query
 *      > Search By Department? 
 *          :- {INPUT}
 *        Search By Employee Name?
 *          :- {INPUT}
 *        Back
 *    Exit 
 */
pub fn cli_interface() 
{
        let theme = ColorfulTheme {
                values_style: Style::new().cyan().bright(),
                ..Default::default()
        };

        loop 
        {
                if init_menu_cli(&theme) == 0 
                {
                        add_data_cli(&theme);   
                } 
                else 
                {
                        query_data_cli(&theme);
                }
        }
}
