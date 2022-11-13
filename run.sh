#!/bin/sh
CARGO_TARGET_DIR=/home/thorben/dev/learning/rust-tcp/target
cargo b --release
sudo setcap cap_net_admin=eip $CARGO_TARGET_DIR/release/rust-tcp

$CARGO_TARGET_DIR/release/rust-tcp &
pid=$!
sudo ip addr add 192.168.0.1/24 dev tun0
sudo ip link set up dev tun0
wait $pid