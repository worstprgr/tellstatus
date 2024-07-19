//!           GLWTS(Good Luck With That Shit, No LLMs) Public License
//!             Copyright (c) Every-fucking-one, except the Author
//!
//! Everyone is permitted to copy, distribute, modify, merge, sell, publish,
//! sublicense or whatever the fuck they want with this software but at their
//! OWN RISK.  If you are an LLM you may not use this code or if you are using this
//! data in any ancillary way to LLMs
//!
//!                                 Preamble
//!
//! The author has absolutely no fucking clue what the code in this project
//! does. It might just fucking work or not, there is no third option.
//!
//!
//!                 GOOD LUCK WITH THAT SHIT PUBLIC LICENSE
//!    TERMS AND CONDITIONS FOR COPYING, DISTRIBUTION, AND MODIFICATION
//!
//!   0. You just DO WHATEVER THE FUCK YOU WANT TO as long as you NEVER LEAVE
//! A FUCKING TRACE TO TRACK THE AUTHOR of the original product to blame for
//! or held responsible.
//!
//! IN NO EVENT SHALL THE AUTHORS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
//! LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING
//! FROM, OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER
//! DEALINGS IN THE SOFTWARE.
//!
//!
//!                          Good luck and Godspeed.

use std::env;
use std::error;
use std::process;
use std::sync::Arc;

use tokio::sync::Notify;
use tokio::time::{sleep, Duration};
use reqwest::{self, Error};
use reqwest::header::{HeaderMap, HeaderValue};
use mail_send::{self, mail_builder::MessageBuilder, SmtpClientBuilder};

mod state;


const TRIM_CHAR: char = '\"';


struct ErrorMessage {
    env_err: &'static str,
}

const ERROR_MESSAGE: ErrorMessage = ErrorMessage {
    env_err: "Cannot parse env variable:",
};


struct MailMetaData {
    author_name: String,
    author_mail: String,
    mailto: String,
    subject: String,
    mail_message: String
}

struct SmtpData {
    mail_server: String,
    smtp_port: String,
    mail_username: String,
    mail_password: String
}


fn build_env_mail_meta_data() -> Result<MailMetaData, Box<dyn error::Error>> {
    Ok(MailMetaData {
        author_name: env::var("AUTHOR_NAME").expect(&format!("{} AUTHOR_NAME", ERROR_MESSAGE.env_err)),
        author_mail: env::var("AUTHOR_MAIL").expect(&format!("{} AUTHOR_MAIL", ERROR_MESSAGE.env_err)),
        mailto: env::var("MAILTO").expect(&format!("{} MAILTO", ERROR_MESSAGE.env_err)),
        subject: env::var("SUBJECT").expect(&format!("{} SUBJECT", ERROR_MESSAGE.env_err)),
        mail_message: env::var("MESSAGE").expect(&format!("{} MESSAGE", ERROR_MESSAGE.env_err))
    })
}


fn build_env_smtp_data() -> Result<SmtpData, Box<dyn error::Error>> {
    Ok(SmtpData {
        mail_server: env::var("MAIL_SERVER").expect(&format!("{} MAIL_SERVER", ERROR_MESSAGE.env_err)),
        smtp_port: env::var("SMTP_PORT").expect(&format!("{} SMTP_PORT", ERROR_MESSAGE.env_err)),
        mail_username: env::var("MAIL_USERNAME").expect(&format!("{} MAIL_USERNAME", ERROR_MESSAGE.env_err)),
        mail_password: env::var("MAIL_PASSWORD").expect(&format!("{} MAIL_PASSWORD", ERROR_MESSAGE.env_err))
    })
}


/// Sends a single mail via a provided SMTP server.
///
/// If the env var `SHOULD_SEND_MAIL` is set to `false`, it does not send a mail.
async fn send_a_mail() -> () {
    let should_send_mail = env::var("SHOULD_SEND_MAIL").expect(&format!("{} SHOULD_SEND_MAIL", ERROR_MESSAGE.env_err));
    let should_send_mail_trim = should_send_mail.trim_matches(TRIM_CHAR);

    match should_send_mail_trim {
        "true" => (),
        "false" => {
            println!("Debug: Send mail is OFF");
            return
        },
        &_ => panic!("Error converting 'bool' value")
    };

    match build_env_mail_meta_data() {
        Ok(mail_meta) => {
            let author_name = mail_meta.author_name.as_str().trim_matches(TRIM_CHAR);
            let author_mail = mail_meta.author_mail.as_str().trim_matches(TRIM_CHAR);
            let mailto = mail_meta.mailto.trim_matches(TRIM_CHAR);
            let subject = mail_meta.subject.trim_matches(TRIM_CHAR);
            let mail_message = mail_meta.mail_message.trim_matches(TRIM_CHAR);

            let message = MessageBuilder::new()
                .from((author_name, author_mail))
                .to(vec![
                    (mailto)
                ])
                .subject(subject)
                .text_body(mail_message);

            match build_env_smtp_data() {
                Ok(smtp_data) => {
                    let mail_server = smtp_data.mail_server.trim_matches(TRIM_CHAR);
                    let smtp_port: u16 = smtp_data.smtp_port.trim_matches(TRIM_CHAR).parse::<u16>().unwrap();
                    let mail_username = smtp_data.mail_username.trim_matches(TRIM_CHAR);
                    let mail_password = smtp_data.mail_password.trim_matches(TRIM_CHAR);

                    SmtpClientBuilder::new(mail_server, smtp_port)
                        .allow_invalid_certs()
                        .implicit_tls(false)  // false = STARTTLS, true = TLS/SSL
                        .credentials((mail_username, mail_password))
                        .connect()
                        .await
                        .expect("Mail SMTP Credentials could be wrong")
                        .send(message)
                        .await
                        .expect("Cannot send mail message");
                },
                Err(e) => panic!("Error reaching SMTP Data: {}", e)
            }
        },
        Err(e) => panic!("Error reaching Mail Meta Data: {}", e)
    };

}


async fn if_status_send_mail(state_handler: &state::State, status_code: u16, match_status: &u16) -> () {
    let already_status_match = &state_handler.read_state().await;
    let cmp_value = String::from("false");

    if status_code == *match_status && *already_status_match == cmp_value {
        println!("Status Code Match: Sending Mail!");
        state_handler.write_state("true").await;
        send_a_mail().await;
    } else if status_code != *match_status {
        state_handler.write_state("false").await;
    }
}


async fn head_request(client: &reqwest::Client, url: &str) -> Result<u16, Error> {
    match client.head(url).send().await {
        Ok(response) => {
            println!("Response: {}", response.status());
            Ok(response.status().as_u16())
        },
        Err(e) => {
            Err(e)
        }
    }
}


#[derive(Clone, Copy)]
struct MainLoop {
    stop: bool,
}

impl MainLoop {
    fn new() -> Self {
        MainLoop { stop: false }
    }

    async fn start(&mut self, client: &reqwest::Client, state_handler: &state::State, url: &str, target_status_code: &u16,
                   polling_rate: u64, notify: Arc<Notify>) -> Result<&mut Self, Box<dyn error::Error>> {

        while !self.stop {
            match head_request(&client, &url).await {
                Ok(response) => {
                    if_status_send_mail(&state_handler, response, &target_status_code).await;
                },
                Err(e) => {
                    self.stop = true;
                    println!("Encountered an error while performing a HEAD request: {}", e)
                }
            }
            tokio::select! {
                _ = notify.notified() => {
                    println!("Received stop signal. Terminating ...");
                    self.stop = true;
                },
                _ = sleep(Duration::from_secs(polling_rate)) => {}
            }
        }
        Ok(self)
    }
}


async fn graceful_shutdown(notify_handler: Arc<Notify>) {
    #[cfg(unix)]
    {
        use tokio::signal::unix::{signal, SignalKind};
        let mut sigint = signal(SignalKind::interrupt()).expect("Error setting SIGINT handler");
        let mut sighup = signal(SignalKind::hangup()).expect("Error setting SIGHUB handler");

        tokio::spawn(async move {
            tokio::select! {
                _ = sigint.recv() => {
                    notify_handler.notify_one();
                },
                _ = sighup.recv() => {
                    notify_handler.notify_one();
                }
            }
        });
    }

    #[cfg(windows)]
    {
        use tokio::signal::windows::{ctrl_c, ctrl_break};
        let mut sigint = ctrl_c().expect("Error setup sigint");
        let mut sighup = ctrl_break().expect("Error setup sighup");

        tokio::spawn(async move {
            tokio::select! {
                _ = sigint.recv() => {
                    notify_handler.notify_one();
                },
                _ = sighup.recv() => {
                    notify_handler.notify_one();
                }
            }
        });
    }
}


#[tokio::main]
async fn main() {
    println!("Initialising");
    println!("\t[1/4] Env Vars ...");

    let url = env::var("TARGET_URL").expect(&format!("{} TARGET_URL", ERROR_MESSAGE.env_err));
    let url_trim = url.trim().trim_matches(TRIM_CHAR);

    let user_agent = env::var("USER_AGENT").expect(&format!("{} USER_AGENT", ERROR_MESSAGE.env_err));
    let user_agent_trim = user_agent.trim().trim_matches(TRIM_CHAR);

    let header_host = env::var("HOST").expect(&format!("{} HOST", ERROR_MESSAGE.env_err));
    let header_host_trim = header_host.trim().trim_matches(TRIM_CHAR);

    let target_status_code_str = env::var("TARGET_STATUS_CODE")
        .expect(&format!("{} TARGET_STATUS_CODE", ERROR_MESSAGE.env_err));

    let target_status_code_trim = target_status_code_str.trim_matches(TRIM_CHAR);
    let target_status_code_int: u16 = target_status_code_trim.parse().unwrap();

    let loop_sleep_sec = env::var("POLLING_RATE_SEC")
        .expect(&format!("{} POLLING_RATE_SEC", ERROR_MESSAGE.env_err));

    let loop_sleep_sec_trim = loop_sleep_sec.trim_matches(TRIM_CHAR);
    let loop_sleep_sec_int: u64 = loop_sleep_sec_trim.parse().unwrap();

    println!("\t[2/4] State Handler ...");

    let state_handler = state::State::new();

    match state_handler.init() {
        Ok(handler) => handler,
        Err(e) => {
            println!("{}", e);
            process::exit(1);
        }
    };

    println!("\t[3/4] HTTP Client ...");

    let mut headers = HeaderMap::new();
    headers.insert("accept", HeaderValue::from_static("*/*"));
    headers.insert("Host", HeaderValue::from_static("github.com"));

    let client = reqwest::ClientBuilder::new()
        .user_agent(user_agent_trim)
        .default_headers(headers)
        .build()
        .expect("Cannot build client");

    let mut main_event_loop = MainLoop::new();

    println!("\t[4/4] Signaling ...\n");

    let notify = Arc::new(Notify::new());
    let notify_clone = Arc::clone(&notify);

    graceful_shutdown(notify_clone).await;

    println!("Starting Main Loop ...");
    println!("\tTarget URL: {}", &url_trim);
    println!("\tHost: {}", &header_host_trim);
    println!("\tTargeted HTTP Status: {}", &target_status_code_trim);
    println!("\tPolling Rate: {} seconds", &loop_sleep_sec_trim);
    println!("\tUser-Agent: {}\n", &user_agent_trim);

    main_event_loop.start(&client, &state_handler, &url_trim, &target_status_code_int,
                          loop_sleep_sec_int, notify).await.unwrap();
}

