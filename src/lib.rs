use tree_sitter::Parser;
use pyo3::prelude::*;

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
    //"text": node.utf8_text(source.as_bytes()).unwrap(),
    "children": children,
  })
}

#[pyfunction]
fn parse_verilog(s: &str) -> PyResult<String> {
  let mut parser = Parser::new();
  parser.set_language(&tree_sitter_systemverilog::LANGUAGE.into()).expect("Error loading Rust grammar");
  let tree = parser.parse(s, None).unwrap();
  let root_node = tree.root_node();
  let j = node_to_json(s, &root_node);
  Ok(serde_json::to_string(&j).unwrap())
}

#[pyfunction]
fn parse_verilog_pretty(s: &str) -> PyResult<String> {
  let mut parser = Parser::new();
  parser.set_language(&tree_sitter_systemverilog::LANGUAGE.into()).expect("Error loading Rust grammar");
  let tree = parser.parse(s, None).unwrap();
  let root_node = tree.root_node();
  let j = node_to_json(s, &root_node);
  Ok(serde_json::to_string_pretty(&j).unwrap())
}

#[pymodule]
fn silly_python_treesitter(m: &Bound<'_, PyModule>) -> PyResult<()> {
  m.add_function(wrap_pyfunction!(parse_verilog, m)?)?;
  m.add_function(wrap_pyfunction!(parse_verilog_pretty, m)?)?;
  Ok(())
}

