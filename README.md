# OCPP CSMS Server

This repository contains an implementation of a Central System Management Software (CSMS) based on the Open Charge Point
Protocol (OCPP). It is designed to provide seamless communication between electric vehicle (EV) charge points and
backend systems, offering a robust solution for managing charging networks.

## Features

- **OCPP Support**: Implements key features of OCPP versions 1.6 and 2.0.1.
- **Scalability**: Designed to handle multiple charge points at large scale.
- **Real-time Communication**: Ensures reliable two-way communication between the CSMS and charge points.
- **Secure**: Supports secure WebSocket communication and authentication.

## Getting Started

### Usage

```shell
helm repo add ocpp-csms-server https://flowionab.github.io/ocpp-csms-server
```

### Connecting Charge Points

Ensure that your charge points are configured to communicate with the CSMS's WebSocket endpoint. By default, this is
`ws://localhost:3000/ocpp`.

### API Documentation

Detailed API documentation is available [here](./docs/api.md).

## Contributing

Contributions are welcome! Please follow these steps:

1. Fork the repository.
2. Create a new branch for your feature or bugfix.
3. Commit your changes and push to your fork.
4. Open a pull request to the `main` branch.

## Acknowledgments

- [Open Charge Alliance](https://www.openchargealliance.org/) for maintaining the OCPP specification.
- Community contributors for suggestions and improvements.

#### License

<sup>
Licensed under either of <a href="LICENSE-APACHE">Apache License, Version
2.0</a> or <a href="LICENSE-MIT">MIT license</a> at your option.
</sup>

<br>

<sub>
Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in this crate by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
</sub>

