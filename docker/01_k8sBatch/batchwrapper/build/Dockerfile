FROM openjdk:11-jre

ENV DEBIAN_FRONTEND noninteractive

RUN apt-get update -y \
&&  apt-get -y upgrade \
&&  apt-get install -y --no-install-recommends \
        vim \
        nano \
        htop \
        python3 \
        python3-pip \
&&  apt-get clean && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

RUN  pip3 install --upgrade pip setuptools \
&&   pip3 --no-cache-dir install awscli boto3

WORKDIR /app
ADD app .

ENV PATH="/app/bin:${PATH}"
RUN chmod -R u+x /app/bin/*

CMD ["welcome.sh"]

