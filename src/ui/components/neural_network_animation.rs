use leptos::*;
use std::cell::RefCell;
use std::rc::Rc;
use std::time::Duration;

#[derive(Clone, Copy, PartialEq)]
struct Node {
    id: usize,
    x: f64,
    y: f64,
    layer_idx: usize,
    is_active: RwSignal<bool>,
    activation_strength: RwSignal<f64>,
    node_type: NodeType,
}

#[derive(Clone, Copy, PartialEq)]
enum NodeType {
    Input,
    Hidden,
    Output,
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

    // Configuration améliorée pour un design plus moderne
    let layer_counts = vec![5, 8, 6, 3];
    let layer_x_positions = vec![12.0, 35.0, 65.0, 88.0];

    // Palette de couleurs moderne et élégante
    let layer_colors = vec![
        "#0ea5e9", // Bleu moderne (input)
        "#3b82f6", // Bleu électrique (hidden 1)
        "#6366f1", // Indigo (hidden 2)
        "#8b5cf6", // Violet (output)
    ];

    let layer_counts_for_view = layer_counts.clone();
    let layer_colors_for_view = layer_colors.clone();

    // Fonction d'activation récursive améliorée
    let activate_and_propagate_rc: Rc<
        RefCell<Option<Rc<dyn Fn(usize, f64, RwSignal<Vec<Node>>, RwSignal<Vec<Connection>>)>>>,
    > = Rc::new(RefCell::new(None));

    // Génération de la structure du réseau avec amélioration
    create_effect(move |_| {
        let mut new_nodes = Vec::new();
        let mut new_connections = Vec::new();
        let mut id_counter = 0;

        let lc_clone_effect = layer_counts.clone();
        let lx_clone_effect = layer_x_positions.clone();

        // Génération des nœuds avec positionnement amélioré
        for (layer_idx, &count) in lc_clone_effect.iter().enumerate() {
            let x = lx_clone_effect[layer_idx];
            let y_start = 50.0 - (count as f64 * 10.0) / 2.0;

            let node_type = match layer_idx {
                0 => NodeType::Input,
                n if n == lc_clone_effect.len() - 1 => NodeType::Output,
                _ => NodeType::Hidden,
            };

            for i in 0..count {
                let y = y_start + (i as f64 * 10.0) + 5.0;
                new_nodes.push(Node {
                    id: id_counter,
                    x,
                    y,
                    layer_idx,
                    is_active: create_rw_signal(false),
                    activation_strength: create_rw_signal(0.0),
                    node_type,
                });
                id_counter += 1;
            }
        }

        // Génération des connexions avec densité optimisée
        for layer_idx in 0..lc_clone_effect.len() - 1 {
            let current_layer_nodes: Vec<&Node> = new_nodes
                .iter()
                .filter(|n| n.layer_idx == layer_idx)
                .collect();

            let next_layer_nodes: Vec<&Node> = new_nodes
                .iter()
                .filter(|n| n.layer_idx == layer_idx + 1)
                .collect();

            for node1 in current_layer_nodes {
                for node2 in &next_layer_nodes {
                    // Probabilité de connexion ajustée pour un réseau plus réaliste
                    let connection_probability = match layer_idx {
                        0 => 0.6, // Input vers hidden : moins dense
                        1 => 0.8, // Hidden vers hidden : plus dense
                        _ => 0.7, // Hidden vers output : modérément dense
                    };

                    if js_sys::Math::random() > (1.0 - connection_probability) {
                        let weight = js_sys::Math::random() * 0.6 + 0.4; // Poids plus équilibrés
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

    // Fonction d'activation avec améliorations fluides
    let activate_and_propagate_rc_clone = Rc::clone(&activate_and_propagate_rc);
    *activate_and_propagate_rc.borrow_mut() = Some(Rc::new(
        move |node_id: usize,
              strength: f64,
              nodes_signal: RwSignal<Vec<Node>>,
              connections_signal: RwSignal<Vec<Connection>>| {
            // Activation du nœud avec animation plus fluide
            if let Some(node) = nodes_signal.get().iter().find(|n| n.id == node_id) {
                node.is_active.set(true);
                node.activation_strength.set(strength);

                let node_active_signal = node.is_active;
                let node_strength_signal = node.activation_strength;

                // Décroissance progressive plus naturelle
                let decay_interval = set_interval_with_handle(
                    move || {
                        let current_strength = node_strength_signal.get();
                        if current_strength > 0.05 {
                            node_strength_signal.set(current_strength * 0.92); // Décroissance plus douce
                        } else {
                            node_active_signal.set(false);
                            node_strength_signal.set(0.0);
                        }
                    },
                    Duration::from_millis(80), // Interval plus fluide
                );

                set_timeout(
                    move || {
                        decay_interval.map(|h| h.clear()).ok();
                    },
                    Duration::from_millis(3000), // Durée plus longue
                );
            }

            // Propagation vers les nœuds connectés avec timing amélioré
            let outgoing_connections: Vec<Connection> = connections_signal
                .get()
                .iter()
                .filter(|c| c.from == node_id)
                .cloned()
                .collect();

            for (idx, conn) in outgoing_connections.into_iter().enumerate() {
                let target_node_id = conn.to;
                let propagated_strength = strength * conn.weight * 0.8; // Meilleure propagation

                // Animation de pulsation améliorée
                if let Some(connection_to_activate) = connections_signal
                    .get()
                    .iter()
                    .find(|c| c.from == conn.from && c.to == conn.to)
                {
                    connection_to_activate.is_active.set(true);
                    connection_to_activate.pulse_progress.set(0.0);

                    let conn_signal = connection_to_activate.is_active;
                    let pulse_signal = connection_to_activate.pulse_progress;

                    let pulse_interval = set_interval_with_handle(
                        move || {
                            let current_progress = pulse_signal.get();
                            if current_progress < 1.0 {
                                pulse_signal.set(current_progress + 0.03); // Progression plus fluide
                            } else {
                                conn_signal.set(false);
                                pulse_signal.set(0.0);
                            }
                        },
                        Duration::from_millis(30), // Animation plus fluide
                    );

                    set_timeout(
                        move || {
                            pulse_interval.map(|h| h.clear()).ok();
                        },
                        Duration::from_millis(1000), // Durée ajustée
                    );
                }

                let activate_and_propagate_clone = Rc::clone(&activate_and_propagate_rc_clone);

                set_timeout(
                    move || {
                        if propagated_strength > 0.15 && js_sys::Math::random() > 0.2 {
                            if let Some(activate_fn) =
                                activate_and_propagate_clone.borrow().as_ref()
                            {
                                activate_fn(
                                    target_node_id,
                                    propagated_strength,
                                    nodes_signal,
                                    connections_signal,
                                );
                            }
                        }
                    },
                    Duration::from_millis(300 + (idx * 80) as u64), // Timing plus naturel
                );
            }
        },
    ));

    // Animation automatique avec rythme amélioré
    create_effect(move |_| {
        let current_nodes = nodes.get();
        if current_nodes.is_empty() {
            return;
        }

        let activate_and_propagate_rc_effect = Rc::clone(&activate_and_propagate_rc);

        let interval = set_interval_with_handle(
            move || {
                let first_layer_nodes: Vec<Node> = current_nodes
                    .iter()
                    .filter(|n| n.layer_idx == 0)
                    .cloned()
                    .collect();

                if let Some(node) = first_layer_nodes
                    .get((js_sys::Math::random() * first_layer_nodes.len() as f64).floor() as usize)
                {
                    // Activation plus fréquente mais contrôlée
                    if js_sys::Math::random() > 0.3 {
                        if let Some(activate_fn) =
                            activate_and_propagate_rc_effect.borrow().as_ref()
                        {
                            activate_fn(node.id, 1.0, nodes, connections);
                        }
                    }
                }
            },
            Duration::from_millis(2500), // Rythme plus vivant
        );

        on_cleanup(move || {
            interval.map(|h| h.clear()).ok();
        });
    });

    view! {
        <div class="neural-background-enhanced">
            <svg width="100%" height="100%" viewBox="0 15 100 100" class="neural-svg-enhanced">
                <defs>
                    // Effet de lueur moderne
                    <filter id="modernGlow">
                        <feGaussianBlur stdDeviation="1.5" result="coloredBlur"/>
                        <feMerge>
                            <feMergeNode in="coloredBlur"/>
                            <feMergeNode in="SourceGraphic"/>
                        </feMerge>
                    </filter>

                    // Gradient pour les connexions
                    <linearGradient id="connectionGradient" x1="0%" y1="0%" x2="100%" y2="0%">
                        <stop offset="0%" style="stop-color:#0ea5e9;stop-opacity:0.6"/>
                        <stop offset="100%" style="stop-color:#6366f1;stop-opacity:0.3"/>
                    </linearGradient>

                    // Effet de pulsation pour les nœuds actifs
                    <filter id="pulseGlow">
                        <feGaussianBlur stdDeviation="2" result="coloredBlur"/>
                        <feMerge>
                            <feMergeNode in="coloredBlur"/>
                            <feMergeNode in="SourceGraphic"/>
                        </feMerge>
                    </filter>
                </defs>

                // Connexions avec design amélioré
                <g class="connections-enhanced">
                    {move || {
                        connections.get().into_iter().map(move |conn| {
                            let current_nodes_owned = nodes.get_untracked();
                            let from_node = current_nodes_owned.iter().find(|n| n.id == conn.from).copied().unwrap();
                            let to_node = current_nodes_owned.iter().find(|n| n.id == conn.to).copied().unwrap();

                            let base_opacity = 0.15;
                            let active_opacity = 0.8;
                            let opacity = if conn.is_active.get() { active_opacity } else { base_opacity };
                            let stroke_width = conn.weight * 0.12 + 0.08;

                            view! {
                                <line
                                    x1=from_node.x
                                    y1=from_node.y
                                    x2=to_node.x
                                    y2=to_node.y
                                    stroke="url(#connectionGradient)"
                                    stroke-width=stroke_width
                                    stroke-opacity=opacity
                                    class="connection-enhanced"
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
                                                fill="#00d9ff"
                                                filter="url(#modernGlow)"
                                                class="pulse-enhanced"
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

                // Nœuds avec design moderne
                <g class="nodes-enhanced">
                    {move || {
                        let current_nodes = nodes.get();
                        let _lc_clone = layer_counts_for_view.clone();
                        let colors_clone = layer_colors_for_view.clone();

                        current_nodes.into_iter().map(move |node| {
                            let base_radius = match node.node_type {
                                NodeType::Input => 0.8,
                                NodeType::Hidden => 0.6,
                                NodeType::Output => 1.0,
                            };
                            let layer_color = colors_clone[node.layer_idx];

                            let radius = move || {
                                if node.is_active.get() {
                                    base_radius + (node.activation_strength.get() * 0.6)
                                } else {
                                    base_radius
                                }
                            };

                            let opacity = move || {
                                if node.is_active.get() {
                                    0.6 + (node.activation_strength.get() * 0.2)
                                } else {
                                    0.4
                                }
                            };

                            let stroke_width = if node.is_active.get() { "0.15" } else { "0.1" };
                            let stroke_color = if node.is_active.get() { "#ffffff" } else { "#94a3b8" };

                            view! {
                                <circle
                                    cx=node.x
                                    cy=node.y
                                    r=radius
                                    fill=layer_color
                                    fill-opacity=opacity
                                    stroke=stroke_color
                                    stroke-width=stroke_width
                                    class="neuron-node-enhanced"
                                    filter=move || if node.is_active.get() { "url(#pulseGlow)" } else { "none" }
                                />
                            }
                        }).collect_view()
                    }}
                </g>
            </svg>
        </div>

        <style>
            ".neural-background-enhanced {
                position: absolute;
                top: 0;
                left: 0;
                width: 100vw;
                opacity: 0.9;
                background: linear-gradient(135deg, #0c1426 0%, #1e293b 50%, #334155 100%);
                backdrop-filter: blur(0.5px);
            }

            .neural-svg-enhanced {
                width: 100%;
                height: 100%;
            }

            .connection-enhanced {
                transition: all 0.4s cubic-bezier(0.4, 0, 0.2, 1);
            }

            .neuron-node-enhanced {
                transition: all 0.5s cubic-bezier(0.4, 0, 0.2, 1);
            }

            .pulse-enhanced {
                animation: pulse-animation-enhanced 1s ease-out;
            }

            @keyframes pulse-animation-enhanced {
                0% {
                    r: 0.1;
                    opacity: 1;
                    fill: #00d9ff;
                }
                50% {
                    r: 0.4;
                    opacity: 0.8;
                    fill: #0ea5e9;
                }
                100% {
                    r: 0.6;
                    opacity: 0;
                    fill: #6366f1;
                }
            }

            // Animation de respiration pour le fond
            @keyframes breathe {
                0%, 100% { opacity: 0.9; }
                50% { opacity: 0.7; }
            }

            .neural-background-enhanced {
                animation: breathe 8s ease-in-out infinite;
            }

            // Effet de particules flottantes
            .neural-background-enhanced::before {
                content: '';
                position: absolute;
                top: 0;
                left: 0;
                width: 100%;
                height: 100%;
                background-image:
                    radial-gradient(circle at 20% 30%, rgba(14, 165, 233, 0.1) 0%, transparent 50%),
                    radial-gradient(circle at 80% 70%, rgba(99, 102, 241, 0.08) 0%, transparent 50%),
                    radial-gradient(circle at 40% 80%, rgba(139, 92, 246, 0.06) 0%, transparent 50%);
                animation: float-particles 15s ease-in-out infinite;
            }

            @keyframes float-particles {
                0%, 100% { transform: translate(0, 0) rotate(0deg); }
                33% { transform: translate(10px, -15px) rotate(120deg); }
                66% { transform: translate(-8px, 10px) rotate(240deg); }
            }

            // Responsive design
            @media (max-width: 768px) {
                .neural-background-enhanced {
                    opacity: 0.6;
                }

                .neuron-node-enhanced {
                    transition: none;
                }

                .connection-enhanced {
                    transition: none;
                }
            }

            // Amélioration pour les appareils à haute densité
            @media (-webkit-min-device-pixel-ratio: 2), (min-resolution: 192dpi) {
                .neural-svg-enhanced {
                    image-rendering: -webkit-optimize-contrast;
                    image-rendering: crisp-edges;
                }
            }"
        </style>
    }
}
