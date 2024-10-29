# API

## /user

### Post

Creates a user in the database and returns that user

**Request**

```json
{
    "username": "james",
    "email": "james@mail.com",
    "password_hash": "jamespass"
}
```

**Response**

```json
{
    "id": 5,
    "username": "james",
    "email": "james@mail.com",
    "password_hash": "jamespass",
    "created_at": "2024-10-29T22:47:31.209771",
    "updated_at": "2024-10-29T22:47:31.209771"
}
```
