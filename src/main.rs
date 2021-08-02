extern crate termion;

extern crate mysql;

use std::io::stdin;
use std::iter;
use std::fs;
use termion::{ color, clear, cursor, terminal_size };
use mysql as my;

mod sql_database;
use crate::sql_database::db_operations;
use crate::sql_database::models::{ Question, Node, NodeType };

fn main() {
    let mut user_command = String::new();
    let mut conn:my::PooledConn = db_operations::connect();

    // Aici voi pune nodurile prin care am trecut pana la nodul curent
    // Ultimul element adaugat este nodul curent
    let mut current_nodes: Vec<Node> = Vec::new();

    print_title();
    print_cursor(&current_nodes);

    loop {

        stdin().read_line( &mut user_command ).expect("Din not enter corect string");
        user_command = user_command.trim().to_owned();

        if user_command.len() == 0 {
            print_help(&mut current_nodes);
        }
        else if user_command == "help" {
            print_help(&mut current_nodes);
        }
        else if user_command == "q" {
            // Add new question
            ask_question(&mut current_nodes, &mut conn);
        }
        else if user_command == "subq" {
            // Add a subquestion
            add_subquestion(&mut current_nodes, &mut conn);
        }
        else if user_command == "am" {
            // Add model
            add_model( &mut conn, &mut current_nodes);
        }
        else if user_command == "doc" {
            // Add documentation
            add_documentation( &mut conn, &mut current_nodes);
        }
        else if user_command == "ls" {
            // List node
            list_node(&mut current_nodes, &mut conn);
        }
        else if user_command == "show" {
            // Show node content
            show_node_content(&mut conn, &mut current_nodes );
        }
        else if user_command == "lq" {
            // List questions
            list_questions(&mut current_nodes, &mut conn );
        }
        else if user_command == "all" {
            // Show all nodes tree
            list_all_nodes_tree(&mut current_nodes, &mut conn );
        }
        else if user_command.len() > 3 && &user_command[0..3].to_string() == "sq " {
            // Select a question
            let argument = &user_command[3..];
            select_question( argument, &mut conn, &mut current_nodes);
        }
        else if user_command.len() > 3 && &user_command[0..3].to_string() == "cd " {
            // Select a node
            let argument = &user_command[3..];
            select_node( argument, &mut conn, &mut current_nodes);
        }
        else if user_command.len() > 3 && &user_command[0..4].to_string() == "del " {
            let node_to_delete = &user_command[4..];
            delete_node( node_to_delete, &mut conn, &mut current_nodes );
        }
        else if user_command == "out" {
            // Node out 
            move_out_current_node(&mut conn, &mut current_nodes);
        }
        else if user_command.len() > 3 && &user_command[0..4] == "lat " {
            // Go to a lateral node. This means in the same parent
            let node_to_move = &user_command[4..];
            move_to_lateral_node(node_to_move, &mut conn, &mut current_nodes);
        }
        else if user_command == "term" {
            // Add new term
            add_new_term( &mut conn, &mut current_nodes );
        }
        else if user_command == "try" {
            // Add new term
            add_new_try( &mut conn, &mut current_nodes );
        }
        else if user_command == "explain" {
            add_explanation( &mut conn, &mut current_nodes );
        }
        else if user_command == "trycom" {
            update_try_comment( &mut conn, &mut current_nodes );
        }
        else if user_command.len() == 5 && &user_command[0..4] == "res " {
            // Update try result
            let argument = &user_command[4..];
            update_try_result( argument, &mut conn, &mut current_nodes);
        }
        else if user_command == "label" {
            update_node_label( &mut conn, &mut current_nodes );
        }
        else if user_command.len() > 5 && &user_command[0..3].to_string() == "mv " {
            let arguments = &user_command[3..];
            move_node(arguments, &mut conn, &mut current_nodes);
        }
        else if user_command == "content" {
            update_model_content(&mut conn, &mut current_nodes );
        }
        else if user_command == "docupdate" {
            update_documentation_content(&mut conn, &mut current_nodes );
        }
        else if user_command.len() < 3 {
            print_title();
            print_header(&mut current_nodes);
            print_content();
            println!("Not a command. help");
            print_cursor(&mut current_nodes);
        }
        else if user_command == "exit" {
            break;
        }
        else {
            print_title();
            print_header(&mut current_nodes);
            print_content();
            println!("{} is not a command. help", user_command);
            print_cursor(&mut current_nodes);
        }

        user_command.clear();
    }

}


fn print_help(current_nodes: &Vec<Node>) {
    print_title();
    print_header(current_nodes);
    print_content();

    let help = "\
    help \t\t show this menu \n\
    exit \t\t exit application\n\
    \nMOVE FUNCTIONS\n\
    sq [id]\t\t select question [index]\n\
    cd [id]\t\t select node [index]\n\
    out\t\t move out of the current node\n\
    lat [id]\t go to a node of the same parent\n\
    mv [id] [to_id]\t Move node with id as child to node with id to_id\n\
    \nLIST FUNCTIONS\n\
    lq\t\t list all questions in database\n\
    ls\t\t Lists the current node and the children\n\
    show\t\t Show the content of the node \n\
    all\t\t Show all the nodes tree\n\
    \nADD FUNCTIONS\n\
    q\t\t Add root question with name\n\
    subq\t\t Add a subquestion to current node\n\
    doc\t\t Add a URL for documentation to current node\n\
    am\t\t Add a model to current node\n\
    term\t\t Add a new term to current node\n\
    explain\t\t Add a new explanation for a TERM node\n\
    try\t\t Add a try node to current node\n\
    content\t\t Update the current model content\n\
    docupdate\t Update the content of a documentation node \n\
    trycom\t\t Update the comment for a try node\n\
    res [val]\t Update the result for a try node\n\
    label\t\t Update the current node label\n\
    del [id]\t Delete a node";

    println!("{}", help);
    print_cursor(current_nodes);
}

fn print_title() {
    println!("{clear}{goto}{red}STUDY APP{reset}",
             clear = clear::All,
             goto = cursor::Goto(4,1),
             red   = color::Fg(color::LightGreen),
             reset = color::Fg(color::Reset));
}

fn print_header(current_nodes: &Vec<Node>) {
    let (x, _y) =terminal_size().unwrap();

    // Print first line
    println!("{goto}", goto = cursor::Goto(4,1) );
    let line: String = iter::repeat("=").take(x.into()).collect();
    println!("{}", line);


    let mut header = "".to_string();

    for node in current_nodes.iter() {

        match node.node_type {
            x if x == NodeType::Question as i32 => header += "[?]",
            x if x == NodeType::Documentation as i32 => header += "[D]",
            x if x == NodeType::Model as i32 => header += "[M]",
            x if x == NodeType::Term as i32 => header += "[T]",
            x if x == NodeType::TryNode as i32 => header += "[Ty]",
            x if x == NodeType::Subquestion as i32 => header += "[?]",
            _ => header += "",
        };

       header += &node.label.trim(); 
       header.push(','); 
       header.push(' '); 
    }

    println!("{}", header);
    // Print second line
    println!("{}", line);
}

fn print_content() {
    println!("{goto}{reset}",
             goto = cursor::Goto(1,4),
             reset = color::Fg(color::Reset));
}

fn print_all_with_content(content: &str, current_nodes: &Vec<Node>) {
    print_title();
    print_header(current_nodes);
    print_content();
    println!("{}", content);
    print_cursor(current_nodes);
}

fn print_cursor_with_text(text: &str) {
    let (_x, y) =terminal_size().unwrap();
    println!("{goto}[{text}]: ", goto = cursor::Goto(1,y - 1), text = text )
}

fn print_cursor(current_nodes: &Vec<Node>) {
    let (_x, y) =terminal_size().unwrap();
    
    if current_nodes.len() == 0 {
        println!("{goto}[-:] ", goto = cursor::Goto(1,y - 2));
    }
    else {
        match current_nodes.get( current_nodes.len() - 1 ) {
            None => println!("{goto}[-:] ", goto = cursor::Goto(1,y - 1)),
            Some(node) => println!("{goto}[{n}]: ", goto = cursor::Goto(1,y - 1), n = node.node_type ),
        }
    }
}

fn print_cursor_for_input(input_text: &str) {
    let (_x, y) =terminal_size().unwrap();
    println!("{goto}{input} - ",
             goto = cursor::Goto(1, y-1),
             input = input_text);
}


fn add_subquestion(current_nodes: &mut Vec<Node>, conn: &mut my::PooledConn ) {
    print_title();
    print_header(current_nodes);

    // Cer input de la user pentru intrebare
    print_cursor_for_input("Question");
    let mut question = String::new();
    stdin().read_line(&mut question).expect("Din not enter correct string");
    let question = question.trim(); 

    let parent_node_id = current_nodes.get( current_nodes.len() - 1 ).unwrap().node_id;

    db_operations::add_subquestion(&question, parent_node_id, conn);

    // Updatez current_nodes
    current_nodes.pop();
    match db_operations::get_node( parent_node_id, conn ) {
        None => {},
        Some(updated_node) => current_nodes.push(updated_node),
    };

    print_all_with_content("Question saved", current_nodes);
}


fn ask_question(current_nodes: &mut Vec<Node>, conn: &mut my::PooledConn ) {
    print_title();
    print_header(current_nodes);

    // Cer input de la user pentru intrebare
    print_cursor_for_input("Question");
    let mut question = String::new();
    stdin().read_line(&mut question).expect("Din not enter correct string");
    question = question.trim().to_string(); 
    db_operations::save_question( &question, conn );

    print_all_with_content("question saved", current_nodes);
}


/// Dupa operatia asta raman in acelasi nod pentru a putea adauga in continuare daca este cazul
fn add_documentation( conn: &mut my::PooledConn, current_nodes: &mut Vec<Node> ) {
    let parent_node_id = current_nodes.get( current_nodes.len() - 1 ).unwrap().node_id;

    // Cer input de la user pentru label
    print_title();
    print_header(current_nodes);
    print_cursor_for_input("Doc label");
    let mut label = String::new();
    stdin().read_line(&mut label).expect("Din not enter correct string");

    // Cer input de la user pentru content
    print_title();
    print_header(current_nodes);
    print_cursor_for_input("Doc content. Content il iau din /tmp/study.txt. Ready?");

    let mut answer = String::new();
    stdin().read_line(&mut answer).expect("Did not enter correct string");
    answer = answer.trim().to_owned();

    if answer != "y" {
        print_all_with_content("Canceled", current_nodes);
        return;
    }

    let filename = "/tmp/study.txt";
    let mut documentation = fs::read_to_string(filename).expect("Cannot read the file");
    documentation = documentation.replace("\"", "\\\"");
    documentation = documentation.trim().to_string(); 

    db_operations::save_documentation(&label, &documentation, parent_node_id, conn);

    // Updatez nodul curent
    current_nodes.pop();
    match db_operations::get_node( parent_node_id, conn ) {
        None => {},
        Some(updated_node) => current_nodes.push(updated_node),
    };

    print_all_with_content("Doc saved", current_nodes);
}

/// Adaug model la ultimul nod in care am intrat. Nu intru in noul nod, dupa ce il adaug
fn add_model( conn: &mut my::PooledConn, current_nodes: &mut Vec<Node> ) {
    let parent_node_id = current_nodes.get( current_nodes.len() - 1 ).unwrap().node_id;

    print_title();
    print_header(current_nodes);
    print_cursor_for_input("Model label. Content il iau din /tmp/study.txt ?");
    let mut label = String::new();
    stdin().read_line(&mut label).expect("Did not enter correct string");
    label = label.trim().to_string();

    let filename = "/tmp/study.txt";
    let mut model = fs::read_to_string(filename).expect("Cannot read the file");
    model = model.replace("\"", "\\\"");

    db_operations::save_model(&label, &model, parent_node_id, conn);

    // Updatez nodul curent
    current_nodes.pop();
    match db_operations::get_node( parent_node_id, conn ) {
        None => {},
        Some(updated_node) => current_nodes.push(updated_node),
    };

    print_all_with_content("Model saved", current_nodes);
}

fn add_new_term( conn: &mut my::PooledConn, current_nodes: &mut Vec<Node> ) {
    let parent_node_id = current_nodes.get( current_nodes.len() - 1 ).unwrap().node_id;

    // Cer input de la user pentru noul termen. term va fi si labelul nodului
    print_title();
    print_header(current_nodes);
    print_cursor_for_input("Term");
    let mut term = String::new();
    stdin().read_line(&mut term).expect("Did not enter correct string");
    term = term.trim().to_string();

    db_operations::save_term(&term, parent_node_id, conn);

    // Updatez current_nodes
    current_nodes.pop();
    match db_operations::get_node( parent_node_id, conn ) {
        None => {},
        Some(updated_node) => current_nodes.push(updated_node),
    };

    print_all_with_content("Term saved", current_nodes);
}


fn add_explanation( conn: &mut my::PooledConn, current_nodes: &mut Vec<Node> ) {
    if current_nodes.get( current_nodes.len() - 1 ).unwrap().node_type != NodeType::Term as i32  {
        print_all_with_content("Not in a TERM node!", current_nodes);
        return;
    }

    let term_id = current_nodes.get( current_nodes.len() - 1 ).unwrap().node_id;

    print_title();
    print_header(current_nodes);
    print_cursor_for_input("New explanation:");
    let mut explanation = String::new();
    stdin().read_line(&mut explanation).expect("Did not enter correct string");
    explanation = explanation.trim().to_string();

    db_operations::update_explanation(&explanation, term_id, conn); 

    // Updatez current_nodes
    current_nodes.pop();
    match db_operations::get_node( term_id, conn ) {
        None => {},
        Some(updated_node) => current_nodes.push(updated_node),
    };

    print_all_with_content("Term updated", current_nodes);
}


fn add_new_try( conn: &mut my::PooledConn, current_nodes: &mut Vec<Node> ) {
    let parent_node_id = current_nodes.get( current_nodes.len() - 1 ).unwrap().node_id;

    // Cer input de la user pentru noul try node pentru label-ul nodului
    print_title();
    print_header(current_nodes);
    print_cursor_for_input("Try label");
    let mut try_node = String::new();
    stdin().read_line(&mut try_node).expect("Did not enter correct string");
    try_node = try_node.trim().to_string();

    db_operations::save_try(&try_node, parent_node_id, conn);

    // Updatez current_nodes
    current_nodes.pop();
    match db_operations::get_node( parent_node_id, conn ) {
        None => {},
        Some(updated_node) => current_nodes.push(updated_node),
    };

    print_all_with_content("Try saved", current_nodes);
}


fn update_try_comment( conn: &mut my::PooledConn, current_nodes: &mut Vec<Node> ) {

    if current_nodes.get( current_nodes.len() - 1 ).unwrap().node_type != NodeType::TryNode as i32  {
        print_all_with_content("Not in a TRY node!", current_nodes);
        return;
    }

    print_title();
    print_header(current_nodes);
    print_cursor_for_input("Try comment. Continutul il iau din /tmp/study.txt ?[y]:");
    let mut answer = String::new();
    stdin().read_line(&mut answer).expect("Did not enter correct string");
    answer = answer.trim().to_owned();

    if answer == "y" {
        let filename = "/tmp/study.txt";
        let mut comment = fs::read_to_string(filename).expect("Cannot read the file");
        comment = comment.replace("\"", "\\\"");

        let try_id = current_nodes.get( current_nodes.len() - 1 ).unwrap().node_id;

        db_operations::update_try_comment(&comment, try_id, conn); 

        // Updatez current_nodes
        current_nodes.pop();
        match db_operations::get_node( try_id, conn ) {
            None => {},
            Some(updated_node) => current_nodes.push(updated_node),
        };

    print_all_with_content("Try updated", current_nodes);

    }
    else {
        print_all_with_content("Canceled", current_nodes);
    }
}

fn update_try_result( argument: &str, conn: &mut my::PooledConn, current_nodes: &mut Vec<Node> ) {

    if current_nodes.len() < 1 && current_nodes.get( current_nodes.len() - 1 ).unwrap().node_type != NodeType::TryNode as i32  {
        print_all_with_content("Not in a TRY node!", current_nodes);
        return;
    }

    let node_id = current_nodes.get( current_nodes.len() - 1 ).unwrap().node_id;

    match argument.parse::<i32>() {
        Err(_e) => {},
        Ok(result) => {
            if result == 0 || result == 1 {
                db_operations::update_try_result(result, node_id, conn);
            }
        },
    }

    list_all_nodes_tree(current_nodes, conn);
    print_cursor_with_text("Try updated");

}


fn update_node_label( conn: &mut my::PooledConn, current_nodes: &mut Vec<Node> ) {
    // Sa am selectat un nod care nu este intrebarea principala
    if current_nodes.len() < 2 {
        print_all_with_content("A node that is not a main question must be selected", current_nodes);
        return;
    }

    // Iau noul label de la command line
    print_title();
    print_header(current_nodes);
    print_cursor_for_input("New node label:");
    let mut label = String::new();
    stdin().read_line(&mut label).expect("Did not enter correct string");
    let label = label.trim();

    let node_id = current_nodes.get( current_nodes.len() - 1 ).unwrap().node_id;
    db_operations::update_node_label(label, node_id, conn);

    // Updatez current_nodes
    current_nodes.pop();
    match db_operations::get_node( node_id, conn ) {
        None => {},
        Some(updated_node) => current_nodes.push(updated_node),
    };

    print_all_with_content("Node updated", current_nodes);
}


fn move_node(arguments: &str,  conn: &mut my::PooledConn, current_nodes: &mut Vec<Node> ) {
    let args: Vec<i32> = arguments.split(" ")
            .map(|id| id.parse::<i32>() )                   // parsez valoare string a id-ului
            .map(|id_parsed| id_parsed.unwrap_or(-1) )      // dupa parsare rezulta Result. daca este Err, atunci o scot -1
            .collect();

    // Verific formatul comenzii
    if args.len() < 2 {
        print_all_with_content("Not enough ids. Format of copy command is copy [id_to_copy] [into_parent_id]", current_nodes);
        return;
    }

    let node_to_copy = args[0];
    let parent_node = args[1];

    // Verific sa nu fiu in nodul copiat
    if current_nodes.len() > 1 && current_nodes.get( current_nodes.len() - 1 ).unwrap().node_id == node_to_copy {
        print_all_with_content("Cannot copy current node", current_nodes);
        return;
    }

    db_operations::move_node_to_parent(node_to_copy, parent_node, conn);

    list_all_nodes_tree(current_nodes, conn);
}


fn update_model_content( conn: &mut my::PooledConn, current_nodes: &mut Vec<Node> ) {
    // Verific sa am selectat cel putin un nod
    if current_nodes.len() == 0 {
        print_all_with_content("Current node must be of type MODEL", current_nodes);
        return;
    }

    // Verific sa fiu intr-un nod de tip model
    if current_nodes.get( current_nodes.len() - 1 ).unwrap().node_type != NodeType::Model as i32  {
        print_all_with_content("Current node must be of type MODEL", current_nodes);
        return;
    }

    print_title();
    print_header(current_nodes);
    print_cursor_for_input("New model content il iau din /tmp/study.txt ?[y]");
    let mut answer = String::new();
    stdin().read_line(&mut answer).expect("Did not enter correct string");
    answer = answer.trim().to_owned();

    if answer == "y" {
        let filename = "/tmp/study.txt";
        let mut model = fs::read_to_string(filename).expect("Cannot read the file");
        model = model.replace("\"", "\\\"");

        let node_id = current_nodes.get( current_nodes.len() - 1 ).unwrap().node_id;

        db_operations::update_model_content(&model, node_id, conn);

        // Updatez nodul curent
        current_nodes.pop();
        match db_operations::get_node( node_id, conn ) {
            None => {},
            Some(updated_node) => current_nodes.push(updated_node),
        };

        print_all_with_content("Model saved", current_nodes);
    }
    else {
        print_all_with_content("Canceled", current_nodes);
    }

}


fn update_documentation_content( conn: &mut my::PooledConn, current_nodes: &mut Vec<Node> ) {
    // Verific sa am selectat cel putin un nod
    if current_nodes.len() == 0 {
        print_all_with_content("Current node must be of type DOCUMENTATION", current_nodes);
        return;
    }

    // Verific sa fiu intr-un nod de tip model
    if current_nodes.get( current_nodes.len() - 1 ).unwrap().node_type != NodeType::Documentation as i32  {
        print_all_with_content("Current node must be of type DOCUMENTATION", current_nodes);
        return;
    }

    print_title();
    print_header(current_nodes);
    print_cursor_for_input("New documentation content il iau din /tmp/study.txt ?[y]");
    let mut answer = String::new();
    stdin().read_line(&mut answer).expect("Did not enter correct string");
    answer = answer.trim().to_owned();

    if answer == "y" {
        let filename = "/tmp/study.txt";
        let mut documentation = fs::read_to_string(filename).expect("Cannot read the file");
        documentation = documentation.replace("\"", "\\\"");

        let node_id = current_nodes.get( current_nodes.len() - 1 ).unwrap().node_id;

        db_operations::update_documentation_content(&documentation, node_id, conn);

        // Updatez nodul curent
        current_nodes.pop();
        match db_operations::get_node( node_id, conn ) {
            None => {},
            Some(updated_node) => current_nodes.push(updated_node),
        };

        print_all_with_content("Documentation saved", current_nodes);
    }
    else {
        print_all_with_content("Canceled", current_nodes);
    }

}


fn delete_node( argument: &str, conn: &mut my::PooledConn, current_nodes: &mut Vec<Node> ) {
    let node_to_delete: i32 = argument.parse().expect("That was not a number");

    // Verific sa nu fie nodul curent
    let current_node_id = current_nodes.get( current_nodes.len() - 1 ).unwrap().node_id;
    if current_node_id == node_to_delete {
        print_all_with_content("Cannot delete current node", current_nodes);
        return;
    }

    // Verific sa nu aiba copii
    if current_nodes.get( current_nodes.len() - 1 ).unwrap().child_nodes.len() > 0 {
        print_all_with_content("Cannot delete a node with children", current_nodes);
        return;
    }
    
    db_operations::delete_node(node_to_delete, conn);

    // Updatez current_nodes
    current_nodes.pop();
    match db_operations::get_node( current_node_id, conn ) {
        None => {},
        Some(updated_node) => current_nodes.push(updated_node),
    };

    list_all_nodes_tree(current_nodes, conn);
}


fn list_questions(current_nodes: &mut Vec<Node>, conn: &mut my::PooledConn ) {
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

    print_title();
    print_header(current_nodes);
    print_content();

    let _v: Vec<_> = questions.iter().map( |q| println!("{}. {}", q.node_id, q.question_text) ).collect();

    print_cursor(current_nodes);
}

fn list_node(current_nodes: &mut Vec<Node>, conn: &mut my::PooledConn ) {
    if current_nodes.len() == 0 {
        print_all_with_content("No current selection", current_nodes );
        return;
    }


    print_title();
    print_header(current_nodes);
    print_content();

    let current_node = current_nodes.get( current_nodes.len() - 1 );

    // Printez nodul parinte
    println!("Current node: {}", current_node.unwrap().label );

    // Tree sunt liniile de la inceput care arata ramificatiile
    let tree: String = iter::repeat(" |__ ").take(1).collect();
    
    // Printez ce copii are
    current_node.unwrap().child_nodes.split(" ").for_each(|child_node_id| {
        match child_node_id.parse::<i32>() {
            Ok(n_id) => {
                let child_node = db_operations::get_node( n_id, conn );
                match child_node {
                    Some(mut cn) => { println!("{}{}", tree, &cn.to_string()) },
                    None => {}
                }
            },
            Err(_e) => {}
        }
    });

    print_cursor(current_nodes);
}

fn list_all_nodes_tree(current_nodes: &mut Vec<Node>, conn: &mut my::PooledConn ) {
    if current_nodes.len() == 0 {
        print_all_with_content("No question selected", current_nodes );
        return;
    }

    let level = 0;
    let question = current_nodes.get(0).unwrap();
    let current_node_id = current_nodes.get( current_nodes.len() - 1).unwrap().node_id;

    print_title();
    print_header(current_nodes);
    print_content();
    println!("[Q]{}", question.label);
    
    print_children_at_level(&question.child_nodes, level, conn, current_node_id);
    print_cursor(current_nodes);
}

fn print_children_at_level(child_ids: &str, level: i32, conn: &mut my::PooledConn, current_node_id: i32 ) {
    let space: String = iter::repeat("    ").take(level as usize).collect();

    child_ids.split(" ").for_each(| child_node_id | {
        match child_node_id.parse::<i32>() {
            Ok(n_id) => {
                let child_node = db_operations::get_node( n_id, conn );
                match child_node {
                    Some(mut cn) => { 
                        let is_current = if cn.node_id == current_node_id {
                            true
                        }
                        else {
                            false
                        };

                        print_line_with_colors(&space, &cn.to_short_string(), cn.node_type, is_current);

                        // Daca are copii, execut recursiv functia asta
                        if cn.child_nodes.len() > 0 {
                            print_children_at_level(&cn.child_nodes, level + 1, conn, current_node_id);
                        }
                    },
                    None => {}
                }
            },
            Err(_e) => {}
        }
    });
}

fn print_line_with_colors(space: &str, desc: &str, node_type: i32, is_current: bool) {
    if is_current {
        println!(" {sp}  |__ {color}{desc}{reset}", 
                sp = space,
                color = color::Fg(color::Red),
                desc = desc,
                reset = color::Fg(color::Reset));
        return
    }

    match node_type {
        x if x == NodeType::Documentation as i32 => {
            println!(" {sp}  |__ {color}{desc}{reset}", 
                    sp = space,
                    color = color::Fg(color::LightGreen),
                    desc = desc,
                    reset = color::Fg(color::Reset));
        },
        x if x == NodeType::Model as i32 => {
            println!(" {sp}  |__ {color}{desc}{reset}", 
                    sp = space,
                    color = color::Fg(color::LightBlue),
                    desc = desc,
                    reset = color::Fg(color::Reset));
        },
        x if x == NodeType::TryNode as i32 => {
            println!(" {}  |__ {}", space, desc);
        },
        x if x == NodeType::Subquestion as i32 => {
            println!(" {sp}  |__ {color}{desc}{reset}", 
                    sp = space,
                    color = color::Fg(color::LightCyan),
                    desc = desc,
                    reset = color::Fg(color::Reset));
        },
        _ => {
            println!(" {}  |__ {}", space, desc);
        },
    };
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

    //print_title();
    //print_header(current_nodes);
    //print_cursor(current_nodes);
    
    list_all_nodes_tree(current_nodes, conn);

    let mut first_question_iter = questions.drain(0..1);
    return first_question_iter.next();
}


fn select_node( argument: &str, conn: &mut my::PooledConn, current_nodes: &mut Vec<Node>) {
    let node_id: i32 = argument.parse().expect("That was not a number");

    // Daca nu am selectat o intrebare sa nu pot intra in nici un nod
    if current_nodes.len() == 0 {
        print_all_with_content( "Question is not selected", current_nodes );
        return;
    }

    // Verific daca nodul selectat este unul din copiii nodului curent
    if current_nodes.len() > 0 {
        let child_count = current_nodes.get( current_nodes.len() - 1).unwrap().child_nodes.split(" ")
            .map(|child_node_id| child_node_id.parse::<i32>() ) // parsez valoare string a id-ului
            .map( |id_parsed| id_parsed.unwrap_or(-1) )         // dupa parsare rezulta Result. daca este Err, atunci o scot -1
            .filter(|val| val == &node_id )                     // Compar cu valoarea selectata de user
            .count();
            
        if child_count == 0 {
            print_all_with_content( &("Current node has no child with this id ".to_owned() + argument), current_nodes );
            return;
        }
    }


    match db_operations::get_node( node_id, conn ) {
        None => { 
            print_title();
            print_header(current_nodes);
            print_content();
            println!("Node {} not found", node_id);
            print_cursor(current_nodes);
        },
        Some(node) => {
            current_nodes.push(node);
            //show_node_content(conn, current_nodes);
            list_all_nodes_tree(current_nodes, conn);
        },
    }
    
}

fn move_out_current_node(conn: &mut my::PooledConn, current_nodes: &mut Vec<Node>) {
    if current_nodes.len() < 2 {
        print_all_with_content( "Cannot move out. Not enough nodes depth", current_nodes );
        return;
    }
    current_nodes.pop();
    list_all_nodes_tree(current_nodes, conn);
}

fn move_to_lateral_node(argument: &str, conn: &mut my::PooledConn, current_nodes: &mut Vec<Node>) {
    let node_to_move: i32 = argument.parse().expect("That was not a number");

    if current_nodes.len() < 2 {
        print_all_with_content( "Cannot move down. Not enough nodes depth", current_nodes );
        return;
    }

    // Verific daca nodul selectat este unul din copiii nodului curent
    //let current_node_id = current_nodes.get( current_nodes.len() - 1 ).unwrap().node_id;

    let parent_children: Vec<i32> = current_nodes.get( current_nodes.len() - 2 ).unwrap().child_nodes.split(" ")
            .map(|id| id.parse::<i32>() )                   // parsez valoare string a id-ului
            .map(|id_parsed| id_parsed.unwrap_or(-1) )      // dupa parsare rezulta Result. daca este Err, atunci o scot -1
            .filter(|id| *id != -1)
            .collect();

    match parent_children.iter().position( |id| id == &node_to_move ) {
        None => {
            print_all_with_content( &("That node is not at the same level as the current one ".to_owned() + argument),  current_nodes );
        },
        Some(_index) => {
            current_nodes.pop();
            select_node(argument, conn, current_nodes);
        },
    }
}

fn show_node_content(conn: &mut my::PooledConn, current_nodes: &mut Vec<Node>) {
    let current_node = current_nodes.get( current_nodes.len() - 1 ).unwrap();

    // Functie de ce tip este, caut un tabela corespunzatoare contentul si il afisez in sectiunea
    // content
    match current_node.node_type {
        x if x == NodeType::Question as i32 => { 
            match db_operations::get_question(current_node.node_id, conn) {
                None => print_all_with_content("Question not found", current_nodes ),
                Some(question) => print_all_with_content( &question.question_text, current_nodes ),
            }
        },
        x if x == NodeType::Documentation as i32 => {
            match db_operations::get_documentation(current_node.node_id, conn) {
                None => print_all_with_content("Documentation not found", current_nodes ),
                Some(documentation) => print_all_with_content( &documentation.content, current_nodes ),
            }
        },
        x if x == NodeType::Model as i32 => {
            match db_operations::get_model(current_node.node_id, conn) {
                None => print_all_with_content( "Model not found", current_nodes ),
                Some(model) => print_all_with_content( &model.content, current_nodes ),
            }
        },
        x if x == NodeType::Term as i32 => {
            match db_operations::get_term(current_node.node_id, conn) {
                None => print_all_with_content( "Term not found", current_nodes ),
                Some(term) => {
                    let mut content = term.term;
                    content += " = ";
                    content += &term.explanation;
                    print_all_with_content( &content, current_nodes );
                },
            }
        },
        x if x == NodeType::TryNode as i32 => {
            match db_operations::get_try(current_node.node_id, conn) {
                None => print_all_with_content( "Try node not found", current_nodes ),
                Some(try_node) => {
                    let mut content = "".to_string();
                    if try_node.result == 1 {
                        content += "SUCCESS\n";
                    }
                    else {
                        content += "FAILED\n";
                    }
                    content += &try_node.comment;
                    print_all_with_content( &content, current_nodes );
                },
            }
        },
        x if x == NodeType::Subquestion as i32 => { 
            match db_operations::get_question(current_node.node_id, conn) {
                None => print_all_with_content("Question not found", current_nodes ),
                Some(question) => print_all_with_content( &question.question_text, current_nodes ),
            }
        },
        _ => {},
    };
}
