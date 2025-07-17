use leptos::*;
use std::rc::Rc;
use std::cell::RefCell;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq)]
struct Node {
    id: usize,
    x: f64,
    y: f64,
    layer_idx: usize,
    is_active: RwSignal<bool>,
    activation_strength: RwSignal<f64>,
}

#[derive(Clone, Copy, PartialEq)]
struct Connection {
    from: usize,
    to: usize,
    is_active: RwSignal<bool>,
    pulse_progress: RwSignal<f64>,
    weight: f64,
}

#[component]
pub fn NeuralNetworkBackground() -> impl IntoView {
    let nodes = create_rw_signal(Vec::<Node>::new());
    let connections = create_rw_signal(Vec::<Connection>::new());

    // Configuration simplifiée pour arrière-plan
    let layer_counts = vec![4, 6, 4, 2];
    let layer_x_positions = vec![15.0, 40.0, 65.0, 85.0];
    let layer_colors = vec!["#4CAF50", "#2196F3", "#FF9800", "#F44336"];

    let layer_counts_for_view = layer_counts.clone();
    let layer_colors_for_view = layer_colors.clone();

    // Fonction d'activation récursive
    let activate_and_propagate_rc: Rc<RefCell<Option<Rc<dyn Fn(usize, f64, RwSignal<Vec<Node>>, RwSignal<Vec<Connection>>)>>>> =
        Rc::new(RefCell::new(None));

    // Génération de la structure du réseau
    create_effect(move |_| {
        let mut new_nodes = Vec::new();
        let mut new_connections = Vec::new();
        let mut id_counter = 0;

        let lc_clone_effect = layer_counts.clone();
        let lx_clone_effect = layer_x_positions.clone();

        // Génération des nœuds
        for (layer_idx, &count) in lc_clone_effect.iter().enumerate() {
            let x = lx_clone_effect[layer_idx];
            let y_start = 50.0 - (count as f64 * 8.0) / 2.0;
            
            for i in 0..count {
                let y = y_start + (i as f64 * 16.0) + 8.0;
                new_nodes.push(Node {
                    id: id_counter,
                    x,
                    y,
                    layer_idx,
                    is_active: create_rw_signal(false),
                    activation_strength: create_rw_signal(0.0),
                });
                id_counter += 1;
            }
        }

        // Génération des connexions
        for layer_idx in 0..lc_clone_effect.len() - 1 {
            let current_layer_nodes: Vec<&Node> = new_nodes.iter()
                .filter(|n| n.layer_idx == layer_idx)
                .collect();

            let next_layer_nodes: Vec<&Node> = new_nodes.iter()
                .filter(|n| n.layer_idx == layer_idx + 1)
                .collect();

            for node1 in current_layer_nodes {
                for node2 in &next_layer_nodes {
                    if js_sys::Math::random() > 0.3 {
                        let weight = js_sys::Math::random() * 0.8 + 0.2;
                        new_connections.push(Connection {
                            from: node1.id,
                            to: node2.id,
                            is_active: create_rw_signal(false),
                            pulse_progress: create_rw_signal(0.0),
                            weight,
                        });
                    }
                }
            }
        }

        nodes.set(new_nodes);
        connections.set(new_connections);
    });

    // Fonction d'activation récursive
    let activate_and_propagate_rc_clone = Rc::clone(&activate_and_propagate_rc);
    *activate_and_propagate_rc.borrow_mut() = Some(Rc::new(move |node_id: usize, strength: f64, nodes_signal: RwSignal<Vec<Node>>, connections_signal: RwSignal<Vec<Connection>>| {
        // Activation du nœud
        if let Some(node) = nodes_signal.get().iter().find(|n| n.id == node_id) {
            node.is_active.set(true);
            node.activation_strength.set(strength);
            
            let node_active_signal = node.is_active;
            let node_strength_signal = node.activation_strength;
            
            // Décroissance progressive
            let decay_interval = set_interval_with_handle(
                move || {
                    let current_strength = node_strength_signal.get();
                    if current_strength > 0.1 {
                        node_strength_signal.set(current_strength * 0.95);
                    } else {
                        node_active_signal.set(false);
                        node_strength_signal.set(0.0);
                    }
                },
                Duration::from_millis(50),
            );
            
            set_timeout(move || {
                decay_interval.map(|h| h.clear()).ok();
            }, Duration::from_millis(2000));
        }

        // Propagation vers les nœuds connectés
        let outgoing_connections: Vec<Connection> = connections_signal.get().iter()
            .filter(|c| c.from == node_id)
            .cloned()
            .collect();

        for (idx, conn) in outgoing_connections.into_iter().enumerate() {
            let target_node_id = conn.to;
            let propagated_strength = strength * conn.weight * 0.7;
            
            // Animation de pulsation
            if let Some(connection_to_activate) = connections_signal.get().iter().find(|c| c.from == conn.from && c.to == conn.to) {
                connection_to_activate.is_active.set(true);
                connection_to_activate.pulse_progress.set(0.0);
                
                let conn_signal = connection_to_activate.is_active;
                let pulse_signal = connection_to_activate.pulse_progress;
                
                let pulse_interval = set_interval_with_handle(
                    move || {
                        let current_progress = pulse_signal.get();
                        if current_progress < 1.0 {
                            pulse_signal.set(current_progress + 0.05);
                        } else {
                            conn_signal.set(false);
                            pulse_signal.set(0.0);
                        }
                    },
                    Duration::from_millis(25),
                );
                
                set_timeout(move || {
                    pulse_interval.map(|h| h.clear()).ok();
                }, Duration::from_millis(600));
            }

            let activate_and_propagate_clone = Rc::clone(&activate_and_propagate_rc_clone);
            
            set_timeout(move || {
                if propagated_strength > 0.2 && js_sys::Math::random() > 0.3 {
                    if let Some(activate_fn) = activate_and_propagate_clone.borrow().as_ref() {
                        activate_fn(target_node_id, propagated_strength, nodes_signal, connections_signal);
                    }
                }
            }, Duration::from_millis(400 + (idx * 100) as u64));
        }
    }));

    // Animation automatique pour arrière-plan
    create_effect(move |_| {
        let current_nodes = nodes.get();
        if current_nodes.is_empty() {
            return;
        }

        let activate_and_propagate_rc_effect = Rc::clone(&activate_and_propagate_rc);

        let interval = set_interval_with_handle(
            move || {
                let first_layer_nodes: Vec<Node> = current_nodes.iter()
                    .filter(|n| n.layer_idx == 0)
                    .cloned()
                    .collect();

                if let Some(node) = first_layer_nodes.get((js_sys::Math::random() * first_layer_nodes.len() as f64).floor() as usize) {
                    // Activation moins fréquente pour l'arrière-plan
                    if js_sys::Math::random() > 0.4 {
                        if let Some(activate_fn) = activate_and_propagate_rc_effect.borrow().as_ref() {
                            activate_fn(node.id, 1.0, nodes, connections);
                        }
                    }
                }
            },
            Duration::from_millis(3000),
        );

        on_cleanup(move || { interval.map(|h| h.clear()).ok(); });
    });

    view! {
        <div class="neural-background">
            <svg width="100%" height="100%" viewBox="0 0 100 100" class="neural-svg">
                <defs>
                    <filter id="glow">
                        <feGaussianBlur stdDeviation="1" result="coloredBlur"/>
                        <feMerge>
                            <feMergeNode in="coloredBlur"/>
                            <feMergeNode in="SourceGraphic"/>
                        </feMerge>
                    </filter>
                </defs>

                // Connexions
                <g class="connections">
                    {move || {
                        connections.get().into_iter().map(move |conn| {
                            let current_nodes_owned = nodes.get_untracked();
                            let from_node = current_nodes_owned.iter().find(|n| n.id == conn.from).copied().unwrap();
                            let to_node = current_nodes_owned.iter().find(|n| n.id == conn.to).copied().unwrap();
                            
                            let opacity = if conn.is_active.get() { 0.6 } else { 0.2 };
                            let stroke_width = conn.weight * 0.15 + 0.05;
                            
                            view! {
                                <line
                                    x1=from_node.x
                                    y1=from_node.y
                                    x2=to_node.x
                                    y2=to_node.y
                                    stroke="#666"
                                    stroke-width=stroke_width
                                    stroke-opacity=opacity
                                    class="connection"
                                />
                                {move || -> leptos::View {
                                    if conn.is_active.get() && conn.pulse_progress.get() > 0.0 {
                                        let progress = conn.pulse_progress.get();
                                        let pulse_x = from_node.x + (to_node.x - from_node.x) * progress;
                                        let pulse_y = from_node.y + (to_node.y - from_node.y) * progress;
                                        
                                        view! {
                                            <circle
                                                cx=pulse_x
                                                cy=pulse_y
                                                r="0.3"
                                                fill="#00ff88"
                                                filter="url(#glow)"
                                                class="pulse"
                                            />
                                        }.into_view()
                                    } else {
                                        view! {}.into_view()
                                    }
                                }}
                            }
                        }).collect_view()
                    }}
                </g>

                // Nœuds
                <g class="nodes">
                    {move || {
                        let current_nodes = nodes.get();
                        let lc_clone = layer_counts_for_view.clone();
                        let colors_clone = layer_colors_for_view.clone();
                        
                        current_nodes.into_iter().map(move |node| {
                            let is_output = node.layer_idx == lc_clone.len() - 1;
                            let base_radius = if is_output { 1.5 } else { 1.0 };
                            let layer_color = colors_clone[node.layer_idx];
                            
                            let radius = move || {
                                if node.is_active.get() {
                                    base_radius + (node.activation_strength.get() * 0.8)
                                } else {
                                    base_radius
                                }
                            };
                            
                            let opacity = move || {
                                if node.is_active.get() {
                                    0.6 + (node.activation_strength.get() * 0.2)
                                } else {
                                    0.5
                                }
                            };
                            
                            view! {
                                <circle
                                    cx=node.x
                                    cy=node.y
                                    r=radius
                                    fill=layer_color
                                    fill-opacity=opacity
                                    stroke="white"
                                    stroke-width="0.1"
                                    class="neuron-node"
                                    filter=move || if node.is_active.get() { "url(#glow)" } else { "none" }
                                />
                            }
                        }).collect_view()
                    }}
                </g>
            </svg>
        </div>

        <style>
            ".neural-background {
                position: fixed;
                top: 0;
                left: 0;
                width: 100vw;
                height: 100vh;
                z-index: -1;
                opacity: 0.6;
                background: linear-gradient(135deg, #0f1419 0%, #1a2332 100%);
            }
            
            .neural-svg {
                width: 100%;
                height: 100%;
            }
            
            .connection {
                transition: all 0.3s ease;
            }
            
            .neuron-node {
                transition: all 0.3s ease;
            }
            
            .pulse {
                animation: pulse-animation 0.6s ease-in-out;
            }
            
            @keyframes pulse-animation {
                0% { r: 0.2; opacity: 1; }
                100% { r: 0.6; opacity: 0; }
            }"
        </style>
    }
}
