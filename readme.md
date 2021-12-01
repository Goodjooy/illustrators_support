# Illustrators Support 画师包养计划

post /user/login 
request
```json
{
    "name":Name,
    "qq":QQ,
}
```
post /user/new crate new user

```json
{
     "name":Name,
    "qq":QQ,
    "invit":InvitCode
}
```

respond:


post  /illustrator/new

```json
{
    "name":IllustratorName,
    "url":SupportPage,
}
```

```json
{
    "temp_ident": ""
}

```


post /illustrator/add_arts/{temp_ident}
file upload

post /illustrator/done

get /illustrator/all

post /illustrator/{ident}
