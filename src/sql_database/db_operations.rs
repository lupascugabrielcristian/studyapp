use mysql as my;

use crate::sql_database::models::{ Node, Question, Documentation, Model, Term, TryNode };

pub fn connect() -> my::PooledConn {
    let pool = my::Pool::new("mysql://root:studymqsql@localhost:3306/mysql").unwrap();
    let mut conn = pool.get_conn().unwrap();

    // Questions table
    // | node_id | question_text |
    if check_for_table(&mut conn, "\'questions\'") == false {
        println!("Creating table questions");
        let create_table = "CREATE TABLE questions ( node_id INT NOT NULL, question_text TEXT not null)";
        pool.prep_exec(create_table, ()).unwrap();
    }

    // Nodes table
    // | node_id* | node_type | child_nodes | parent_node | label |
    // Child_nodes va fi un string care contine id-urile nodurilor copii, separate cu spatiu ' '
    // label va fi folosit pentru cursor
    if check_for_table(&mut conn, "\'nodes\'") == false {
        println!("Creating nodes table");
        let create_table = "CREATE TABLE nodes (node_id int NOT NULL AUTO_INCREMENT PRIMARY KEY, node_type INT not null, child_nodes TEXT, parent_node INT, label TEXT NOT NULL )";
        pool.prep_exec(create_table, ()).unwrap();
    }

    // Documentation table
    // | node_id | content |
    if check_for_table(&mut conn, "\'documentation\'") == false {
        println!("Creating documentation table");
        let create_table = "CREATE TABLE documentation ( node_id int NOT NULL, content TEXT NOT NULL)";
        pool.prep_exec(create_table, ()).unwrap();
    }


    // Models table
    // | node_id | content |
    if check_for_table(&mut conn, "\'models\'") == false {
        println!("Creating models table");
        let create_table = "CREATE TABLE models ( node_id int NOT NULL, content TEXT NOT NULL)";
        pool.prep_exec(create_table, ()).unwrap();
    }

    // New Term table
    // | node_id | term | explanation |
    if check_for_table(&mut conn, "\'terms\'") == false {
        println!("Creating terms table");
        let create_table = "CREATE TABLE terms ( node_id int NOT NULL, term TEXT NOT NULL, explanation TEXT)";
        pool.prep_exec(create_table, ()).unwrap();
    }

    // Tries table
    // | node_id | result | comment |
    // result va fi 0 sau 1
    if check_for_table(&mut conn, "\'tries\'") == false {
        println!("Creating tries table");
        let create_table = "CREATE TABLE tries ( node_id int NOT NULL, result int, comment TEXT)";
        pool.prep_exec(create_table, ()).unwrap();
    }

    return conn;
}

pub fn get_node( node_id: i32, conn: &mut my::PooledConn) -> Option<Node> {

    let query = "SELECT * FROM nodes WHERE node_id=':node_id'";
    let query = query.replace(":node_id", &node_id.to_string() );

    let mut found_nodes: Vec<Node> =
    conn.prep_exec(query, ()).map( |result| {
        result.map(|x| x.unwrap()).map(|row| {
            let ( node_id, node_type, child_nodes, parent_node, label ) = my::from_row(row);
            Node {
                node_id: node_id,
                node_type: node_type,
                child_nodes: child_nodes,
                parent_node: parent_node,
                label: label
            }
        }).collect()
    }).unwrap();

    if found_nodes.len() == 0 {
        return None
    }

    return found_nodes.drain(0..1).next();
}

pub fn save_question( question_text: &String, conn: &mut my::PooledConn ) {
    // Salvez in nodes table, 
    // iau node_id, cu care s-a salvat, 
    // si salvez in questions table cu aces node_id
    let add_node_query = "INSERT INTO mysql.nodes ( node_type, child_nodes, parent_node, label ) VALUES(\"0\", \"\", -1, \":label\"); SELECT LAST_INSERT_ID() INTO @id; INSERT INTO mysql.questions( node_id, question_text ) VALUES( @id, \":question_text\")";
    let add_node_query = add_node_query.replace(":question_text", &question_text);
    let add_node_query = add_node_query.replace(":label", &question_text);
    conn.start_transaction(false, None, None)
        .and_then(|mut t| {
            t.query(add_node_query).unwrap();
            t.commit()
        }).unwrap();

}

pub fn save_documentation( label: &String, documentation: &String, parent_id: i32, conn: &mut my::PooledConn ) {
    // Salvez in nodes tables si folosesc id-ul generat acolo ca sa salvez in documentation table
    // Updatez nodul parinte sa adaug acest nod ca si child
    let add_node_query = "INSERT INTO mysql.nodes ( node_type, child_nodes, parent_node, label ) VALUES(\"1\", \"\", \":parent_id\", \":label\");
                        SELECT LAST_INSERT_ID() INTO @id;
                        INSERT INTO mysql.documentation( node_id, content ) VALUES( @id, \":content\");
                        UPDATE mysql.nodes SET child_nodes=CONCAT(child_nodes,' ',@id) WHERE node_id=\":p_i_d\"";
    let add_node_query = add_node_query.replace(":parent_id", &parent_id.to_string());
    let add_node_query = add_node_query.replace(":label", &label);
    let add_node_query = add_node_query.replace(":content", &documentation);
    let add_node_query = add_node_query.replace(":p_i_d", &parent_id.to_string());


    conn.start_transaction(false, None, None)
        .and_then(|mut t| {
            t.query(add_node_query).unwrap();
            t.commit()
        }).unwrap();
}


pub fn save_model( label: &String, documentation: &String, parent_id: i32, conn: &mut my::PooledConn ) {
    // Salvez in nodes tables si folosesc id-ul generat acolo ca sa salvez in models table
    // Updatez nodul parinte sa adaug acest nod ca si child node
    let add_node_query = "INSERT INTO mysql.nodes ( node_type, child_nodes, parent_node, label ) VALUES(\"2\", \"\", \":parent_id\", \":label\");
                        SELECT LAST_INSERT_ID() INTO @id;
                        INSERT INTO mysql.models( node_id, content ) VALUES( @id, \":content\");
                        UPDATE mysql.nodes SET child_nodes=CONCAT(child_nodes,' ',@id) WHERE node_id=\":p_i_d\"";
    let add_node_query = add_node_query.replace(":parent_id", &parent_id.to_string());
    let add_node_query = add_node_query.replace(":label", &label);
    let add_node_query = add_node_query.replace(":content", &documentation);
    let add_node_query = add_node_query.replace(":p_i_d", &parent_id.to_string());

    conn.start_transaction(false, None, None)
        .and_then(|mut t| {
            t.query(add_node_query).unwrap();
            t.commit()
        }).unwrap();

}


pub fn save_term( term: &str, parent_id: i32, conn: &mut my::PooledConn ) {
    // Salvez in nodes table si folosesc id-ul generat acolo ca sa salvez in terms table
    // Updatez nodul parinte sa adaug acest nod ca si child node
    let add_node_query = "INSERT INTO mysql.nodes ( node_type, child_nodes, parent_node, label ) VALUES(\"3\", \"\", \":parent_id\", \":label\");
                        SELECT LAST_INSERT_ID() INTO @id;
                        INSERT INTO mysql.terms( node_id, term, explanation ) VALUES( @id, \":new_term\", \"\");
                        UPDATE mysql.nodes SET child_nodes=CONCAT(child_nodes,' ',@id) WHERE node_id=\":p_i_d\"";
    let add_node_query = add_node_query.replace(":parent_id", &parent_id.to_string());
    let add_node_query = add_node_query.replace(":label", term);
    let add_node_query = add_node_query.replace(":new_term", term);
    let add_node_query = add_node_query.replace(":p_i_d", &parent_id.to_string());

    conn.start_transaction(false, None, None)
        .and_then(|mut t| {
            t.query(add_node_query).unwrap();
            t.commit()
        }).unwrap();
}


pub fn save_try( try_node_label: &str, parent_id: i32, conn: &mut my::PooledConn ) {
    // Salvez in nodes table si folosesc id-ul generat acolo ca sa salvez in terms table
    // Updatez nodul parinte sa adaug acest nod ca si child node
    let add_node_query = "INSERT INTO mysql.nodes ( node_type, child_nodes, parent_node, label ) VALUES(\"4\", \"\", \":parent_id\", \":label\");
                        SELECT LAST_INSERT_ID() INTO @id;
                        INSERT INTO mysql.tries( node_id, result, comment ) VALUES( @id, -1, \"\");
                        UPDATE mysql.nodes SET child_nodes=CONCAT(child_nodes,' ',@id) WHERE node_id=\":parent_id\"";
    let add_node_query = add_node_query.replace(":parent_id", &parent_id.to_string());
    let add_node_query = add_node_query.replace(":label", try_node_label);

    conn.start_transaction(false, None, None)
        .and_then(|mut t| {
            t.query(add_node_query).unwrap();
            t.commit()
        }).unwrap();
}

pub fn update_explanation( explanation: &str, term_id: i32, conn: &mut my::PooledConn ) {
    let update_query = "UPDATE mysql.terms SET explanation=\":new_explanation\" WHERE node_id=\":term_id\"";
    let update_query = update_query.replace(":new_explanation", explanation );
    let update_query = update_query.replace(":term_id", &term_id.to_string());

    conn.start_transaction(false, None, None)
        .and_then(|mut t| {
            t.query(update_query).unwrap();
            t.commit()
        }).unwrap();
}

pub fn add_try_comment( comment: &str, try_id: i32, conn: &mut my::PooledConn ) {
    let update_query = "UPDATE mysql.tries SET comment=\":new_comment\" WHERE node_id=\":try_id\"";
    let update_query = update_query.replace(":new_comment", comment );
    let update_query = update_query.replace(":try_id", &try_id.to_string());

    conn.start_transaction(false, None, None)
        .and_then(|mut t| {
            t.query(update_query).unwrap();
            t.commit()
        }).unwrap();
}


pub fn get_question(node_id:i32, conn: &mut my::PooledConn) -> Option<Question> {
    let query = "SELECT * FROM questions WHERE node_id=':node_id'";
    let query = query.replace(":node_id", &node_id.to_string() );

    let mut found_questions: Vec<Question> =
    conn.prep_exec(query, ()).map( |result| {
        result.map(|x| x.unwrap()).map(|row| {
            let ( node_id, question_text ) = my::from_row(row);
            Question {
                node_id,
                question_text,
            }
        }).collect()
    }).unwrap();

    if found_questions.len() == 0 {
        return None
    }

    return found_questions.drain(0..1).next();
}


pub fn get_documentation(node_id:i32, conn: &mut my::PooledConn) -> Option<Documentation> {
    let query = "SELECT * FROM documentation WHERE node_id=':node_id'";
    let query = query.replace(":node_id", &node_id.to_string() );

    let mut found: Vec<Documentation> =
    conn.prep_exec(query, ()).map( |result| {
        result.map(|x| x.unwrap()).map(|row| {
            let ( node_id, content ) = my::from_row(row);
            Documentation {
                node_id,
                content,
            }
        }).collect()
    }).unwrap();

    if found.len() == 0 {
        return None
    }

    return found.drain(0..1).next();
}


pub fn get_model(node_id:i32, conn: &mut my::PooledConn) -> Option<Model> {
    let query = "SELECT * FROM models WHERE node_id=':node_id'";
    let query = query.replace(":node_id", &node_id.to_string() );

    let mut found: Vec<Model> =
    conn.prep_exec(query, ()).map( |result| {
        result.map(|x| x.unwrap()).map(|row| {
            let ( node_id, content ) = my::from_row(row);
            Model {
                node_id,
                content,
            }
        }).collect()
    }).unwrap();

    if found.len() == 0 {
        return None
    }

    return found.drain(0..1).next();
}

pub fn get_term(node_id: i32, conn: &mut my::PooledConn) -> Option<Term> {
    let query = "SELECT * FROM terms WHERE node_id=':node_id'";
    let query = query.replace(":node_id", &node_id.to_string() );

    let mut found: Vec<Term> =
    conn.prep_exec(query, ()).map( |result| {
        result.map(|x| x.unwrap()).map(|row| {
            let ( node_id, term, explanation ) = my::from_row(row);
            Term {
                node_id,
                term,
                explanation,
            }
        }).collect()
    }).unwrap();

    if found.len() == 0 {
        return None
    }

    return found.drain(0..1).next();
}


pub fn get_try(node_id: i32, conn: &mut my::PooledConn) -> Option<TryNode> {
    let query = "SELECT * FROM tries WHERE node_id=':node_id'";
    let query = query.replace(":node_id", &node_id.to_string() );

    let mut found: Vec<TryNode> =
    conn.prep_exec(query, ()).map( |result| {
        result.map(|x| x.unwrap()).map(|row| {
            let ( node_id, result, comment ) = my::from_row(row);
            TryNode {
                node_id,
                result,
                comment,
            }
        }).collect()
    }).unwrap();

    if found.len() == 0 {
        return None
    }

    return found.drain(0..1).next();
}


pub fn delete_node(node_id: i32, conn: &mut my::PooledConn ) {
    // Mai intai iau parent_id din tabela nodes, cu randul corespunzator node_id
    // Din tabela nodes sterg nodul cu id-ul node_id
    // Tot in tabela nodes, updatez nodul parinte, in care sterg nodul node_id, primit
    let delete_query = "SELECT parent_node INTO @id FROM mysql.nodes WHERE node_id=':node_id' LIMIT 1;
                        DELETE FROM mysql.nodes WHERE node_id=':node_id';
                        UPDATE mysql.nodes SET child_nodes=REPLACE(child_nodes, ':node_id', '') WHERE node_id=@id";


    let delete_query = delete_query.replace(":node_id", &node_id.to_string());

    conn.start_transaction(false, None, None)
        .and_then(|mut t| {
            t.query(delete_query).unwrap();
            t.commit()
        }).unwrap();
}

fn check_for_table(conn: &mut my::PooledConn, db_name: &str) -> bool {
    let query = "SELECT COUNT(*) from information_schema.tables where table_schema = database() and table_name = :db_name ";
    let query = query.replace(":db_name", db_name);
    // Numar tabelele "questions" din baza de data mysql, cea aleasa in url string
    // rows este QueryResult
    let rows = conn.prep_exec( query, ()).unwrap();

    let mut result: bool = false;

    for row_result in rows {
        // row_result este Result

        // row este valoarea Row
        let row = row_result.unwrap();

        // row.unwrap imi da un Val[Int(1)]
        // este de tip Vec<Value>
        let vec = row.unwrap();

        if  vec[0] == my::Value::from(1) {
            result = true;
            break;
        }
    }

    return result;
}

