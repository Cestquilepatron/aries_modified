use aries::planning::classical::search::{plan_search,step,compare,h_step};
use aries::planning::classical::{from_chronicles, grounded_problem};
use aries::planning::parsing::pddl_to_chronicles;

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
            let oprien= 
            
            for sta in grounded.initial_state.literals(){
               println!("init state: {} ",sta.val()); 
            }
            println!("init size : {}", grounded.initial_state.size());
            println!("=============");
            println!("Got plan: {} actions", plan.len());
            println!("=============");

            let mut etat = grounded.initial_state.clone();
            let mut histo = Vec::new();

            for &op in &plan {
                //inserer la création de l'état intermediaire ici
                
                //etat=step(&etat,&op,&grounded.operators);
                let (e,h)=h_step(&etat,&op,&grounded.operators,histo);
                etat=e;
                histo=h;
                println!("{}", symbols.format(grounded.operators.name(op)));

                compare(&etat,&grounded.initial_state);
            }

            println!("=============");
            let iterbut = grounded.goals.iter();
            for but in iterbut{
               println!("goal state: {} ",but.val()); 
            }

        }
        None => println!("Infeasible"),
    }

    Ok(())
}
