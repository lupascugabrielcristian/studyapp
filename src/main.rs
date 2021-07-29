extern crate termion;

extern crate mysql;

use std::io::stdin;
use std::iter;
use termion::{color, clear, cursor};
use mysql as my;

mod sql_database;
use crate::sql_database::db_operations;
use crate::sql_database::models::{ Location, Question, Node};

fn main() {


    println!("{clear}{goto}{red}STUDY APP{reset}",
             clear = clear::All,
             goto = cursor::Goto(4,2),
             red   = color::Fg(color::LightGreen),
             reset = color::Fg(color::Reset));

    let mut user_command = String::new();
    let mut conn:my::PooledConn = db_operations::connect();
    let mut location: Location = Location::Initial;

    // Aici voi pune nodurile prin care am trecut pana la nodul curent
    // Ultimul element adaugat este nodul curent
    let mut current_nodes: Vec<Node> = Vec::new();

    loop {
        print_cursor(&current_nodes, &location);

        stdin().read_line( &mut user_command ).expect("Din not enter corect string");
        user_command = user_command.trim().to_owned();

        if user_command == "help" {
            print_help();
        }
        else if user_command == "q" {
            // Add new question
            let user_input = ask_question();
            println!("{}", user_input);
            let question_text = String::from(user_input);
            db_operations::save_question( &question_text, &mut conn );
        }
        else if user_command == "ls" {
            // List node
            list_node(&mut current_nodes, &mut conn);
        }
        else if user_command == "list q" {
            // List questions
            list_questions( &mut conn );
        }
        else if user_command.len() < 3 {
            println!("Not a command. help");
        }
        else if &user_command[0..3].to_string() == "s q" {
            // Select a question
            let argument = &user_command[4..];
            select_question( argument, &mut conn, &mut current_nodes);
            location = Location::Question;
        }
        else if &user_command[0..3].to_string() == "a d" {
            // Add documentation
            add_documentation( &mut conn, &mut current_nodes);
            location = Location::Documentation;
        }
        else if &user_command[0..3].to_string() == "a m" {
            // Add documentation
            add_model( &mut conn, &mut current_nodes);
            location = Location::Model;
        }
        else if user_command == "exit" {
            break;
        }

        user_command.clear();
    }

}

fn print_cursor(current_nodes: &Vec<Node>, location: &Location) {
    let nodes_i = current_nodes.iter();
    for node in nodes_i {
        match location {
            Location::Initial => println!("->"),
            Location::Question => println!("{} -?", node.label),
            Location::Model => println!("{} -M", node.label),
            Location::Documentation => println!("{} -D", node.label)
        }

        println!("\t");
    }
}

fn print_help() {
    let help = "\nStudy commands\n\n\
    help \t\t\t\t show this menu \n\
    exit \t\t\t\t exit application\n\
    \nSELECT FUNCTIONS\n\
    select question(s q [index])\t select question [index]\n\
    \nLIST FUNCTIONS\n\
    list questions(list q)\t\t list all questions in database\n\
    list node(ls)\t\t\t Lists the current node and the children
    \nADD FUNCTIONS\n\
    question(q)\t\t\t add root question with name\n\
    add documentation(a d)\t\t Adds a URL for documentation to current node\n\
    add model(a m)\t\t\t Adds a model to current node";

    println!("{}", help);
}


fn ask_question() -> String {
    println!("[?]");
    let mut question = String::new();
    stdin().read_line(&mut question).expect("Din not enter correct string");
    return question.trim().to_string(); 
}


/// Dupa operatia asta raman in acelasi nod pentru a putea adauga in continuare daca este cazul
fn add_documentation( conn: &mut my::PooledConn, current_nodes: &mut Vec<Node> ) {
    let parent_node_id = current_nodes.get( current_nodes.len() - 1 ).unwrap().node_id;

    println!("[Doc label ?-]");
    let mut label = String::new();
    stdin().read_line(&mut label).expect("Din not enter correct string");

    println!("[Doc content ?-]");
    let mut documentation = String::new();
    stdin().read_line(&mut documentation).expect("Din not enter correct string");

    documentation = documentation.trim().to_string(); 
    db_operations::save_documentation(&label, &documentation, parent_node_id, conn);
}

/// Adaug model la ultimul nod in care am intrat. Nu intru in noul nod, dupa ce il adaug
fn add_model( conn: &mut my::PooledConn, current_nodes: &mut Vec<Node> ) {
    let parent_node_id = current_nodes.get( current_nodes.len() - 1 ).unwrap().node_id;

    println!("[Model label ?-]");
    let mut label = String::new();
    stdin().read_line(&mut label).expect("Din not enter correct string");
    label = label.trim().to_string();

    println!("[Model content ?-]");
    let mut model = String::new();
    stdin().read_line(&mut model).expect("Din not enter correct string");
    model = model.trim().to_string(); 
    model = model.replace("\"", "'");

    db_operations::save_model(&label, &model, parent_node_id, conn);
}


fn list_questions( conn: &mut my::PooledConn ) {
    let query = "SELECT * from questions";

    let questions: Vec<Question> = 
    conn.prep_exec(query, ()).map( |result| {
        result.map(|x| x.unwrap()).map(|row| {
            let ( node_id, question_text ) = my::from_row(row);
            Question {
                node_id,
                question_text,
            }
        }).collect()
    }).unwrap(); // Unwrap `Vec<Question>`

    let _v: Vec<_> = questions.iter().map( |q| println!("{}. {}", q.node_id, q.question_text) ).collect();
}

fn list_node(current_nodes: &mut Vec<Node>, conn: &mut my::PooledConn ) {
    for i in 0..current_nodes.len() {

        // Printez nodul parinte
        println!("{}", current_nodes.get(i).unwrap().label);
        let node = db_operations::get_node( current_nodes.get(i).unwrap().node_id, conn );

        // Printez ce copii are
        let header: String = iter::repeat("|__ ").take(i + 1).collect();
        
        node.unwrap().child_nodes.split(" ").for_each(|child_node_id| {
            match child_node_id.parse::<i32>() {
                Ok(n_id) => {
                    let child_node = db_operations::get_node( n_id, conn );
                    match child_node {
                        Some(mut cn) => { println!("{}{}", header, &cn.to_string()) },
                        None => {}
                    }
                },
                Err(_e) => {}
            }
        });
    }
}

// Vreau sa primesc aici un numar. Numarul reprezinta id-ul randului question din tabela questions
// Extrag din baza de date intrebarea cu acel id
fn select_question( argument: &str, conn: &mut my::PooledConn, current_nodes: &mut Vec<Node>) -> Option<Question> {
    let question_index: i32 = argument.parse().expect("That was not a number");

    let query = "SELECT * FROM questions WHERE node_id=':node_id'";
    let query = query.replace(":node_id", &question_index.to_string());

    let mut questions: Vec<Question> =
    conn.prep_exec(query, ()).map( |result| {
        result.map(|x| x.unwrap()).map(|row| {
            let ( node_id, question_text ) = my::from_row(row);
            Question {
                node_id: node_id,
                question_text: question_text,
            }
        }).collect()
    }).unwrap();

    let node = db_operations::get_node( question_index, conn );
    current_nodes.clear();
    current_nodes.push(node.unwrap());
    
    let mut first_question_iter = questions.drain(0..1);
    return first_question_iter.next();
}
