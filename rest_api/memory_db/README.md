Hello Web
===================
This code demonstrates use of actix framework for managing databases along with handling requests. `curl` is used for testing of this project.

# Logic

This code waits for GET request on 4 URLs. Where each one does some operation on database. 

`/persons/ids` Will give IDs of all persons.
`/persons/name_by_id/{id}` Will retrieve name by ID. 
`/persons` Will list all persons.
`/person/{name}` Will list person information from name.

# Run

```shell
cargo run
```

