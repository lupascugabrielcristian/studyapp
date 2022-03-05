#[derive(Debug, PartialEq, Eq)]
pub struct Question {
    pub node_id: i32,
    pub question_text: String,
    pub root_question: i32,
}

pub struct Documentation {
    pub node_id: i32,
    pub content: String,
}
 
pub struct Model {
    pub node_id: i32,
    pub content: String,
}

pub struct Term {
    pub node_id: i32,
    pub term: String,
    pub explanation: String,
}

pub struct TryNode {
    pub node_id: i32,
    pub result: i32,
    pub comment: String,
}

pub struct Node {
    pub node_id: i32,
    pub node_type: i32,
    pub child_nodes: String,
    pub parent_node: i32,
    pub parent_question: i32,
    pub label: String
}

pub enum NodeType {
    Question = 0,
    Documentation = 1,
    Model = 2,
    Term = 3,
    TryNode = 4,
    Subquestion = 5,
}


// Pentru a salva in baza de date folosesc valorile:
// question: 0,
// documentation: 1, 
// model: 2, 
// new term: 3
// try: 4
// subquestion: 5
