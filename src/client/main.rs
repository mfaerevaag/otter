extern crate ws;

use ws::{connect, CloseCode};

fn main () {

    if let Err(error) = connect("ws://127.0.0.1:10000", |out| {

        // queue a message to be sent when the WebSocket is open
        if let Err(_) = out.send("hello") {
            println!("failed to queue initial message")
        } else {
            println!("sent 'hello'")
        }

        // the handler needs to take ownership of out, so we use move
        move |msg| {

            // handle messages received on this connection
            println!("received '{}'", msg);

            // close the connection
            out.close(CloseCode::Normal)
        }

    }) {
        println!("failed to connect: {:?}", error);
    }

}
