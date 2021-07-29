#[derive(Debug, PartialEq, Eq)]
pub struct Question {
    pub node_id: i32,
    pub question_text: String,
}

pub struct Documentation {
    pub node_id: i32,
    pub content: String,
}
 
pub struct Model {
    pub node_id: i32,
    pub content: String,
}

pub struct Node {
    pub node_id: i32,
    pub node_type: i32,
    pub child_nodes: String,
    pub parent_node: i32,
    pub label: String
}

impl Node {
    pub fn to_string(&mut self) -> String {
        let types: [&str; 4] = ["?", "Doc  ", "Model", "."];

        let mut describe: String = "[:type] ".to_string();
        describe += "[:id] ";
        describe = describe.replace(":id", &self.node_id.to_string());
        describe += &self.label.trim();
        describe = describe.replace(":type", types[self.node_type as usize]);

        return describe;
    }
}

pub enum NodeType {
    Question = 0,
    Documentation = 1,
    Model = 2,
}


// Pentru a salva in baza de date folosesc valorile:
// question: 0,
// documentation: 1, 
// model: 2, 
// new term: 3
// try: 4
