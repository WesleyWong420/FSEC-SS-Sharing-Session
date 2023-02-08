FROM ubuntu:22.04

RUN apt-get update \
  && apt-get install knockd -y && apt install iptables iptables-persistent -y && apt install net-tools -y && apt-get install iproute2 -y \
  && apt-get install -y \
  xinetd \
  && rm -rf /var/lib/apt/lists/*

RUN mkdir -p /challenge
RUN useradd -M -d /challenge ctf
WORKDIR /challenge

RUN mkdir -p /temp

COPY ctf.xinetd /etc/xinetd.d/ctf
COPY . /challenge/
COPY knockknock /challenge/temp/
COPY decoy /challenge/temp/

COPY knockd.conf /etc/knockd.conf
COPY knockd /etc/default/knockd

RUN chown -R ctf:ctf /challenge && chmod -R 770 /challenge
RUN chown -R root:ctf /challenge && \
  chmod -R 750 /challenge

CMD ["/usr/sbin/xinetd", "-dontfork"]

RUN cp /challenge/temp/decoy /challenge/knockknock

WORKDIR /

EXPOSE 1337