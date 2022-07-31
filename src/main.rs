use simple_websockets::{Event, Responder, Message};
use std::{collections::HashMap};
use openssl::error::ErrorStack;
use openssl::ssl::{SslConnector, SslFiletype, SslMethod, SslVerifyMode};
use postgres_openssl::MakeTlsConnector;
use postgres::Client;
extern crate postgres;
// use postgres::{Client, Error as postgres_, NoTls};

use crate::model::UserLocation;
use std::error::Error;

mod model;

fn ssl_config() -> Result<MakeTlsConnector, ErrorStack> {
    let mut builder = SslConnector::builder(SslMethod::tls())?;
    builder.set_ca_file("./cert.pem")?;
    let connector = MakeTlsConnector::new(builder.build());
    // builder.set_verify(SslVerifyMode::NONE);
    Ok(connector)
}

fn main() {
    // listen for WebSockets on port 8080:
    let event_hub = simple_websockets::launch(5050)
        .expect("failed to listen on port 8080");
    // map between client ids and the client's `Responder`:
    let mut clients: HashMap<u64, Responder> = HashMap::new();


    let connector = ssl_config().unwrap();
    let mut db_client =
        Client::connect("postgresql://doadmin:AVNS_EKK8c_nqpRc6vx_thcC@db-postgresql-fra1-49922-do-user-12132994-0.b.db.ondigitalocean.com:25060/defaultdb", connector).unwrap();
        // dbClient.execute("SELECT * FROM location_data", &[]).unwrap();

    loop {
        match event_hub.poll_event() {
            Event::Connect(client_id, responder) => {
                println!("A client connected with id #{}", client_id);
                // add their Responder to our `clients` map:
                clients.insert(client_id, responder);
            },
            Event::Disconnect(client_id) => {
                println!("Client #{} disconnected.", client_id);
                // remove the disconnected client from the clients map:
                clients.remove(&client_id);
            },
            Event::Message(client_id, message) => {
                println!("Received a message from client #{}: {:?}", client_id, message);
                let abcd = resend_messages(&clients, &message, &mut db_client); 
                abcd;
                
            },
        }
    }
    
    fn resend_messages(clients: &HashMap<u64, Responder>, value: &Message, database: &mut Client) -> Result<(), Box<dyn Error>> { 
        // let mut dbClient = Client::connect("postgresql://aid:dtldtl123@localhost:5432/postgres", NoTls).unwrap();

        for client in clients.iter() {
            let client = client.1;
            let message = value.clone();
            
            match &message {
                Message::Text(string) => { 
                    client.send(value.clone());

                    let userLocation: model::UserLocation = serde_json::from_str(&string)?;

                    database.execute(
                        "INSERT INTO location_data (altitude, latitude, name, longtitude, uuid, date, typeg) VALUES ($1, $2, $3, $4, $5, $6, $7)",
                        &[&userLocation.altitude, &userLocation.latitude, &userLocation.name, &userLocation.longtitude, &userLocation.uuid, &userLocation.date, &userLocation.typeg],
                     )?;
                   

                    // print!(userLocation)
                    print!("THISIS {:?}", userLocation);
                },
                Message::Binary(data) => print!("binary"),
                
            }
        }
        return Ok(()); 
    } 

    // fn test(string: String) -> Result<model::UserLocation, Error> {
    //     return Err();
    // }
}