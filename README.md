# Tell Status
This application sends you an email, if a specific homepage returns a specific http status code.  

> [!NOTE]
> I wrote that application only for learning Rust. So don't rely on it.
> Feel free to fork it and do whatever you want with it.


## Configuration
You can (and should) configure the environment variables:  

```
AUTHOR_NAME=            "<Email Sender Name>"
AUTHOR_MAIL=            "<name@domain.tld>"
MAILTO=                 "<foo@bar.lol>"
SUBJECT=                "<Mail Subject>"
MESSAGE=                "<url> does not exist any more"
MAIL_SERVER=            "mail.foo.bar"
SMTP_PORT=              "587"
MAIL_USERNAME=          "<smtp user>"
MAIL_PASSWORD=          "<smtp password>"
TARGET_URL=             "<https://www.foo.bar>"
HOST=                   "<foo.bar>"
TARGET_STATUS_CODE=     "404"
SHOULD_SEND_MAIL=       "true"
USER_AGENT=             "Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:127.0) Gecko/20100101 Firefox/127.0"
POLLING_RATE_SEC=       "21600"
```

| Env Var               | Description                                                                          |
| --------------------- | ------------------------------------------------------------------------------------ |
| TARGET_URL            | Which URL should be monitored. (With https://)                                       |
| HOST                  | Required for a http request. Ex. you monitor `www.foo.bar`, so the host is `foo.bar` |
| TARGET_STATUS_CODE    | At which http status code it should send you a mail. Ex. 404                         |
| SHOULD_SEND_MAIL      | More a debug flag, if you just want to test something. true -> sends a mail          |
| POLLING_RATE_SEC      | How often it should check the http status, in seconds                                |


## Behaviour
If the target url returns the matching status code, it'll send you a mail. But only once.  
After that, it only resets it's state, if the target url returns any other status code.  

If you're hosting it in a container, a reload will reset the state. Otherwise you can delete 
the `state` file inside the working directory.  

## Usage
You can use it in a container or without. The only important thing is, that you set the above mentioned 
environment variables. Otherwise the application will terminate.  

Run it with: `./tellstatus`  

I recommend to explicitly change the working directory, where the application is stored. 
Otherwise it will create the `state` file, where you initialy invoked the terminal.  

## Build
This project uses some crates (tokio, reqwest and send_mail), so you have to build it 
with `cargo`.  

`cargo build -r`  

If you want to cross compile it, you'll need [Cross](https://github.com/cross-rs/cross) and Docker.  
You can configure `cross` further with the `Cross.toml` file if you want. But the necessary things 
are set.  

In the `makefile` are some presets. Like compiling and build for docker.  
Some examples:  

* `make` -> Building for AARCH64, creating and saving the Docker image
* `make fast` -> same like `make` but also deploying a Docker container
* `make build-rel-lnx` -> Building for AMD64 Linux
* `make docker-build-linux` -> Building a Docker image for AMD64 Linux
* `make clean` -> Deleting the Docker image



