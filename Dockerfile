FROM debian:bullseye-slim

WORKDIR /app

# COPY target/x86_64-unknown-linux-gnu/release/tellstatus /app
# COPY target/x86_64-unknown-linux-gnu/debug/tellstatus /app

COPY target/aarch64-unknown-linux-gnu/release/tellstatus /app
# COPY target/aarch64-unknown-linux-gnu/debug/tellstatus /app

RUN chmod +x /app/tellstatus

RUN apt-get update && apt-get install -y ca-certificates

ENV AUTHOR_NAME="<Email Sender Name>"
ENV AUTHOR_MAIL="<name@domain.tld>"
ENV MAILTO="<foo@bar.lol>"
ENV SUBJECT="<Mail Subject>"
ENV MESSAGE="<url> does not exist any more"
ENV MAIL_SERVER="mail.foo.bar"
ENV SMTP_PORT="587"
ENV MAIL_USERNAME="<smtp user>"
ENV MAIL_PASSWORD="<smtp password>"
ENV TARGET_URL="<https://www.foo.bar>"
ENV HOST="<foo.bar>"
ENV TARGET_STATUS_CODE="404"
ENV SHOULD_SEND_MAIL="true"
ENV USER_AGENT="Mozilla/5.0 (Windows NT 10.0; Win64; x64; rv:127.0) Gecko/20100101 Firefox/127.0"
ENV POLLING_RATE_SEC="21600"

CMD ["/app/tellstatus"]
