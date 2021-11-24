use actix::prelude::*;

struct PingActor {
    count: usize,
}

impl Actor for PingActor {
    type Context = Context<Self>;
}

#[derive(Message)]
#[rtype(result = "usize")]
struct Ping(usize);

impl Handler<Ping> for PingActor {
    type Result = usize;

    fn handle(&mut self, message: Ping, _ctx: &mut Context<Self>) -> Self::Result {
        self.count += message.0;
        self.count
    }
}

#[actix_rt::main]
async fn main() {
    let ping_actor_address = PingActor { count: 10 }.start();
    let ping_actor_response = ping_actor_address.send( Ping(10) ).await;
    println!("ping actor response is: {}", ping_actor_response.unwrap());
    System::current().stop();
}