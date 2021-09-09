Redis
===================
This code demonstrates how we can store key value pairs in Redis Server and retrieve them.

# Where is Server?

I hosted Redis Server through docker on my PC. It can be downloaded by

```
docker pull redis
```

Then start with this command

```
docker run --name redis -p 6379:6379 -dt redis
```

Now you can connect to `localhost:6379`. 

## Warning

The approach followed by me is risky. We are exposing redis server to internet, better put some firewall rule on port 6379. I'm not running on any production server but my personal machine. So I'm Ok with this.

# Run

```shell
cargo run
```

