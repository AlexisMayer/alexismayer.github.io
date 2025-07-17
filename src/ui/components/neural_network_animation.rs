use leptos::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq)]
struct Node {
    id: usize,
    x: f64,
    y: f64,
    layer_idx: usize, // Added layer index
    is_active: RwSignal<bool>,
}

#[derive(Clone, Copy, PartialEq)] // Added PartialEq for Connection
struct Connection {
    from: usize,
    to: usize,
    is_active: RwSignal<bool>, // Added activation state for connections
}

#[component]
pub fn NeuralNetworkAnimation() -> impl IntoView {
    let nodes = create_rw_signal(Vec::<Node>::new());
    let connections = create_rw_signal(Vec::<Connection>::new());

    // Define layers similar to the JS example
    let layer_counts = vec![3, 5, 2]; // Number of nodes per layer
    let layer_x_positions = vec![15.0, 50.0, 85.0]; // X-coordinates for each layer (relative to 0-100 viewBox)

    // Clone for use in the view! macro
    let layer_counts_for_view = layer_counts.clone();

    // Function to activate a node and propagate (recursive closure pattern)
    // We use Rc<RefCell<Option<...>>> to allow for recursive calls and interior mutability.
    let activate_and_propagate_rc: Rc<RefCell<Option<Rc<dyn Fn(usize, RwSignal<Vec<Node>>, RwSignal<Vec<Connection>>)>>>> =
        Rc::new(RefCell::new(None));

    // This `create_effect` is for generating the network structure
    create_effect(move |_| {
        let mut new_nodes = Vec::new();
        let mut new_connections = Vec::new();
        let mut id_counter = 0;

        // Clone layer_counts and layer_x_positions for this effect
        let lc_clone_effect = layer_counts.clone();
        let lx_clone_effect = layer_x_positions.clone();

        // Generate nodes
        for (layer_idx, &count) in lc_clone_effect.iter().enumerate() {
            let x = lx_clone_effect[layer_idx];
            let y_spacing = 100.0 / (count as f64 + 1.0); // Distribute nodes evenly vertically

            for i in 0..count {
                let y = y_spacing * (i as f64 + 1.0);
                new_nodes.push(Node {
                    id: id_counter,
                    x,
                    y,
                    layer_idx,
                    is_active: create_rw_signal(false),
                });
                id_counter += 1;
            }
        }

        // Generate connections with random skipping
        for layer_idx in 0..lc_clone_effect.len() - 1 {
            let current_layer_nodes: Vec<&Node> = new_nodes.iter()
                .filter(|n| n.layer_idx == layer_idx)
                .collect();

            let next_layer_nodes: Vec<&Node> = new_nodes.iter()
                .filter(|n| n.layer_idx == layer_idx + 1)
                .collect();

            for node1 in current_layer_nodes {
                for node2 in &next_layer_nodes { // Iterate over reference
                    // Randomly skip some connections (e.g., 25% chance to skip)
                    if js_sys::Math::random() > 0.25 {
                        new_connections.push(Connection { from: node1.id, to: node2.id, is_active: create_rw_signal(false) });
                    }
                }
            }
        }

        nodes.set(new_nodes);
        connections.set(new_connections);
    });

    // Define the recursive closure *after* nodes and connections are available
    // This closure needs to be defined using `Rc::new` and stored in the RefCell.
    let activate_and_propagate_rc_clone_for_recursive_call = Rc::clone(&activate_and_propagate_rc); // Clone for use inside the closure
    *activate_and_propagate_rc.borrow_mut() = Some(Rc::new(move |node_id: usize, nodes_signal: RwSignal<Vec<Node>>, connections_signal: RwSignal<Vec<Connection>>| {
        // Activate the current node
        if let Some(node) = nodes_signal.get().iter().find(|n| n.id == node_id) {
            node.is_active.set(true);
            let node_signal = node.is_active;
            set_timeout(move || node_signal.set(false), Duration::from_millis(1500)); // Deactivate after 1.5s
        }

        // Find outgoing connections
        let outgoing_connections: Vec<Connection> = connections_signal.get().iter()
            .filter(|c| c.from == node_id)
            .cloned()
            .collect();

        // Propagate to connected nodes with a delay
        for (idx, conn) in outgoing_connections.into_iter().enumerate() {
            let target_node_id = conn.to;
            let nodes_signal_clone = nodes_signal; // Clone for the closure
            let connections_signal_clone = connections_signal; // Clone for the closure
            
            // Activate the connection
            if let Some(connection_to_activate) = connections_signal_clone.get().iter().find(|c| c.from == conn.from && c.to == conn.to) {
                connection_to_activate.is_active.set(true);
                let conn_signal = connection_to_activate.is_active;
                set_timeout(move || conn_signal.set(false), Duration::from_millis(1000)); // Deactivate connection after 1s
            }

            // Use the cloned Rc for the recursive call
            let activate_and_propagate_clone = Rc::clone(&activate_and_propagate_rc_clone_for_recursive_call);

            set_timeout(move || {
                // Recursively activate the target node with a random chance
                if js_sys::Math::random() > 0.3 { // 70% chance to propagate
                    if let Some(activate_fn) = activate_and_propagate_clone.borrow().as_ref() {
                        activate_fn(target_node_id, nodes_signal_clone, connections_signal_clone);
                    }
                }
            }, Duration::from_millis(200 + (idx * 100) as u64)); // Staggered delay
        }
    }));

    // Activation logic
    create_effect(move |_| {
        let current_nodes = nodes.get();
        if current_nodes.is_empty() {
            return;
        }

        // Clone activate_and_propagate_rc for use in this effect's closure
        let activate_and_propagate_rc_effect = Rc::clone(&activate_and_propagate_rc);

        let interval = set_interval_with_handle(
            move || {
                // Activate a random node in the first layer
                let first_layer_nodes: Vec<Node> = current_nodes.iter()
                    .filter(|n| n.layer_idx == 0)
                    .cloned()
                    .collect();

                if let Some(node) = first_layer_nodes.get((js_sys::Math::random() * first_layer_nodes.len() as f64).floor() as usize) {
                    // Start propagation from a random first-layer node
                    // Get the actual closure from the RefCell
                    if let Some(activate_fn) = activate_and_propagate_rc_effect.borrow().as_ref() {
                        activate_fn(node.id, nodes, connections); // Pass signals
                    }
                }
            },
            Duration::from_secs(3),
        );

        on_cleanup(move || { interval.map(|h| h.clear()).ok(); });
    });

    view! {
        <svg width="100%" height="100%" preserveAspectRatio="xMidYMid slice" viewBox="0 0 100 100">
            <g class="neural-network-bg">
                {move || {
                    let current_nodes = nodes.get();
                    connections.get().into_iter().map(|conn| {
                        let from_node = current_nodes.iter().find(|n| n.id == conn.from).unwrap();
                        let to_node = current_nodes.iter().find(|n| n.id == conn.to).unwrap();
                        let class = move || format!("connection {}", if conn.is_active.get() { "active" } else { "" });
                        view! { <line x1=from_node.x y1=from_node.y x2=to_node.x y2=to_node.y class=class /> }
                    }).collect_view()
                }}
            </g>
            <g>
                {move || {
                    let current_nodes = nodes.get();
                    let lc_clone = layer_counts_for_view.clone();
                    current_nodes.into_iter().map(move |node| {
                        let is_output = node.layer_idx == lc_clone.len() - 1;
                        let class = move || format!("neuron-node {}", if node.is_active.get() { if is_output { "output-neuron active" } else { "active" } } else { "" });
                        
                        // The pulse path should be associated with the connection, not the node directly.
                        // For now, we'll keep a placeholder or remove it if not directly used for node pulse.
                        // The CSS `neuron-pulse` class will be applied to the connection lines.
                        view! {
                            <circle cx=node.x cy=node.y r=if is_output { 2.5 } else { 1.5 } class=class />
                        }
                    }).collect_view()
                }}
            </g>
        </svg>
    }
}
