use am_core::{Blueprint, Node, Value};

#[test]
fn node() {
    let bp = Blueprint::new("Hitbox");
    let mut node = Node::new(&bp);
    assert_eq!(node.name(), "Hitbox");

    node.set_property("Damage", Value::Int(10));
    assert_eq!(node.get_property("Damage"), Some(&Value::Int(10)));

    node.add_child(Node::new(&bp).with_name("Child"));
    assert_eq!(node.children().len(), 1);
}

#[test]
fn with_name() {
    let bp = Blueprint::new("");
    let node = Node::new(&bp).with_name("Hitbox");
    assert_eq!(node.name(), "Hitbox");
}

#[test]
fn with_property() {
    let bp = Blueprint::new("Animation").with_property("Length", Value::Int(21));
    let node = Node::new(&bp).with_property("Repeat", Value::Bool(true));
    assert_eq!(node.name(), "Animation");
    assert_eq!(node.get_property("Length"), Some(&Value::Int(21)));
    assert_eq!(node.get_property("Repeat"), Some(&Value::Bool(true)));
}
