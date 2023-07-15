tuntap:
	ip tuntap add mode tun dev tun0 &&\
	ip link set tun0 up &&\
	ip addr add 10.0.0.1/24 dev tun0

run:
	cargo build
	sudo ./target/debug/tcp_ip_rust

capture:
	tcpdump -tn -i tun0

curl:
	curl --interface tun0 http://10.0.0.2/
