#!/bin/bash

: ${USERNAME:=ubuntu}
: ${PASSWORD:=changeme}

useradd -ms /bin/bash $USERNAME
echo "$USERNAME:$PASSWORD" | chpasswd
adduser $USERNAME sudo

if [ -n "$AUTHORIZED_KEYS" ]; then
    mkdir -p /home/$USERNAME/.ssh
    echo "$AUTHORIZED_KEYS" > /home/$USERNAME/.ssh/authorized_keys
    chown -R $USERNAME:$USERNAME /home/$USERNAME/.ssh
fi

/usr/sbin/sshd -D