use mysql::{Opts, OptsBuilder, Pool, PooledConn};
use once_cell::unsync::Lazy;
use std::env;
use std::sync::{Arc, Mutex};
use std::time::Duration;

pub static mut CONNECTION: Lazy<Mutex<Pool>> = make_connection();

const fn make_connection() -> Lazy<Mutex<Pool>> {
    Lazy::new(|| {
        let connection_string =
            env::var("CONNECTION_STRING").expect("CONNECTION_STRING must be set");

        let opts = Opts::from_url(&connection_string).unwrap();
        let opts =
            OptsBuilder::from_opts(opts)
                .tcp_connect_timeout(
                    Some(Duration::from_millis(20000))
                );
        let pool = Pool::new(opts).unwrap();

        // let connection =  pool.get_conn();
        // let mut connection = connection.unwrap();
        // connection.reset_connection(true);
        Mutex::new(pool)
    })
}

pub fn connect<'a>() -> mysql::Result<PooledConn>
//-> LockResult<&'a mut PooledConn>
{
    unsafe {
        // let locked_connection = CONNECTION.get_mut().unwrap().clo;
        // let mut connection = locked_connection.unwrap();
        //
        // if !connection.{
        //     CONNECTION = make_connection();
        //     return CONNECTION.get_mut()
        // }
        let connection = CONNECTION
            .get_mut()?
            .try_get_conn(Duration::from_millis(200));
        // if let Err(I) = &connection{
        //     println!("{:?}", I)
        // }
        // if let Ok(I) = &connection{
        //     println!("{:?}", I)
        // }
        connection
    }
}

// pub fn  connect() -> Mutex<PooledConn> {
//     // let connection_string =
//     //     env::var("CONNECTION_STRING")
//     //         .expect("CONNECTION_STRING must be set");
//     //
//     // let opts = Opts::from_url(&connection_string)?;
//     //
//     // let pool = Pool::new(opts)?;
//
//     return CONNECTION.connection
// }
