FROM rust:latest

RUN apt-get update && apt-get install -y \
    build-essential \
    libssl-dev \
    zlib1g-dev \
    libyaml-dev \
    libffi-dev \
    ruby-full \
    && rm -rf /var/lib/apt/lists/*

RUN gem update && gem install brakeman

WORKDIR /workspace

CMD [ "bash" ]