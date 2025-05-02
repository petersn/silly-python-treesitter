use tree_sitter::Parser;

fn node_to_json(source: &str, node: &tree_sitter::Node) -> serde_json::Value {
  let mut children = Vec::new();

  let mut cursor = node.walk();
  for child in node.children(&mut cursor) {
      children.push(node_to_json(source, &child));
  }

  serde_json::json!({
      "kind": node.kind(),
      "start_byte": node.start_byte(),
      "end_byte": node.end_byte(),
      // "text": node.utf8_text(source.as_bytes()).unwrap(),
      "children": children,
  })
}

// fn node_to_json(node: &tree_sitter::Node) -> serde_json::Value {
//   let mut obj = serde_json::Map::new();
//   obj.insert("type".to_string(), serde_json::Value::String(node.kind().to_string()));
//   obj.insert("start".to_string(), serde_json::Value::Number(serde_json::Number::from(node.start_byte() as i64)));
//   obj.insert("end".to_string(), serde_json::Value::Number(serde_json::Number::from(node.end_byte() as i64)));
//   let mut children = Vec::new();
//   node.children(
//   // obj.insert("children".to_string(), serde_json::Value::Array(node.children().iter().map(|child| node_to_json(child)).collect()));
//   serde_json::Value::Object(obj)
// }

fn main() {
  let mut parser = Parser::new();
  parser.set_language(&tree_sitter_systemverilog::LANGUAGE.into()).expect("Error loading Rust grammar");

  let s = "//asdf\nmodule foo();\nendmodule";
  let tree = parser.parse(s, None).unwrap();
  let root_node = tree.root_node();

  // assert_eq!(root_node.kind(), "source_file");
  // assert_eq!(root_node.start_position().column, 0);
  // assert_eq!(root_node.end_position().column, 12);

  let big_json = node_to_json(s, &root_node);

  println!("{}", serde_json::to_string_pretty(&big_json).unwrap());

  // println!("Got: {:?}", root_node);
  // let root_sexp = root_node.to_sexp();
  // println!("{:?}", root_sexp);
}
