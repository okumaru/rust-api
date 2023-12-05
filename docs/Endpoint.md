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
| `star`        | `boolean` | **Required**. Account favorite    |
| `type`        | `string`  | **Required**. Account type        |
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
| `star`        | `boolean` | **Optional**. Account favorite    |
| `type`        | `string`  | **Optional**. Account type        |
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

### Get all type category

```http
  GET /cat_types
```

### Get one type category

```http
  GET /cat_types?id=${id}
```

**Request Query**
| Query | Type     | Description                                |
| :---- | :------- | :----------------------------------------- |
| `id`  | `string` | **Required**. Id type of category to fetch |

### Add type category

```http
  PUT /cat_types
```

**Body JSON**
| Params        | Type     | Description                                |
| :------------ | :------- | :----------------------------------------- |
| `type`        | `string` | **Required**. type category                |
| `description` | `string` | **Optional**. description of type category |
| `icon`        | `string` | **Required**. icon of type category        |

### Update type category

```http
  POST /cat_types?id=${id}
```

**Request Query**
| Query | Type     | Description                                 |
| :---- | :------- | :------------------------------------------ |
| `id`  | `string` | **Required**. Id type of category to update |

**Body JSON**

| Params        | Type     | Description                                |
| :------------ | :------- | :----------------------------------------- |
| `type`        | `string` | **Optional**. type category                |
| `description` | `string` | **Optional**. description of type category |
| `icon`        | `string` | **Optional**. icon of type category        |

### Delete type category

```http
  DELETE /cat_types?id=${id}
```

**Request Query**
| Query | Type     | Description                                 |
| :---- | :------- | :------------------------------------------ |
| `id`  | `string` | **Required**. Id type of category to delete |

---

### Get all category

```http
  GET /trx_cats
```

### Get one category

```http
  GET /trx_cats?id=${id}
```

**Request Query**
| Query | Type     | Description                           |
| :---- | :------- | :------------------------------------ |
| `id`  | `string` | **Required**. Id of category to fetch |

### Add category

```http
  PUT /trx_cats
```

**Body JSON**
| Params        | Type               | Description                                |
| :------------ | :----------------- | :----------------------------------------- |
| `name`        | `string`           | **Required**. category name                |
| `description` | `string`           | **Optional**. category description         |
| `typeid`      | `integer`          | **Required**. id type of category          |
| `budget`      | `(Obj) Add Budget` | **Optional**. data add budget for category |

### Update category

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


### Delete category

```http
  DELETE /trx_cats?id=${id}
```

**Request Query**
| Query | Type     | Description                                        |
| :---- | :------- | :------------------------------------------------- |
| `id`  | `string` | **Required**. Id of transaction category to delete |

---

### Get all budget for category

```http
  GET /trx_cats_budgets?categoryid=${id}
```

**Request Query**
| Query | Type     | Description                  |
| :---- | :------- | :--------------------------- |
| `id`  | `string` | **Required**. Id of category |


### Get one budget for category

```http
  GET /trx_cats_budgets?id=${id}
```

**Request Query**
| Query | Type     | Description                         |
| :---- | :------- | :---------------------------------- |
| `id`  | `string` | **Required**. Id of budget to fetch |

### Add budget for category

```http
  PUT /trx_cats_budgets
```

**Body JSON**
| Params       | Type      | Description                    |
| :----------- | :-------- | :----------------------------- |
| `periode`    | `string`  | **Required**. Periode budget   |
| `allocated`  | `integer` | **Required**. Budget allocated |
| `spent`      | `integer` | **Required**. Budget spent     |
| `available`  | `integer` | **Required**. Budget available |
| `categoryid` | `integer` | **Required**. Id of category   |

### Update budget for category

```http
  POST /trx_cats_budgets?id=${id}
```

**Request Query**
| Query | Type     | Description                          |
| :---- | :------- | :----------------------------------- |
| `id`  | `string` | **Required**. Id of budget to update |

**Body JSON**

| Params       | Type      | Description                    |
| :----------- | :-------- | :----------------------------- |
| `periode`    | `string`  | **Optional**. Periode budget   |
| `allocated`  | `integer` | **Optional**. Budget allocated |
| `spent`      | `integer` | **Optional**. Budget spent     |
| `available`  | `integer` | **Optional**. Budget available |
| `categoryid` | `integer` | **Optional**. Id of category   |

### Delete budget for category

```http
  DELETE /trx_cats_budgets?id=${id}
```

**Request Query**
| Query | Type     | Description                          |
| :---- | :------- | :----------------------------------- |
| `id`  | `string` | **Required**. Id of budget to delete |


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
| Params        | Type      | Description                                                         |
| :------------ | :-------- | :------------------------------------------------------------------ |
| `credit`      | `integer` | **Required**. transaction _in_ to account                           |
| `debit`       | `integer` | **Required**. transaction _out_ from account                        |
| `description` | `string`  | **Optional**. transaction description                               |
| `datetime`    | `string`  | **Required**. transaction description. format (yyyy-MM-dd HH:mm:ss) |
| `accountid`   | `integer` | **Required**. transaction out from account                          |
| `categoryid`  | `integer` | **Required**. transaction out from account                          |

### Update transaction

```http
  POST /trxs?id=${id}
```

**Request Query**
| Query | Type     | Description                               |
| :---- | :------- | :---------------------------------------- |
| `id`  | `string` | **Required**. Id of transaction to update |

**Body JSON**

| Params        | Type      | Description                                                         |
| :------------ | :-------- | :------------------------------------------------------------------ |
| `credit`      | `integer` | **Optional**. transaction _in_ to account                           |
| `debit`       | `integer` | **Optional**. transaction _out_ from account                        |
| `description` | `string`  | **Optional**. transaction description                               |
| `datetime`    | `string`  | **Optional**. transaction description. format (yyyy-MM-dd HH:mm:ss) |


### Delete transaction

```http
  DELETE /trxs?id=${id}
```

**Request Query**
| Query | Type     | Description                               |
| :---- | :------- | :---------------------------------------- |
| `id`  | `string` | **Required**. Id of transaction to delete |
