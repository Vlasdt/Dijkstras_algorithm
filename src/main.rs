use plotters::prelude::*;
use std::vec;

struct Graph<const N: usize> {
    V: [usize; N],
    E: Vec<(usize, usize, i64)>,
}

impl<const N: usize> Graph<N> {
    fn new(V: [usize; N]) -> Self {
        Graph { V, E: Vec::new() }
    }

    // Список смежности для ориентированного графа
    fn adjacency_list(&self) -> Vec<Vec<(usize, i64)>> {
        let mut adj = vec![vec![]; N];
        for &(u, v, w) in &self.E {
            adj[u].push((v, w));
        }
        adj
    }

    fn draw_graph(&self, filename: &str) -> Result<(), Box<dyn std::error::Error>> {
        let adj_list = self.adjacency_list();
        let n = adj_list.len();
        let size = 400.0;
        let radius = 160.0;
        let center = size / 2.0;

        let positions: Vec<(f64, f64)> = (0..n)
            .map(|i| {
                let angle = 2.0 * std::f64::consts::PI * (i as f64) / (n as f64);
                let (x, y) = (center + radius * angle.cos(), center + radius * angle.sin());
                (x, y)
            })
            .collect();

        let root = BitMapBackend::new(filename, (400, 400)).into_drawing_area();
        root.fill(&WHITE)?;
        let mut chart = ChartBuilder::on(&root)
            .margin(20)
            .build_cartesian_2d(0f64..size, 0f64..size)?;

        for (i, neighbors) in adj_list.iter().enumerate() {
            for &(j, weight) in neighbors {
                // Рисуем стрелку (упрощённо – просто линия)
                chart.draw_series(LineSeries::new(vec![positions[i], positions[j]], &BLACK))?;

                let (x1, y1) = positions[i];
                let (x2, y2) = positions[j];
                let mid_x = (x1 + x2) / 2.0;
                let mid_y = (y1 + y2) / 2.0;

                chart.draw_series(std::iter::once(Text::new(
                    format!("{}", weight),
                    (mid_x, mid_y),
                    ("sans-serif", 12).into_font().color(&BLUE),
                )))?;
            }
        }

        for (i, &(x, y)) in positions.iter().enumerate() {
            chart.draw_series(std::iter::once(Circle::new((x, y), 10, RED.filled())))?;
            chart.draw_series(std::iter::once(Text::new(
                (i + 1).to_string(),
                (x - 5.0, y + 3.0),
                ("sans-serif", 15).into_font().color(&BLACK),
            )))?;
        }

        root.present()?;
        Ok(())
    }

    fn dijkstra(&self, start: usize, end: usize) -> ([usize; N], i64) {
        let adj_list = self.adjacency_list();
        let mut D = [i64::MAX; N];
        let mut H = [0; N];
        let mut X = [false; N];
        D[start] = 0;
        X[start] = true;
        let mut p = start;
        loop {
            for &(neighbor, weight) in &adj_list[p] {
                if !X[neighbor] {
                    let new_dist = D[p].saturating_add(weight);
                    if new_dist < D[neighbor] {
                        D[neighbor] = new_dist;
                        H[neighbor] = p;
                    }
                }
            }
            let mut next_vertex = None;
            let mut min_dist = i64::MAX;
            for v in 0..N {
                if !X[v] && D[v] < min_dist {
                    min_dist = D[v];
                    next_vertex = Some(v);
                }
            }
            let next_vertex = next_vertex.expect("");
            X[next_vertex] = true;
            if next_vertex == end {
                return (H, D[end]);
            }
            p = next_vertex;
        }
    }

    //v^3 v^2
    fn critical_path(&self) -> (i64, Vec<usize>) {
        let mut in_degree = vec![0; N];
        let mut out_degree = vec![0; N];
        for &(u, v, _) in &self.E {
            out_degree[u] += 1;
            in_degree[v] += 1;
        }
        let s = (0..N).find(|&i| in_degree[i] == 0).expect("1");
        let t = (0..N).find(|&i| out_degree[i] == 0).expect("2");

        let mut dist = [[i64::MIN; N]; N];
        let mut next = [[None; N]; N];
        for i in 0..N {
            dist[i][i] = 0;
            next[i][i] = Some(i);
        }
        for &(u, v, w) in &self.E {
            if w > dist[u][v] {
                dist[u][v] = w;
                next[u][v] = Some(v);
            }
        }

        for k in 0..N {
            for i in 0..N {
                if dist[i][k] == i64::MIN {
                    continue;
                }
                for j in 0..N {
                    if dist[k][j] == i64::MIN {
                        continue;
                    }
                    let new_dist = dist[i][k] + dist[k][j];
                    if new_dist > dist[i][j] {
                        dist[i][j] = new_dist;
                        next[i][j] = next[i][k];
                    }
                }
            }
        }

        let length = dist[s][t];
        if length == i64::MIN {
            panic!("Пут {}  {} не существует", s + 1, t + 1);
        }

        let mut path = Vec::new();
        let mut cur = s;
        while cur != t {
            path.push(cur);
            cur = next[cur][t].expect("3");
        }
        path.push(t);

        (length, path)
    }
}

fn main() {
    let mut graph = Graph::<14>::new([0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13]);

    graph.E.push((0, 1, 5)); // 1 -> 2
    graph.E.push((0, 2, 7)); // 1 -> 3
    graph.E.push((0, 3, 10)); // 1 -> 4
    graph.E.push((1, 2, 16)); // 2 -> 3
    graph.E.push((1, 5, 8)); // 2 -> 6
    graph.E.push((1, 6, 9)); // 2 -> 7
    graph.E.push((1, 7, 14)); // 2 -> 8
    graph.E.push((2, 4, 19)); // 3 -> 5
    graph.E.push((2, 5, 11)); // 3 -> 6
    graph.E.push((3, 2, 11)); // 4 -> 3
    graph.E.push((3, 4, 11)); // 4 -> 5
    graph.E.push((3, 9, 26)); // 4 -> 10
    graph.E.push((3, 10, 30)); // 4 -> 11
    graph.E.push((4, 9, 3)); // 5 -> 10
    graph.E.push((5, 9, 13)); // 6 -> 10
    graph.E.push((5, 11, 17)); // 6 -> 12
    graph.E.push((6, 8, 30)); // 7 -> 9
    graph.E.push((7, 8, 18)); // 8 -> 9
    graph.E.push((7, 11, 21)); // 8 -> 12
    graph.E.push((8, 12, 8)); // 9 -> 13
    graph.E.push((9, 10, 15)); // 10 -> 11
    graph.E.push((10, 11, 19)); // 11 -> 12
    graph.E.push((10, 12, 12)); // 11 -> 13
    graph.E.push((11, 12, 14)); // 12 -> 13
    graph.E.push((12, 13, 7)); // 13 -> 14

    graph.draw_graph("graph.png").unwrap();

    let (length, path) = graph.critical_path();
    let path_1based: Vec<usize> = path.iter().map(|&v| v + 1).collect();

    println!("Длина: {} дней", length);
    println!("Путь: {:?}", path_1based);
}
