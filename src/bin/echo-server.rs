use tokio::io::{self, AsyncReadExt, AsyncWriteExt};
use tokio::net::TcpListener;

// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut f = File::open("foo.txt").await?;
//     // let mut buffer = [0;10];

//     // read up to 10 bytes
//     // let n = f.read(&mut buffer[..]).await?;

//     // println!("The bytes: {:?}", &buffer[..n]);
//     // Read  the whole fiel
//     let mut buffer2 = Vec::new();
//     f.read_to_end(&mut buffer2).await?;
//     println!("The bytes: {:?}", buffer2);

//     Ok(())
// }

// #[tokio::main]

// async fn main() ->io::Result<()> {
//     let mut file = File::open("foo.txt").await?;
//     let n = file.write(b"some bytes").await?;
//     println!("Wrote the first {} bytes of 'some bytes' ", n);

//     Ok(())
// }

// write the entire buffer into the writer

// #[tokio::main]
// async fn main() ->io::Result<()>{
//     let mut file = File::create("foo.txt").await?;
//     file.write_all(b"some bytes").await?;
//     Ok(())
// }

// Helper functions section
// tokio::io::copy asynchronously copies the entire contents of a reader into a writer
// #[tokio::main]
// async fn main() -> io::Result<()> {
//     let mut reader = b"hello";
//     let file = File::create("foox.txt").await?;
//     io::copy(&mut reader, &mut file).await?;
//     // file.write_all(&mut reader).await?;
//     Ok(())
// }

#[tokio::main]

async fn main() -> io::Result<()> {
    let listener = TcpListener::bind("127.0.0.16142").await?;
    loop {
        let (mut socket, _) = listener.accept().await?;
        tokio::spawn(async move {
            let mut buf = vec![0; 1024];
            loop {
                match socket.read(&mut buf).await {
                    Ok(0) => return ,
                    Ok(n) => {
                        // copy the data back to socket
                        if socket.write_all(&buf[..n]).await.is_err(){
                            println!("Error");
                            return ;
                        }
                    }
                    Err(_) => {
                        return ;
                    }
                }
            }
        });

    }
    
}