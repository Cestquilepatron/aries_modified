use aries::planning::classical::search::*;
use aries::planning::classical::{from_chronicles, grounded_problem};
use aries::planning::parsing::pddl_to_chronicles;

//ajout pour initialisation de l'historique
use aries::planning::classical::state::*;

fn main() -> Result<(), String> {
    let arguments: Vec<String> = std::env::args().collect();
    if arguments.len() != 3 {
        return Err("Usage: ./gg <domain> <problem>".to_string());
    }
    let dom_file = &arguments[1];
    let pb_file = &arguments[2];

    let dom = std::fs::read_to_string(dom_file).map_err(|o| format!("{}", o))?;

    let prob = std::fs::read_to_string(pb_file).map_err(|o| format!("{}", o))?;

    let spec = pddl_to_chronicles(&dom, &prob)?;

    let lifted = from_chronicles(&spec)?;

    let grounded = grounded_problem(&lifted)?;

    let symbols = &lifted.world.table;

    match plan_search(
        &grounded.initial_state,
        &grounded.operators,
        &grounded.goals,
    ) {
        Some(plan) => {
            // creation 
            let plan2=plan.clone();

            for sta in grounded.initial_state.literals(){
               println!("init state: {} ",sta.val()); 
            }
            println!("init size : {}", grounded.initial_state.size());
            println!("=============");
            println!("Got plan: {} actions", plan.len());
            println!("=============");

            let mut etat = grounded.initial_state.clone();
            let mut histo = Vec::new();
            let mut affichage=Vec::new();
            let mut count =0;
            let mut index =0;
            while index < etat.size() {
                let init=defaultresume();
                histo.push(init);
                index=index+1;
            }

            for &op in &plan {
                //inserer la création de l'état intermediaire ici
                
                //etat=step(&etat,&op,&grounded.operators);
                let (e,h)=h_step(&etat,&op,&grounded.operators,count,histo);
                etat=e;
                histo=h.clone();
                println!("{}", symbols.format(grounded.operators.name(op)));
                if count ==10{
                    affichage=h.clone();
                }

                compare(&etat,&grounded.initial_state);
                count=count+1;
            }


            println!("=============");
            println!("affichage historique etape 10");
            println!("=============");
            let mut var=0;
            for res in affichage{
                if res.numero()>=0 {
                    let opr=res.op();
                    let opr=opr.unwrap();
                    let affiche = &grounded.operators.name(opr);
                    //terminer affichage afficher operator lié à l'Op opr
                    println!("variable {}, {} dernier opérateur à l'avoir modifié, durant l'étape {}", var,symbols.format(affiche) ,res.numero() );
                    let pre=grounded.operators.preconditions(opr);
                    //println!(" précond {}",*pre.val());
                }
                var=var+1;
            }


            println!("=============");
            println!("affichage cause opérateur");
            println!("=============");
            let cause=causalite(10,plan,&grounded.initial_state,&grounded.operators);
            let op=plan2.get(10).unwrap();
            let opname=&grounded.operators.name(*op);
            println!("Affichage des Opérateur nécessaire à {} de l'étape {}",symbols.format(opname),10);
            println!("=========");
            for res in cause{
                match res.op(){
                    None => println!("variable non changé depuis l'état initial"),
                    Some(Resume)=>println!("{}, de l'étape {}",symbols.format(&grounded.operators.name(res.op().unwrap())),res.numero()),
                    _ => (),
                }
                
            }


            println!("=============");
            println!("GOALS");
            let iterbut = grounded.goals.iter();
            for but in iterbut{
               println!("goal state: {} ",but.val()); 
            }

        }
        None => println!("Infeasible"),
    }

    Ok(())
}
