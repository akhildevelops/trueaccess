use rstest::*;
mod fixtures;
use fixtures::linux::*;
use std::sync::mpsc;
use std::thread;
use std::time::Duration;
use trueaccess::linux::user::Linux;
use trueaccess::user::User;
#[rstest]
fn check_connection(docker_linux: Linux) {
    docker_linux
        .create_session()
        .unwrap()
        .create_user("dumm27", "asdf")
        .unwrap();
}

fn time_out<T, F, K>(mut f: F, args: K, duration: Duration) -> Result<T, mpsc::RecvTimeoutError>
where
    F: FnMut(K) -> T + Send + 'static,
    T: Send + 'static,
    K: Send + 'static,
{
    let (tx, rx) = mpsc::channel();
    thread::spawn(move || tx.send(f(args)));

    rx.recv_timeout(duration)
}

#[rstest]
fn sample() {
    use ssh2::{Channel, Session};
    use std::error::Error;
    use std::io::prelude::*;
    use std::net::TcpStream;

    const HELLO: &str = "<hello xmlns=\"urn:ietf:params:xml:ns:netconf:base:1.0\">
  <capabilities>
    <capability>urn:ietf:params:netconf:base:1.1</capability>
  </capabilities>
</hello>
]]>]]>";

    const PAYLOAD: &str = "<?xml version=\"1.0\" encoding=\"UTF-8\"?>
    <rpc xmlns=\"urn:ietf:params:xml:ns:netconf:base:1.1\" message-id=\"2\">
    <cli xmlns=\"http://cisco.com/ns/yang/cisco-nx-os-device\"><mode>EXEC</mode><cmdline>show version</cmdline></cli>
</rpc>";

    fn read(channel: &mut Channel) -> Result<String, Box<dyn Error>> {
        let mut result = String::new();
        // dbg!(channel.wait_eof().unwrap());
        loop {
            // If you plan to use this, be aware that reading 1 byte at a time is terribly
            // inefficient and should be optimized for your usecase. This is just an example.
            let mut buffer = [1u8; 1];
            let bytes_read = match time_out(
                |x| channel.read(x),
                &mut buffer[..],
                Duration::from_millis(1000),
            ) {
                Ok(x) => x.unwrap(),
                Err(_) => break,
            };
            let s = String::from_utf8_lossy(&buffer[..bytes_read]);
            result.push_str(&s);
            if result.ends_with("]]>]]>") {
                println!("Found netconf 1.0 terminator, breaking read loop");
                break;
            }
            if result.ends_with("##") {
                println!("Found netconf 1.1 terminator, breaking read loop");
                break;
            }
            if bytes_read == 0 || channel.eof() {
                println!("Buffer is empty, SSH channel read terminated");
                break;
            }
            dbg!(&bytes_read);
        }
        Ok(result)
    }

    fn main() -> Result<(), Box<dyn Error>> {
        let tcp = TcpStream::connect("localhost:4567")?;
        let mut sess = Session::new()?;
        sess.set_tcp_stream(tcp);
        sess.handshake().unwrap();
        sess.userauth_password("test", "test")?;

        let mut channel = sess.channel_session()?;
        channel.shell()?;
        sess.set_blocking(false);
        let result = read(&mut channel)?;
        println!("Result from connection:\n{}", result);

        let payload = format!("{}\n#{}\n{}\n##\n", HELLO, PAYLOAD.len(), PAYLOAD);
        let a = channel.write(payload.as_bytes())?;
        println!("Written {} bytes payload", a);
        let result = read(&mut channel)?;
        println!("Result from payload execution:\n{}", result);
        // sess.set_blocking(true);
        // channel.send_eof()?;
        // channel.wait_eof()?;
        channel.close()?;
        // channel.wait_close()?;
        Ok(())
    }
    main().unwrap();
}
