# (r)audius discovery

This project is an implementation of the audius discovery provider written in rust.

## workspace division

### server

The layer that props up the API. Matches the swagger spec in the [official documentation](https://docs.audius.org/developers/rest-api).

### indexer

The layer that indexes operations from entity manager and persists them in the database used by the server.

### database

The persistence layer. Originally implemented in postgres but could branch into other database types.

### node

The binary that bundles these processes together.
