FROM ghcr.io/catthehacker/ubuntu:rust-22.04

RUN apt-get update

RUN apt-get install -y rsync