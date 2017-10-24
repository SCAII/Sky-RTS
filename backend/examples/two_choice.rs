extern crate backend;
extern crate scaii_defs;
extern crate websocket;
extern crate prost;
extern crate rand;

use std::error::Error;

use scaii_defs::protos::ScaiiPacket;

use websocket::client::sync::Client;
use websocket::stream::sync::TcpStream;

fn main() {
    use rand;
    use rand::Rng;
    use std::thread;
    use std::time::Duration;
    use backend::engine::system::trigger::VictoryState;
    use backend::engine::system::System;

    use scaii_defs::protos::Action;
    use backend::protos;
    use backend::protos::unit_action::Action::MoveTo;

    use prost::Message;

    let mut conn = connect();
    server_startup(&mut conn);

    let mut rts = backend::engine::Rts::new();
    rts.two_setup();

    let sleep_time = Duration::from_millis(17);

    let mut rng = rand::thread_rng();

    let action: usize = rng.gen_range(1,3);
    // println!("{:?}", rts.movement_system.component_map());
    let choice = rts.movement_system.component_map().get(&action).unwrap().clone();

    let rts_action = protos::ActionList {
        actions: vec![
            protos::UnitAction {
                unit_id: 0,
                action: Some(MoveTo(protos::MoveTo { pos: protos::Pos{x: choice.x, y: choice.y}})),
            }
        ]
    };

    let mut buf: Vec<u8> = Vec::new();
    rts_action.encode(&mut buf).expect(
        "Could not encode action"
    );


    let action = Action {
        discrete_actions: vec![],
        continuous_actions: vec![],

        alternate_actions: Some(buf),
    };

    let (mut msg,_) = rts.update(Some(&action));
    let msg = msg.packets.pop().unwrap();
    encode_and_send_proto(&mut conn, &msg);
    receive_and_decode_proto(&mut conn);

    loop {
        thread::sleep(sleep_time);

        let (mut msg,vict) = rts.update(None);

        println!("{:?}", msg);

        encode_and_send_proto(&mut conn, &msg.packets.pop().unwrap());
        receive_and_decode_proto(&mut conn);

        match vict {
            VictoryState::Victory => { println!("Won with reward 100");  return; },
            VictoryState::Defeat => { println!("Lost with reward -100");  return; },
            VictoryState::Continue => {},
        }
    }
}

fn make_viz_init() -> ScaiiPacket {
    use scaii_defs::protos;
    use scaii_defs::protos::{endpoint, scaii_packet};
    ScaiiPacket {
        src: protos::Endpoint {
            endpoint: Some(endpoint::Endpoint::Backend(protos::BackendEndpoint {})),
        },
        dest: protos::Endpoint {
            endpoint: Some(endpoint::Endpoint::Module(
                protos::ModuleEndpoint { name: "viz".to_string() },
            )),
        },

        specific_msg: Some(scaii_packet::SpecificMsg::VizInit(protos::VizInit {})),
    }
}

fn server_startup(client: &mut Client<TcpStream>) {
    let viz_init = make_viz_init();
    encode_and_send_proto(client, &viz_init).expect("Could not send VizInit message");
}

fn connect() -> Client<TcpStream> {
    use websocket::sync::Server;
    use std::time::Duration;
    use std::net::SocketAddr;

    let addr = SocketAddr::new(From::from([127,0,0,1]), 6112);
    let mut server = Server::bind(addr).expect("Could not bind server to provided ip/port");
    // For some reason, we can't use expect here because apparently the result doesn't implement
    // the Debug trait?
    let connection = match server.accept() {
        Err(err) => panic!(err),
        Ok(connection) => connection,
    };

    /* Set timeouts to 5 seconds. Fine for tests, in core
    we'll probably make this configurable */
    connection
        .tcp_stream()
        .set_read_timeout(Some(Duration::new(500, 0)))
        .expect("Could not change read timeout on socket");

    connection
        .tcp_stream()
        .set_write_timeout(Some(Duration::new(500, 0)))
        .expect("Could not change write timeout on socket");

    println!("Connection accepted\n");
    connection.accept().expect("Couldn't accept the connection")
}

fn encode_and_send_proto(
    client: &mut Client<TcpStream>,
    packet: &ScaiiPacket,
) -> Result<(), Box<Error>> {
    use prost::Message;
    use websocket::message;

    let mut buf: Vec<u8> = Vec::new();
    packet.encode(&mut buf).expect(
        "Could not encode SCAII packet (server error)",
    );

    client.send_message(&message::Message::binary(buf))?;
    Ok(())
}

fn receive_and_decode_proto(client: &mut Client<TcpStream>)  {
    let msg = client.recv_message();
}