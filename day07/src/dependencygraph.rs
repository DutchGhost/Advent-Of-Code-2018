use std::{
    hash::Hash,
    collections::{HashMap, HashSet},
    cmp::Eq,
};

pub trait Dependents {
    type Dependency;

    fn empty() -> Self;

    fn add_dependency(&mut self, dependency: Self::Dependency);

    fn remove_dependency(&mut self, dependency: &Self::Dependency);
}

impl <D: Hash + Eq> Dependents for HashSet<D> {
    type Dependency = D;

    #[inline(always)]
    fn empty() -> Self {
        Self::new()
    }
    
    #[inline(always)]
    fn add_dependency(&mut self, dependency: Self::Dependency) {
        self.insert(dependency);
    }

    #[inline(always)]
    fn remove_dependency(&mut self, dependency: &Self::Dependency) {
        self.remove(dependency);
    }
}

/// This datastructs maps any task T
/// to a list of dependencies `Dep`.
pub struct DependencyGraph<T, Dep> {
    graph: HashMap<T, Dep>,
}

impl <T: Eq + Hash, Dep: Dependents> DependencyGraph<T, Dep> {
    /// Returns a new empty graph.
    #[inline(always)]
    pub fn new() -> Self {
        Self { graph: HashMap::new() }
    }
    
    /// Returns `true` if the graph is empty,
    /// false otherwise.
    #[inline(always)]
    pub fn is_empty(&self) -> bool {
        self.graph.is_empty()
    }

    /// Returns an iterator over the (Task, Dependents) pairs
    /// in the graph
    #[inline(always)]
    pub fn iter<'a>(&'a self) -> impl Iterator<Item = (&T, &Dep)> + 'a {
        self.graph.iter()
    }
    
    /// Returns an iterator that yields mutable Deps.
    #[inline(always)]
    pub fn values_mut<'a>(&'a mut self) -> impl Iterator<Item = &mut Dep> + 'a {
        self.graph.values_mut()
    }

    /// Attemts to lookup the task in the map.
    /// If its in the map, return the dependency's
    /// associated with it.
    /// If its not in the map, insert it associated
    /// with an empty dependency list.
    pub fn task(&mut self, task: T) -> &mut Dep {
        self.graph.entry(task).or_insert_with(Dep::empty)
    }

    pub fn remove_task(&mut self, task: &T) {
        self.graph.remove(task);
    }
}
