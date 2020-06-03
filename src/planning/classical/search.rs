use crate::planning::classical::heuristics::*;
use crate::planning::classical::state::*;
use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};

struct Node {
    s: State,
    plan: Vec<Op>,
    f: Cost,
}

impl PartialOrd for Node {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        Cost::cmp(&self.f, &other.f).reverse()
    }
}

impl PartialEq for Node {
    fn eq(&self, other: &Self) -> bool {
        self.cmp(other) == Ordering::Equal
    }
}

impl Eq for Node {}

const WEIGHT: Cost = 3;

pub fn plan_search(initial_state: &State, ops: &Operators, goals: &[Lit]) -> Option<Vec<Op>> {
    let mut heap = BinaryHeap::new();
    let mut closed = HashSet::new();

    let init = Node {
        s: initial_state.clone(),
        plan: Vec::new(),
        f: 0,
    };
    heap.push(init);

    while let Some(n) = heap.pop() {
        if closed.contains(&n.s) {
            continue;
        }
        closed.insert(n.s.clone());
        let hres = hadd(&n.s, ops);
        for &op in hres.applicable_operators() {
            debug_assert!(n.s.entails_all(ops.preconditions(op)));
            let mut s = n.s.clone();
            s.set_all(ops.effects(op));

            let mut plan = n.plan.clone();
            plan.push(op);

            if s.entails_all(goals) {
                return Some(plan);
            }

            let hres = hadd(&s, ops);
            let f = (plan.len() as Cost) + 3 * hres.conjunction_cost(goals);
            let succ = Node { s, plan, f };
            heap.push(succ);
        }
    }

    None
}

pub fn step(initial_state: &State, op: &Op, ops: &Operators)-> State {
    let mut suivant =  initial_state.clone();
    let preco =ops.preconditions(*op);
    let effect = ops.effects(*op);

    //inutile le plan est censé etre bon 
    //mais si jamais ont fais tourné sur un plan en construction
    debug_assert!(initial_state.entails_all(preco));
    //passage des effets de l'action sur l'état pour avoir l'état intermediaire
    suivant.set_all(effect);
    suivant
}

//
pub fn compare(initial_state: &State, inter_state: &State){
    let mut diff = 0;
    for lit in initial_state.literals(){
        for liter in inter_state.literals(){
            if lit.var() == liter.var(){
                if lit.val()!=liter.val(){
                    diff=diff+1;
                }
            }
        }
    }
    println!("il y a {} changments entre les états",diff);
}


pub fn h_step(initial_state: &State, op: &Op, ops: &Operators, numstep: i32, histo: Vec<Resume>)-> (State,Vec<Resume>){
    let etat=step(initial_state,op,ops);

    let mut count=0;
    let mut newhisto= Vec::new();

    //parcours des vecteurs etatique
    for lit in initial_state.literals(){
        for liter in etat.literals(){
            if lit.var() == liter.var(){
                if lit.val()!=liter.val(){
                    //création d'un nouveau resume et incorporation à l'historique
                    let resume=newresume(*op,numstep);
                    newhisto.push(resume);
                }else{
                    //rien ne change on reprend l'ancien historique
                    let oldresume=histo.get(count);
                    //newhisto.push(oldresume);
                    //j'ai essayé mais ça ne fonctionne pas plus
                    newhisto.push(Some(oldresume));
                }
                count=count+1;            
            }
        }
    }
    (etat,newhisto)
}
/*
pub fn causalite(etape: i64, initial_state: &State, op: &Op, ops: &Operators)->Vec<Op>{
    //initialisation

    //etape construction histogramme lié
    hstep

    //liaison opérateur grâce à histogramme et précondition opé


}*/
