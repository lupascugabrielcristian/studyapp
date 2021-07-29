#[derive(Debug, PartialEq, Eq)]
pub struct Question {
    pub node_id: i32,
    pub question_text: String,
}

struct Documentation {
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
        let types: [&str; 4] = ["?", "Doc", "Model", "."];

        let mut describe: String = "[:type] ".to_string();
        describe += &self.label.trim();
        describe = describe.replace(":type", types[self.node_type as usize]);

        return describe;
    }
}

// Pentru a salva in baza de date folosesc valorile:
// question: 0,
// documentation: 1, 
// model: 2, 
// new term: 3
// try: 4
pub enum Location {
    Question,
    Documentation,
    Model,
    //NewTerm,
    //Try,
    Initial
}
