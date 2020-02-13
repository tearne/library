FROM adoptopenjdk:11-jre-hotspot-bionic

ENV DEBIAN_FRONTEND noninteractive

RUN apt-get update -y \
&&  apt-get -y dist-upgrade \
&&  apt-get install -y --no-install-recommends \
        vim \
        nano \
        htop \
        python3 \
        python3-pip \
&&  apt-get clean && rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/*

RUN  python3 -m pip install --upgrade --user pip setuptools \
&&   python3 -m pip --no-cache-dir install awscli boto3

WORKDIR /app
ADD app .

ENV PATH="/app/bin:${PATH}"
RUN chmod -R u+x /app/bin/*

CMD ["welcome.sh"]

