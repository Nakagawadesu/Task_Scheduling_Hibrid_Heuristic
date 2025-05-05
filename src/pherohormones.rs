use std::fs::{self, File};
use std::io::Write;
use std::process::Command;

use petgraph::{
    stable_graph::{EdgeIndex, NodeIndex, StableDiGraph},
    Direction,
};
pub struct Path {
    pub task: i32,
    pub weight: f64,
}
#[derive(Clone)]

pub struct Pherohormones {
    pub di_graph: StableDiGraph<f64, f64>,
    pub pheromones_sum: f64,
    pub n_tasks: i32,
    pub output_dir: String,
}
impl Pherohormones {
    pub fn new(n_tasks: i32, output_dir: &str) -> Pherohormones {
        Pherohormones {
            di_graph: StableDiGraph::new(),
            n_tasks: n_tasks,
            output_dir: output_dir.to_string(),

            pheromones_sum: 0.0,
        }
    }

    pub fn initialize(&mut self) {
        for i in 0..self.n_tasks {
            //add nodes with no arbitrary wheigth for now
            self.di_graph.add_node(i as f64);
        }
    }

    pub fn deposit_pherohormones(
        &mut self,
        task_completed: i32,
        next_task: i32,
        deposit_rate: f64,
    ) {
        let source = NodeIndex::new(task_completed as usize);
        let target = NodeIndex::new(next_task as usize);

        match self.di_graph.find_edge(source, target) {
            Some(edge_idx) => {
                if let Some(weight) = self.di_graph.edge_weight_mut(edge_idx) {
                    *weight += deposit_rate;
                }
            }
            None => {
                // println!(
                //     "Adding edge from {} to {} with weight {:.2}",
                //     task_completed + 1,
                //     next_task + 1,
                //     deposit_rate
                // );
                self.di_graph.add_edge(source, target, deposit_rate);
            }
        }
    }

    pub fn evaporate_pherohormones(&mut self, evaporation_rate: f64) {
        let edges: Vec<EdgeIndex> = self.di_graph.edge_indices().collect();
        for edge in edges {
            // returns the pointer to the weight ,i can change it directly

            if let Some(weight) = self.di_graph.edge_weight_mut(edge) {
                *weight = (*weight - evaporation_rate).max(0.0);
                if *weight <= 0.00 {
                    self.di_graph.remove_edge(edge);
                }
            }
            self.update_pherohormones_sum();
            // self.print_pherohormones();
        }
    }

    // return avauilabe paths from a task related to trhe pherohormones
    pub fn find_paths(&self, task_completed: i32) -> Vec<Path> {
        let mut paths = Vec::new();
        let task_completed_idx = NodeIndex::new(task_completed as usize);

        for neighbor in self
            .di_graph
            .neighbors_directed(task_completed_idx, Direction::Outgoing)
        {
            if let Some(edge_idx) = self.di_graph.find_edge(task_completed_idx, neighbor) {
                if let Some(&weight) = self.di_graph.edge_weight(edge_idx) {
                    paths.push(Path {
                        task: neighbor.index() as i32,
                        weight,
                    });
                }
            }
        }

        paths
    }
    pub fn update_pherohormones_sum(&mut self) {
        // sum all wheights in the graph
        self.pheromones_sum = self
            .di_graph
            .edge_weights()
            .cloned()
            .fold(0.0, |acc, x| acc + x);
        // print the sum
        //println!("Pheromones sum: {}", self.pheromones_sum);
    }

    pub fn print_pherohormones(&mut self) {
        for edge in self.di_graph.edge_indices() {
            let (source, target) = self.di_graph.edge_endpoints(edge).unwrap();
            let weight = self.di_graph.edge_weight(edge).unwrap();
            println!(
                "Edge from {} to {} with weight {}",
                source.index() + 1,
                target.index() + 1,
                weight
            );
        }
    }

    pub fn to_gexf(&self, cycle: i32) -> String {
        let mut gexf = String::new();
        let weights: Vec<f64> = self.di_graph.edge_weights().cloned().collect();
        let max_weight = weights.iter().fold(0.0_f64, |a, &b| a.max(b));
        let min_weight = weights.iter().fold(f64::MAX, |a, &b| a.min(b));
        let weight_range = (max_weight - min_weight).max(f64::EPSILON);

        // Thickness configuration - adjust these values to control edge sizes
        let thickness_min = 0.8; // Minimum visible thickness
        let thickness_max = 3.0; // Maximum thickness
        let thickness_range = thickness_max - thickness_min;

        gexf.push_str(
            r##"<?xml version="1.0" encoding="UTF-8"?>
    <gexf xmlns="http://www.gexf.net/1.3"
          xmlns:viz="http://www.gexf.net/1.3/viz"
          xmlns:xsi="http://www.w3.org/2001/XMLSchema-instance"
          xsi:schemaLocation="http://www.gexf.net/1.3 http://www.gexf.net/1.3/gexf.xsd"
          version="1.3">
      <meta>
        <creator>Pheromone System</creator>
        <description>Cycle "##,
        );
        gexf.push_str(&format!("{}", cycle));
        gexf.push_str(
            r##"</description>
      </meta>
      <graph mode="static" defaultedgetype="directed">
        <attributes class="node">
          <attribute id="0" title="Degree" type="integer"/>
        </attributes>
        <attributes class="edge">
          <attribute id="0" title="Weight" type="float"/>
        </attributes>
        <nodes>"##,
        );

        // Nodes with fixed size and color
        for node in self.di_graph.node_indices() {
            let degree = self
                .di_graph
                .edges_directed(node, Direction::Outgoing)
                .count();
            gexf.push_str(&format!(
                r##"
          <node id="{}" label="T{}">
            <viz:color r="192" g="192" b="192"/>
            <viz:size value="12.0"/>
            <attvalues>
              <attvalue for="0" value="{}"/>
            </attvalues>
          </node>"##,
                node.index(),
                node.index() + 1,
                degree
            ));
        }

        gexf.push_str(
            r##"
        </nodes>
        <edges>"##,
        );

        // Edges with compressed thickness scaling
        for (i, edge) in self.di_graph.edge_indices().enumerate() {
            if let Some((source, target)) = self.di_graph.edge_endpoints(edge) {
                if let Some(&weight) = self.di_graph.edge_weight(edge) {
                    let t = (weight - min_weight) / weight_range;

                    // Non-linear scaling to compress high values
                    let scaled_t = t.powf(0.6); // Experiment with exponent (0.5-0.8)

                    // Calculate thickness in configured range
                    let thickness = thickness_min + (scaled_t * thickness_range);

                    // Color gradient from blue to red
                    let hue = (t * 240.0) as u8; // 0°(blue) to 240°(red)
                    let color = match hue {
                        0..=120 => (120 - hue, hue, 160),        // Blue to cyan
                        121..=240 => (hue - 120, 240 - hue, 60), // Magenta to red
                        _ => (0, 0, 255),
                    };

                    gexf.push_str(&format!(
                        r##"
          <edge id="{}" source="{}" target="{}" weight="{:.4}">
            <viz:color r="{}" g="{}" b="{}"/>
            <viz:thickness value="{:.2}"/>
            <attvalues>
              <attvalue for="0" value="{:.4}"/>
            </attvalues>
          </edge>"##,
                        i,
                        source.index(),
                        target.index(),
                        weight,
                        color.0,
                        color.1,
                        color.2,
                        thickness,
                        weight
                    ));
                }
            }
        }

        gexf.push_str(
            r##"
        </edges>
      </graph>
    </gexf>"##,
        );

        gexf
    }

    // Add this new save method
    pub fn save_gephi(&self, iteration: i32) -> std::io::Result<()> {
        fs::create_dir_all(&self.output_dir)?;
        let path = format!("{}/gephi_{:04}.gexf", self.output_dir, iteration);
        let mut file = File::create(&path)?;

        // Write BOM for UTF-8 compatibility
        file.write_all(b"\xEF\xBB\xBF")?;
        file.write_all(self.to_gexf(iteration).as_bytes())?;

        Ok(())
    }
}
