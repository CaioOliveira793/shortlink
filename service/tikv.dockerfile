FROM docker.io/library/debian:latest as setup

RUN apt update -y && apt upgrade -y && apt install -y curl

RUN useradd tikv --create-home --home /home/tikv --user-group

USER tikv:tikv

WORKDIR /home/tikv

RUN curl --proto '=https' --tlsv1.2 -sSf \
	https://tiup-mirrors.pingcap.com/install.sh | sh

ENV PATH=/home/tikv/.tiup/bin:$PATH

RUN tiup install playground:v1.12.1 \
	pd:v7.0.0 \
	tikv:v7.0.0 \
	prometheus:v7.0.0 \
	grafana:v7.0.0

# Placement Driver client
EXPOSE 2379

# Prometheus
EXPOSE 9090

# Grafana
EXPOSE 3000

FROM setup as tikv-slim

CMD tiup playground --tag tikv_cluster \
	--mode tikv-slim \
	--host '0.0.0.0' \
	--pd 1 --kv 1
