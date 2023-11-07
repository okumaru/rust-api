
![Build Status: Ubuntu](https://github.com/okumaru/rust-api/actions/workflows/rust.yml/badge.svg)


# Rust CRUD REST API

A RESTful API example for simple financial application with Rust Programming Language.


## Develop on

**Windows:** 10 (64-bit)  
**Rustc:** 1.73.0 (cc66ad468 2023-10-03)  
**Cargo:** 1.73.0 (9c4383fb5 2023-08-26)


## Environment Variables

To run this project, you will need to add the following environment variables to your .env file

`DATABASE_URL` : Connection to database (_mysql/mariadb_)

## Run Locally

Clone the project

```bash
  git clone git@github.com:okumaru/rust-api.git
```

Go to the project directory

```bash
  cd rust-api
```

Execute run locally

```bash
  cargo run
```


## API Reference

### Get all accounts

```http
  GET /accounts
```

### Get one account

```http
  GET /accounts?id=${id}
```

**Request Query**
| Query | Type     | Description                          |
| :---- | :------- | :----------------------------------- |
| `id`  | `string` | **Required**. Id of account to fetch |

### Add account

```http
  PUT /accounts
```

**Body JSON**
| Params        | Type      | Description                       |
| :------------ | :-------- | :-------------------------------- |
| `name`        | `string`  | **Required**. Account name        |
| `description` | `string`  | **Optional**. Account description |
| `balance`     | `integer` | **Required**. Account balance     |

### Update account

```http
  POST /accounts?id=${id}
```

**Request Query**
| Query | Type     | Description                           |
| :---- | :------- | :------------------------------------ |
| `id`  | `string` | **Required**. Id of account to update |

**Body JSON**

| Params        | Type      | Description                       |
| :------------ | :-------- | :-------------------------------- |
| `name`        | `string`  | **Optional**. Account name        |
| `description` | `string`  | **Optional**. Account description |
| `balance`     | `integer` | **Optional**. Account balance     |


### Delete account

```http
  DELETE /accounts?id=${id}
```

**Request Query**
| Query | Type     | Description                           |
| :---- | :------- | :------------------------------------ |
| `id`  | `string` | **Required**. Id of account to delete |

---

### Get all transaction category

```http
  GET /trx_cats
```

### Get one transaction category

```http
  GET /trx_cats?id=${id}
```

**Request Query**
| Query | Type     | Description                                       |
| :---- | :------- | :------------------------------------------------ |
| `id`  | `string` | **Required**. Id of transaction category to fetch |

### Add transaction category

```http
  PUT /trx_cats
```

**Body JSON**
| Params        | Type     | Description                                    |
| :------------ | :------- | :--------------------------------------------- |
| `name`        | `string` | **Required**. transaction category name        |
| `description` | `string` | **Optional**. transaction category description |

### Update transaction category

```http
  POST /trx_cats?id=${id}
```

**Request Query**
| Query | Type     | Description                                        |
| :---- | :------- | :------------------------------------------------- |
| `id`  | `string` | **Required**. Id of transaction category to update |

**Body JSON**

| Params        | Type     | Description                                    |
| :------------ | :------- | :--------------------------------------------- |
| `name`        | `string` | **Optional**. transaction category name        |
| `description` | `string` | **Optional**. transaction category description |


### Delete transaction category

```http
  DELETE /trx_cats?id=${id}
```

**Request Query**
| Query | Type     | Description                                        |
| :---- | :------- | :------------------------------------------------- |
| `id`  | `string` | **Required**. Id of transaction category to delete |

---

### Get all transaction

```http
  GET /trxs
```

### Get one transaction

```http
  GET /trxs?id=${id}
```

**Request Query**
| Query | Type     | Description                              |
| :---- | :------- | :--------------------------------------- |
| `id`  | `string` | **Required**. Id of transaction to fetch |

### Add transaction

```http
  PUT /trxs
```

**Body JSON**
| Params        | Type      | Description                                  |
| :------------ | :-------- | :------------------------------------------- |
| `credit`      | `integer` | **Required**. transaction _in_ to account    |
| `debit`       | `integer` | **Required**. transaction _out_ from account |
| `description` | `string`  | **Optional**. transaction description        |
| `accountid`   | `integer` | **Required**. transaction out from account   |
| `categoryid`  | `integer` | **Required**. transaction out from account   |

### Update transaction

```http
  POST /trxs?id=${id}
```

**Request Query**
| Query | Type     | Description                               |
| :---- | :------- | :---------------------------------------- |
| `id`  | `string` | **Required**. Id of transaction to update |

**Body JSON**

| Params        | Type      | Description                                  |
| :------------ | :-------- | :------------------------------------------- |
| `credit`      | `integer` | **Optional**. transaction _in_ to account    |
| `debit`       | `integer` | **Optional**. transaction _out_ from account |
| `description` | `string`  | **Optional**. transaction description        |
| `categoryid`  | `integer` | **Optional**. transaction out from account   |


### Delete transaction

```http
  DELETE /trxs?id=${id}
```

**Request Query**
| Query | Type     | Description                               |
| :---- | :------- | :---------------------------------------- |
| `id`  | `string` | **Required**. Id of transaction to delete |


## License

See the [LICENSE](LICENSE.md) file for license rights and limitations (MIT).