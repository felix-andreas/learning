use {
    std::{thread, time::Duration},
    web_server::{Method, Request, Response, StatusCode},
};

fn main() -> std::io::Result<()> {
    web_server::serve("127.0.0.1:4004".parse().unwrap(), router)
}

fn router(request: &Request) -> Response {
    match (request.method, request.path.as_str()) {
        (Method::Get, "/") => Response::new(StatusCode::Ok, "Hello world!"),
        (_, "/info") => Response::new(StatusCode::Ok, format!("{request:#?}")),
        (Method::Post, "/cookie") => Response::with_headers(
            StatusCode::Ok,
            vec![(
                "Set-Cookie".to_string(),
                "foo=bar; Secure; HttpOnly".to_string(),
            )],
            "set cookie",
        ),
        (Method::Post, "/echo") => {
            Response::new(StatusCode::Ok, request.body.as_deref().unwrap_or(""))
        }
        (Method::Get, "/sleep") => {
            thread::sleep(Duration::from_secs(5));
            Response::new(StatusCode::Ok, "slept for 5 seconds")
        }
        (_, _) => Response::new(StatusCode::NotFound, "Not Found"),
    }
}

/*
 * WEB SERVER
 */

mod web_server {
    use {
        crate::thread_pool::ThreadPool,
        std::{
            fmt::Write as _,
            io::{self, BufRead, BufReader, Read, Write},
            net::{SocketAddr, TcpListener, TcpStream},
            sync::{
                atomic::{AtomicBool, Ordering},
                Arc,
            },
        },
    };

    /*
     * Public
     */

    pub type Router = fn(&Request) -> Response;

    #[derive(Debug)]
    pub struct Request {
        pub method: Method,
        pub path: String,
        pub headers: Vec<(String, String)>,
        pub body: Option<String>,
    }

    #[derive(Debug, Clone, Copy)]
    pub enum Method {
        Get,
        Post,
    }

    #[derive(Debug)]
    pub struct Response {
        status: StatusCode,
        headers: Vec<(String, String)>,
        body: String,
    }

    impl Response {
        pub fn new(status: StatusCode, body: impl Into<String>) -> Response {
            Response {
                status,
                headers: vec![("Content-Type".to_string(), "text/plain".to_string())],
                body: body.into(),
            }
        }

        pub fn with_headers(
            status: StatusCode,
            headers: Vec<(String, String)>,
            body: impl Into<String>,
        ) -> Response {
            Response {
                status,
                headers: {
                    let mut x = vec![("Content-Type".to_string(), "text/plain".to_string())];
                    x.extend(headers);
                    x
                },
                body: body.into(),
            }
        }
    }

    #[derive(Debug, Clone, Copy)]
    #[repr(u16)]
    pub enum StatusCode {
        Ok = 200,
        BadRequest = 400,
        NotFound = 404,
    }

    impl StatusCode {
        fn code(&self) -> u16 {
            *self as u16
        }
        fn reason(&self) -> &str {
            match self {
                Self::Ok => "OK",
                Self::BadRequest => "Bad Request",
                Self::NotFound => "Not Found",
            }
        }
    }

    pub fn serve(address: SocketAddr, router: Router) -> std::io::Result<()> {
        let listener = TcpListener::bind(address)?;
        println!("info: listen to address {address}");

        let ctrlc_pressed = Arc::new(AtomicBool::new(false));
        {
            let ctrlc_pressed = Arc::clone(&ctrlc_pressed);
            ctrlc::set_handler(move || {
                ctrlc_pressed.store(true, Ordering::Relaxed);
                // HACK: we connect to the socket ourself to force main thread to wake up
                let _ = TcpStream::connect(address);
                println!("\nreceived ctrl+c, shutting down ...");
            })
            .unwrap();
        }

        let pool = ThreadPool::new(4);

        for stream in listener.incoming() {
            if ctrlc_pressed.load(Ordering::Relaxed) {
                break;
            }

            let stream = stream?;
            pool.execute(move || {
                if let Err(error) = handle_stream(stream, router) {
                    println!("error: failed to handle request: {error}");
                }
            });
        }

        Ok(())
    }

    /*
     * Private
     */

    fn handle_stream(stream: TcpStream, router: Router) -> io::Result<()> {
        let mut writer = stream.try_clone()?;
        let mut reader = BufReader::new(stream);

        let request = parse_request(&mut reader);

        let response = match &request {
            Ok(request) => router(request),
            Err(error) => Response::new(
                StatusCode::BadRequest,
                match error {
                    ParseRequestError::UnexpectedEof => {
                        "error: unexpected end of request".to_string()
                    }
                    ParseRequestError::InvalidRequestLine(line) => {
                        format!("error: invalid request line '{line}'")
                    }
                    ParseRequestError::InvalidMethod(method) => {
                        format!("error: invalid method '{method}'")
                    }
                    ParseRequestError::InvalidHeader(header) => {
                        format!("error: invalid header '{header}'")
                    }
                    ParseRequestError::Utf8Error => "error: invalid bytes".to_string(),
                },
            ),
        };

        let code = response.status.code();
        let reason = response.status.reason();
        let length = response.body.len();
        let headers: String = vec![("Content-Length".to_string(), length.to_string())]
            .into_iter()
            .chain(response.headers)
            .try_fold(String::new(), |mut buffer, (name, value)| {
                write!(buffer, "{name}: {value}\r\n").map(|_| buffer)
            })
            .unwrap();
        let body = response.body;

        writeln!(
            &mut writer,
            "HTTP/1.1 {code} {reason}\r\n{headers}\r\n{body}"
        )?;

        println!(
            "info: {:?} {} -> {code} {reason}",
            request
                .as_ref()
                .map(|x| format!("{:?}", x.method))
                .unwrap_or("Invalid".to_string()),
            request
                .as_ref()
                .map(|x| x.path.as_str())
                .unwrap_or("Invalid")
        );

        Ok(())
    }

    #[derive(Debug)]
    enum ParseRequestError {
        InvalidMethod(String),
        InvalidHeader(String),
        InvalidRequestLine(String),
        Utf8Error,
        UnexpectedEof,
    }

    // TODO: don't allocate but just reference slices in BufReader
    fn parse_request(reader: &mut BufReader<TcpStream>) -> Result<Request, ParseRequestError> {
        // REQUEST LINE
        let request_line = {
            let mut buffer = Vec::<u8>::new();
            reader
                .read_until(b'\n', &mut buffer)
                .map_err(|_| ParseRequestError::UnexpectedEof)?;
            String::from_utf8(buffer).map_err(|_| ParseRequestError::Utf8Error)?
        };

        let [method, path, _] = request_line.trim().split(' ').collect::<Vec<_>>()[..] else {
            return Err(ParseRequestError::InvalidRequestLine(request_line));
        };

        let method = match method {
            "GET" => Method::Get,
            "POST" => Method::Post,
            _ => return Err(ParseRequestError::InvalidMethod(method.to_string())),
        };
        let path = path.to_string();

        // HEADERS
        let headers: Vec<(String, String)> = {
            let mut headers = Vec::new();
            loop {
                let mut buffer = Vec::<u8>::new();
                reader
                    .read_until(b'\n', &mut buffer)
                    .map_err(|_| ParseRequestError::UnexpectedEof)?;
                let header = String::from_utf8(buffer).map_err(|_| ParseRequestError::Utf8Error)?;
                let header = header.trim().to_string();
                if header.is_empty() {
                    break;
                }
                match header.split_once(": ") {
                    Some((key, value)) => headers.push((key.to_string(), value.to_string())),
                    None => return Err(ParseRequestError::InvalidHeader(header.to_string())),
                }
            }
            headers
        };

        // BODY
        let body = match headers.iter().find_map(|(key, value)| {
            if key.to_lowercase() == "content-length" {
                Some(
                    value
                        .parse::<usize>()
                        .map_err(|_| ParseRequestError::InvalidHeader(format!("{key}: {value}"))),
                )
            } else {
                None
            }
        }) {
            Some(Ok(length)) => {
                let mut buffer = vec![0u8; length];
                reader
                    .read_exact(&mut buffer)
                    .map_err(|_| ParseRequestError::UnexpectedEof)?;
                Some(String::from_utf8(buffer).map_err(|_| ParseRequestError::Utf8Error)?)
            }
            Some(Err(error)) => return Err(error),
            None => None,
        };

        Ok(Request {
            method,
            path,
            headers,
            body,
        })
    }
}

/*
 * THREAD POOL
 */

mod thread_pool {
    use std::{
        sync::{
            mpsc::{self, RecvError},
            Arc, Mutex,
        },
        thread::{self, JoinHandle},
    };

    pub struct ThreadPool<Job: FnOnce() + Send + 'static> {
        workers: Vec<(usize, JoinHandle<()>)>,
        sender: Option<mpsc::Sender<Job>>,
    }

    impl<Job: FnOnce() + Send + 'static> ThreadPool<Job> {
        pub fn new(size: usize) -> ThreadPool<Job> {
            let (sender, receiver) = mpsc::channel::<Job>();
            let sender = Some(sender);
            let receiver = Arc::new(Mutex::new(receiver));
            let workers = (0..size)
                .map(|id| {
                    let receiver = Arc::clone(&receiver);
                    let handle = thread::spawn(move || loop {
                        let maybe_job = { receiver.lock().unwrap().recv() };
                        match maybe_job {
                            Ok(job) => job(),
                            Err(RecvError) => {
                                println!("info: stop worker {id}");
                                break;
                            }
                        };
                    });
                    println!("info: start worker {id}");
                    (id, handle)
                })
                .collect();
            ThreadPool { workers, sender }
        }

        pub fn execute(&self, job: Job) {
            if let Some(ref sender) = self.sender {
                sender.send(job).unwrap()
            }
        }
    }

    impl<Job: FnOnce() + Send + 'static> Drop for ThreadPool<Job> {
        fn drop(&mut self) {
            drop(self.sender.take());
            while let Some((_, handle)) = self.workers.pop() {
                handle.join().unwrap();
            }
        }
    }
}
