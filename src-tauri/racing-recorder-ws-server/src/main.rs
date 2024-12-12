use std::{net::SocketAddr, time::Duration};
use futures_util::{SinkExt, StreamExt};
use log::*;
use simple_logger::SimpleLogger;
use tokio::net::{TcpListener,TcpStream};
use tokio_tungstenite::{
    accept_async, tungstenite::{Error,Message,Result}
};
use serde::{Deserialize,Serialize};
//use serde_json::{Result as serdeResult,Value,Value::Object,Map};
use racing_recorder_lib::{RecorderBuilder,Game,common::RecorderTrait::Recorder};

#[derive(Deserialize)]
struct RecordCommand{
    pub command: String,
    pub game: String
}

#[derive(Deserialize,Serialize)]
struct TelemetryData{
    pub speed:f32,
    pub throttle:f32,
    pub brake:f32,
    pub steering:f32,
    pub current_time:i32,
    pub lap:i32
}

async fn accept_connection(peer: SocketAddr, stream: TcpStream){
    if let Err(e) = handle_connection(peer, stream).await {
        match e {
            Error::ConnectionClosed |Error::Protocol(_) |Error::Utf8 => (),
            err => error!("Error processing connection: {}",err),
        }
    }
}

async fn handle_connection(_peer: SocketAddr,stream: TcpStream)->Result<()>{
    let ws_stream = accept_async(stream).await.expect("Failed to accept");
    //let tcpStream:TcpStream = ws_stream.into();
    let (mut ws_sender,mut ws_reciver) = ws_stream.split();
    let mut interval = tokio::time::interval(Duration::from_millis(50));
    let mut recording: Option<Box<dyn Recorder + Send>> = Option::None;
    
    loop {
        tokio::select! {
            msg = ws_reciver.next() => {
                match msg {
                    Some(msg) => {
                        let msg = msg?;
                        
                        if msg.is_text() {
                            let data = msg.into_text()?;
                            let o_data:RecordCommand = serde_json::from_str(data.as_str()).unwrap();
                            if o_data.command == "record" {
                                if o_data.game == "acc" {
                                    recording = Some(RecorderBuilder::new().setGame(Game::AssettoCorsaCompetizione).build());
                                } else if o_data.game == "rr" {
                                    recording = Some(RecorderBuilder::new().setGame(Game::RaceRoom).build());
                                }
                            }
                        } else if msg.is_close() {
                            break;
                        }
                        
                        /*
                        if msg.is_text() ||msg.is_binary() {
                            ws_sender.send(msg).await?;
                        } else if msg.is_close() {
                            break;
                        }
                         */
                        
                    }
                    None => break,
                }
            }
            _ = interval.tick() => {
                //ws_sender.send(Message::Text("tick".to_owned())).await?
                
                match recording.as_ref(){
                    Some(recorder) => {
                        let data = recorder.getData();
                        let data_send = TelemetryData {
                            speed: data.speed,
                            throttle: data.throttle,
                            brake: data.brake,
                            steering: data.steering,
                            current_time: data.current_time,
                            lap: data.lap
                        };
                        ws_sender.send(Message::Text(serde_json::to_string(&data_send).unwrap().to_owned())).await?
                    },
                    _ => ()
                }
                
                
            }
        }
    }
    
    Ok(())
}

#[tokio::main]
async fn main() {
    //env_logger::init();
    SimpleLogger::new().init().unwrap();
    let addr = "127.0.0.1:9002";
    let listener = TcpListener::bind(&addr).await.expect("Can't listen");
    println!("wait for incoming requests");
    while let Ok((stream, _)) = listener.accept().await {
        let peer = stream.peer_addr().expect("connect streams should have a peer address");
        tokio::spawn(accept_connection(peer, stream));
    }
}
