use mysql as my;

use crate::sql_database::models::Node;

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
        let create_table = "CREATE TABLE models ( node_id int NOT NULL, content TEXT NULL)";
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

    println!("[+] question saved");
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

    println!("[+] doc saved");
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

    println!("[+] model saved");
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
