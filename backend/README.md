# API

## /user

### POST

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
    "created_at": "2024-10-29T22:47:31.209771",
    "updated_at": "2024-10-29T22:47:31.209771"
}
```

### GET

Get a user by username

**Request**

```json
{
    "username": "james"
}
```

**Response**

```json
{
    "id": 1,
    "username": "james",
    "email": "james@mail.com",
    "created_at": "2024-10-30T04:32:56.789418",
    "updated_at": "2024-10-30T04:32:56.789418"
}
```

## /user/score

### GET

**Request**

```json
{
    "username": "james"
}
```

**Response**

```json
{
    "user_id": 1,
    "total_wins": 0,
    "total_losses": 0,
    "points_earned": 0
}
```
