use rand::{self, Rng};
use std::cmp::Ordering;

#[derive(Debug)]
struct Node {
    id: i32,
    inflow: i32,
    outflow: i32,
    value: i32
}

#[derive(Debug)]
struct Edge {
    a: i32,
    b: i32,
    value: i32
}
 
impl Node {

    fn new(id: i32 , inflow: i32 , outflow: i32 , value: i32) -> Self {
	Self {
	    id,
	    inflow,
	    outflow,
	    value
	}
    }
    
    // fn value(&self) -> i32 {
    // 	self.inflow - self.outflow
    // }
}

fn generate_input() -> Vec<Node>{
    let mut rng = rand::thread_rng();
    let mut output = vec![];
    let mut total_value: i32 = 0;

    let custom_range = 1..10;
    
    for i in custom_range.clone() {
	let in_flow = rng.gen_range(custom_range.clone());
	let out_flow = rng.gen_range(custom_range.clone());
	let temp = Node::new(i , in_flow , out_flow , in_flow - out_flow);
	// println!{"{}" , temp.value};
	total_value += temp.value;
	output.push(temp);
    }

    let final_node = Node::new(10 , if total_value < 0 {-total_value} else {0} , if total_value > 0 {total_value} else {0} , -total_value);

    output.push(final_node);
    output
}

fn log<T>(s: &str , x: T) -> T
where T: std::fmt::Debug
{
    println!("\t{s}: {:?}" , x);
    x
}

fn custom_zip<'a>(seeders: &mut Vec<Node> , leechers: &mut Vec<Node> , output: &'a mut Vec<Edge>) -> &'a mut Vec<Edge> {
    // log("\tSeeders Length" , seeders.len());
    // log("\tLeechers Length" , leechers.len());
    match (seeders.is_empty() ,  leechers.is_empty()) {
	(true , true) => output,
	(true , false) => output,
	(false , true) => output,
	// can use ordering. Gimma some time.
	(false , false) => {

	    let result = seeders.last().unwrap().value.cmp(&-leechers.last().unwrap().value);
	    // log("Result" , result);

	    // log("Seeders" , &seeders);
	    // log("Leechers" , &leechers);

	    match result {
		Ordering::Greater => {
		    let mut temp_leecher: Node =  leechers.pop().unwrap();
		    let mut temp_seeder: Node = seeders.pop().unwrap();

		    // log("Temp Seeder" , &temp_seeder);
		    // log("Temp Leecher" , &temp_leecher);
		    output.push(Edge {
			a: temp_leecher.id ,
			b: temp_seeder.id ,
			value: temp_leecher.value
		    });
		    temp_seeder.value += temp_leecher.value;
		    seeders.push(temp_seeder);
		    // log("Seeders" , &seeders);
		    // log("Leechers" , &leechers);
		    custom_zip(seeders , leechers , output)
		},
		Ordering::Less => {
		    let mut temp_seeder: Node =  seeders.pop().unwrap();
		    let mut temp_leecher: Node = leechers.pop().unwrap();

		    // log("Temp Seeder" , &temp_seeder);
		    // log("Temp Leecher" , &temp_leecher);
		    output.push(Edge {
			a: temp_leecher.id ,
			b: temp_seeder.id ,
			value: temp_seeder.value
		    });

		    temp_leecher.value += temp_seeder.value;
		    leechers.push(temp_leecher);
		    custom_zip(seeders , leechers , output)
		},
		Ordering::Equal => {
		    let seeder = seeders.pop().unwrap();
		    let leecher = leechers.pop().unwrap();
		    output.push( Edge {
			a: leecher.id,
			b: seeder.id,
			value: seeder.value
		    });
		    custom_zip(seeders , leechers , output)
		}
	    }
	}
    }
}

fn t_value(nodes: &[Node]) -> i32 {
    let mut total_value = 0;
    for x in nodes.iter() {
	total_value += x.value;
    }
    total_value
}

pub fn run() {
    // let input = generate_input();
    let input = vec![
	Node::new(1 , 20 , 10 , 10),
	Node::new(2 , 15 , 0 , 15),
	Node::new(3 , 0 , 25 , -25)
    ];
    println!("Total Value of Graph ------> {:?}" , t_value(&input));

    let non_zero_value_input = input.into_iter().filter(|x| x.value != 0);
    
    let (mut seeders , mut leechers): (Vec<_> , Vec<_>) = non_zero_value_input.into_iter().partition(|x| x.value > 0);

    for i in seeders.iter() {
	println!("\nSeeder -------> {:?}\n" , i);
    }

    for i in leechers.iter() {
	println!("\nLeecher -------> {:?}\n" , i);
    }
    
    println!("Total Value of Seeder Graph ------> {:?}" , t_value(&seeders));
    println!("Total Value of Leacher Graph ------> {:?}" , t_value(&leechers));

    let mut output: Vec<Edge> = vec![];
    let final_output = custom_zip(&mut seeders , &mut leechers , &mut output);
    for i in final_output.iter() {
	println!("{:?}" , i);
    }
}


// Step 1 : Check whether value of seeder or leacher is greater.
// Step 2 : Take smaller value and pop that node.
// Step 3 : Create an edge from popped node to existing node.
// Step 4: Reduce existing node value - popped node value.
// Step 5 : Repeat.
