Hello Web
===================
This code demonstrates use of actix framework for downloading, uploading, deleting files from server. `curl` is used for testing of this project.

# Logic

This code waits for GET, POST, DELETE or PUT requests on `localhost:8080/{filename}` where filename is a name of the file where operations will be performed. 

`GET` Request will download file.

`POST` and `PUT` will upload file.

`DELETE` will delete file.

# Run

```shell
cargo run
```

