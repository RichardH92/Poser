mod node;
mod domain;

fn main() {
    let _e = domain::entity::Entity { id: 1, x_coordinate: 0.0, y_coordinate: 1.0, z_coordinate: 3.0 };
    
    //let mut node = node::Node::new();

    //node.addEntities(vec![e]);
    // Statements here are executed when the compiled binary is called

    // Print text to the console
    println!("Hello World!");
}
