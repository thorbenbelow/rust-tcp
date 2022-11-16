## Running

```Bash
chmod +x ./run.sh
./run.sh
```

### Send Packages
```
nc 192.168.0.2 80
ping -I tun0 192.168.0.2
```
### Inspect traffic
```
tshark -i tun0
```

## References
- [Implementing TCP in Rust - Jon Gjengset](https://www.youtube.com/watch?v=bzja9fQWzdA&list=PLqbS7AVVErFivDY3iKAQk3_VAm8SXwt1X)
- [TUN/TAP doc](https://www.kernel.org/doc/Documentation/networking/tuntap.txt)
- [RFC 9293 - TCP complete](https://www.rfc-editor.org/rfc/rfc9293)
- [RFC 791 - IP](https://www.rfc-editor.org/rfc/rfc791)
- [RFC 793 - TCP core](https://www.rfc-editor.org/rfc/rfc793)
