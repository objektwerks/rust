use actix::prelude::*;

struct CountActor {
    count: usize,
}

impl Actor for CountActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "usize")]
struct Count(usize);

impl Handler<Count> for CountActor {
    type Result = usize;

    fn handle(&mut self, message: Count,
              _ctx: &mut Context<Self>) -> Self::Result {
        self.count += message.0;
        self.count
    }
}

#[actix_rt::main]
async fn main() {
    let count_actor_address = CountActor { count: 0 }.start();
    let count_actor_response = count_actor_address.send( Count(1) ).await;
    println!("count actor response is: {}", count_actor_response.unwrap());
    System::current().stop();
}