# actix-web-api
This is a simple REST API built with [Actix Web](https://actix.rs/) that supports CRUD operations with an in-memory data store.

## Prerequisites
- [Rust](https://www.rust-lang.org/) 
- [Cargo](https://doc.rust-lang.org/cargo/) 

## Running
Start the API server:
   ```
   cargo run
   ```

By default, the server will be running at `http://127.0.0.1:8080`.

## API Endpoints

### Get All Items
```
GET /items
```

#### Response:

```
[
  {
    "id": "item-id-1",
    "name": "Item Name",
    "quantity": 10
  },
  {
    "id": "item-id-2",
    "name": "Another Item",
    "quantity": 5
  }
]
```

### Get Single Item

```
GET /items/{id}
```

#### Response:

```
{
  "id": "item-id",
  "name": "Item Name",
  "quantity": 10
}
```

### Add New Item

```
POST /items
```

#### Request Body:

```
{
  "name": "New Item",
  "quantity": 12
}
```

#### Response:

```
{
  "id": "newly-generated-id",
  "name": "New Item",
  "quantity": 12
}
```

### Delete Item

```
DELETE /items/{id}
```

#### Response:

```
Status: 200 OK
```


## License

Licensed under the [MIT License](https://opensource.org/license/MIT).


