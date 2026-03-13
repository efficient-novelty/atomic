use pen_core::expr::Expr;
use pen_core::telescope::Telescope;
use std::collections::{BTreeMap, BTreeSet};

pub type ClauseIx = usize;
pub type ClauseGraph = BTreeMap<ClauseIx, BTreeSet<ClauseIx>>;

pub fn build_clause_dependency_graph(telescope: &Telescope) -> ClauseGraph {
    telescope
        .clauses
        .iter()
        .enumerate()
        .map(|(index, clause)| (index, entry_deps(index, 0, &clause.expr)))
        .collect()
}

pub fn terminal_clause_sccs(telescope: &Telescope) -> Vec<Vec<ClauseIx>> {
    let graph = build_clause_dependency_graph(telescope);
    let mut components = strongly_connected_components(&graph, telescope.clauses.len());

    components.retain(|component| {
        !component.is_empty()
            && component.len() < telescope.clauses.len()
            && is_terminal_component(&graph, component)
    });
    components.sort_by_key(|component| component[0]);
    components
}

pub fn remove_clause_set(telescope: &Telescope, remove_ixs: &[ClauseIx]) -> Telescope {
    let remove_set: BTreeSet<_> = remove_ixs.iter().copied().collect();
    Telescope::new(
        telescope
            .clauses
            .iter()
            .enumerate()
            .filter(|(index, _)| !remove_set.contains(index))
            .map(|(_, clause)| clause.clone())
            .collect(),
    )
}

pub fn terminal_scc_sub_bundles(telescope: &Telescope) -> Vec<Telescope> {
    terminal_clause_sccs(telescope)
        .into_iter()
        .map(|component| remove_clause_set(telescope, &component))
        .filter(|candidate| !candidate.clauses.is_empty())
        .collect()
}

fn entry_deps(current: ClauseIx, binder_depth: usize, expr: &Expr) -> BTreeSet<ClauseIx> {
    match expr {
        Expr::App(function, argument)
        | Expr::Pi(function, argument)
        | Expr::Sigma(function, argument) => {
            let mut deps = entry_deps(current, binder_depth, function);
            deps.extend(entry_deps(current, binder_depth, argument));
            deps
        }
        Expr::Lam(body) => entry_deps(current, binder_depth + 1, body),
        Expr::Univ | Expr::Lib(_) | Expr::PathCon(_) => BTreeSet::new(),
        Expr::Var(index) => {
            let mut deps = BTreeSet::new();
            let index = *index as usize;
            if index > binder_depth {
                let shifted = index - binder_depth;
                if let Some(dep) = current.checked_sub(shifted) {
                    deps.insert(dep);
                }
            }
            deps
        }
        Expr::Id(ty, left, right) => {
            let mut deps = entry_deps(current, binder_depth, ty);
            deps.extend(entry_deps(current, binder_depth, left));
            deps.extend(entry_deps(current, binder_depth, right));
            deps
        }
        Expr::Refl(expr)
        | Expr::Susp(expr)
        | Expr::Trunc(expr)
        | Expr::Flat(expr)
        | Expr::Sharp(expr)
        | Expr::Disc(expr)
        | Expr::Shape(expr)
        | Expr::Next(expr)
        | Expr::Eventually(expr) => entry_deps(current, binder_depth, expr),
    }
}

fn strongly_connected_components(graph: &ClauseGraph, node_count: usize) -> Vec<Vec<ClauseIx>> {
    struct Tarjan<'a> {
        graph: &'a ClauseGraph,
        next_index: usize,
        stack: Vec<ClauseIx>,
        on_stack: BTreeSet<ClauseIx>,
        indices: Vec<Option<usize>>,
        lowlinks: Vec<usize>,
        components: Vec<Vec<ClauseIx>>,
    }

    impl<'a> Tarjan<'a> {
        fn visit(&mut self, node: ClauseIx) {
            self.indices[node] = Some(self.next_index);
            self.lowlinks[node] = self.next_index;
            self.next_index += 1;
            self.stack.push(node);
            self.on_stack.insert(node);

            if let Some(neighbors) = self.graph.get(&node) {
                for &neighbor in neighbors {
                    if self.indices[neighbor].is_none() {
                        self.visit(neighbor);
                        self.lowlinks[node] = self.lowlinks[node].min(self.lowlinks[neighbor]);
                    } else if self.on_stack.contains(&neighbor) {
                        let neighbor_index =
                            self.indices[neighbor].expect("neighbor index should exist");
                        self.lowlinks[node] = self.lowlinks[node].min(neighbor_index);
                    }
                }
            }

            if self.lowlinks[node] == self.indices[node].expect("node index should exist") {
                let mut component = Vec::new();
                while let Some(popped) = self.stack.pop() {
                    self.on_stack.remove(&popped);
                    component.push(popped);
                    if popped == node {
                        break;
                    }
                }
                component.sort_unstable();
                self.components.push(component);
            }
        }
    }

    let mut tarjan = Tarjan {
        graph,
        next_index: 0,
        stack: Vec::new(),
        on_stack: BTreeSet::new(),
        indices: vec![None; node_count],
        lowlinks: vec![0; node_count],
        components: Vec::new(),
    };

    for node in 0..node_count {
        if tarjan.indices[node].is_none() {
            tarjan.visit(node);
        }
    }

    tarjan.components
}

fn is_terminal_component(graph: &ClauseGraph, component: &[ClauseIx]) -> bool {
    let members: BTreeSet<_> = component.iter().copied().collect();
    let outgoing: BTreeSet<_> = component
        .iter()
        .flat_map(|index| graph.get(index).into_iter().flatten().copied())
        .collect();

    outgoing.difference(&members).next().is_none()
}

#[cfg(test)]
mod tests {
    use super::{build_clause_dependency_graph, terminal_clause_sccs, terminal_scc_sub_bundles};
    use pen_core::telescope::Telescope;

    #[test]
    fn reference_pi_graph_has_a_terminal_root_component() {
        let telescope = Telescope::reference(4);
        let graph = build_clause_dependency_graph(&telescope);

        assert_eq!(graph.get(&0).cloned().unwrap_or_default().len(), 0);
        assert_eq!(terminal_clause_sccs(&telescope), vec![vec![0]]);
        assert_eq!(terminal_scc_sub_bundles(&telescope).len(), 1);
    }
}
