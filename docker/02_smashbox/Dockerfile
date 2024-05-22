# Build 
# docker build -t smashbox:latest .

# Run
# docker run --rm -d -p 1234:22 smashbox:latest
#
# docker run --rm -d -p someport:22 -e USERNAME=me -e PASSWORD=password -e AUTHORIZED_KEYS="$(cat path/authorized_keys_file)" smashbox:latest


FROM ubuntu:20.04

ENV DEBIAN_FRONTEND=noninteractive
ENV USERNAME=ubuntu
ENV PASSWORD=password

RUN apt-get update \
&&  apt-get install -y openssh-server iputils-ping telnet iproute2 sudo \
&&  apt-get clean \
&&  rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

RUN mkdir -p /run/sshd && chmod 755 /run/sshd

RUN useradd -ms /bin/bash $USERNAME
RUN echo '%sudo ALL=(ALL) NOPASSWD:ALL' >> /etc/sudoers

RUN mkdir -p /home/$USERNAME/.ssh \
&&  chown $USERNAME:$USERNAME /home/$USERNAME/.ssh \
&&  echo "PasswordAuthentication yes" >> /etc/ssh/sshd_config \
&&  echo "PermitRootLogin no" >> /etc/ssh/sshd_config

COPY init-docker.sh /usr/local/bin/
RUN chmod +x /usr/local/bin/init-docker.sh

EXPOSE 22
CMD ["/usr/local/bin/init-docker.sh"]