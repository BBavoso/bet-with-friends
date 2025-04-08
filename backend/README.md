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

## /user/bets

### GET

**Request**

```json
{
    "username": "bob"
}
```

**Response**

```json
[
    {
        "id": 1,
        "creator_id": 2,
        "description": "test bet 1",
        "status": "Active",
        "stop_bets_at": null,
        "created_at": "2025-04-08T21:47:39.659087",
        "updated_at": "2025-04-08T21:47:39.659087",
        "paid_out": false,
        "paid_out_at": null
    },
    {
        "id": 2,
        "creator_id": 2,
        "description": "test bet 1",
        "status": "Active",
        "stop_bets_at": "2025-05-10T21:47:39.659087",
        "created_at": "2025-04-08T21:55:57.692273",
        "updated_at": "2025-04-08T21:55:57.692273",
        "paid_out": false,
        "paid_out_at": null
    }
]
```

## /bet

May be used with and with out cuttoff datetime

Without cuttoff:

### POST

**Request**

```json
{
    "username": "bob",
    "description": "test bet 1"
}
```

**Response**

```json
{
    "id": 1,
    "creator_id": 2,
    "description": "test bet 1",
    "status": "Active",
    "stop_bets_at": null,
    "created_at": "2025-04-08T21:47:39.659087",
    "updated_at": "2025-04-08T21:47:39.659087",
    "paid_out": false,
    "paid_out_at": null
}
```

With cuttoff:

**Request**

```json
{
    "username": "bob",
    "description": "test bet 1",
    "stop_bets_at": "2025-05-10T21:47:39.659087"
}
```

**Response**

```json
{
    "id": 2,
    "creator_id": 2,
    "description": "test bet 1",
    "status": "Active",
    "stop_bets_at": "2025-05-10T21:47:39.659087",
    "created_at": "2025-04-08T21:55:57.692273",
    "updated_at": "2025-04-08T21:55:57.692273",
    "paid_out": false,
    "paid_out_at": null
}
```
