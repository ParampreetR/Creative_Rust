Postgresql
===================
This code demonstrates how we can store data in Postgresql Server and retrieve them.

# Where is Server?

I hosted Redis Server through docker on my PC. It can be downloaded by

```shell
docker pull postgres
```

Then start with this command

```shell
docker run --name postgresql -e POSTGRES_PASSWORD=mypass123 -p 5432:5432 -d postgres
```

Now you can connect to `localhost:5432`. 

We are setting here environmental variable which should be password for postgresql server. This is given in documentation. 

# Run

```shell
cargo run
```

