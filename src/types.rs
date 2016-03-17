use std::collections::HashMap;

struct Node {
    children: Vec<Node>,
    node_type: NodeType,
}

enum NodeType {
    Text(String),
    Element(ElementData),
}

struct ElementData {
    tag_name: String,
    attributes: AttrMap,
}


type AttrMap = HashMap<String, String>;

fn text(text: String) -> Node {
    Node {node_type: NodeType::Text(text), children: Vec::<Node>::new()}
}

#[test]
fn test_text_constructor() {
    let text_node = text("hi".to_string());
    assert!(match text_node.node_type {
        NodeType::Text(ref the_text) if the_text == &"hi".to_string() => true,
        _ => false,
    })
}

fn elem(name: String, attr: AttrMap, children: Vec<Node>) -> Node {
    Node {
        node_type: NodeType::Element(ElementData {
            tag_name: name,
            attributes: attr,
        }),
        children: children,
    }
}

#[test]
fn test_elem_constructor() {
    let attrs = AttrMap::new();
    let elem_node = elem("div".to_string(), attrs, Vec::new());
    assert!(match elem_node.node_type {
        NodeType::Element(ref element_data) if &element_data.tag_name == &"div".to_string() => true,
        _ => false,
    })
}
