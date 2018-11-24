##

###database
Make database:

```sql
create role himporter;
create database hotel_import owner himporter;
```

create some migrations:

```sh
migrant new download-register
#update up/down files with migrations, then run...
migrant apply
# to rollback: migrant apply --down
```
