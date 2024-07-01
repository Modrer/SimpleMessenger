use futures_channel::mpsc::channel;
use rocket::futures::{SinkExt, StreamExt};
use rocket::Shutdown;

use crate::claims::Claims;
use crate::messages_stream::Message;
use crate::STREAM_ARRAY;

#[get("/?<claims>")]
pub fn hello(ws: ws::WebSocket, claims: Claims, mut shutdown: Shutdown) -> ws::Channel<'static> {
    let id = claims.id;

    unsafe {
        println!("{:?}", STREAM_ARRAY.array.lock().unwrap());
    }

    ws.channel(move |mut stream| Box::pin(async move {

        let (sink, mut inner_stream) = channel::<Message>(10240);

        unsafe{
            STREAM_ARRAY.insert(id,sink.clone());
        }

        loop {
            tokio::select! {
                _ = &mut shutdown => {
                    inner_stream.close();
                    drop(inner_stream);
                    break;
                }
                result = stream.next() => {
                    if let None = result{
                        unsafe{
                            STREAM_ARRAY.remove(id,&sink);
                        }
                        inner_stream.close();
                        drop(inner_stream);
                        break;
                    }
                    if let Some(_message) = result{
                        let _ = stream.send("Who are you and what you doing here?!!!".into()).await;
                    }
                },

                result = inner_stream.next() => {

                    let _ = stream.send(result.unwrap().to_string().into()).await;

                    },

            }
        }


        //unsafe { println!("{:?}", STREAM_ARRAY.array.lock().unwrap()); }

        Ok(())
    }))
}
