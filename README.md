# Sakata API

### Pre-Requisites
 - Rust
 - MySQL
 - AWS S3 bucket (image storage)

### Environment Variables
| Variable              | Example                                         |
| --------------------- | ----------------------------------------------- |
| DATABASE_URL          | mysql://admin:password@127.0.0.1:3306/sakata_db |
| IMAGE_BASEURL         | /tmp                                            |
| AWS_ACCESS_KEY_ID     | AKIATYJPK7VM5RGASERT                            |
| AWS_SECRET_ACCESS_KEY | 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        |
| AWS_S3_BUCKET_NAME    | sakata-cards                                    |
| AWS_REGION            | us-east-2                                       |

### Setup
 - Setting up Diesel
```
> cargo install diesel_cli
> export DATABASE_URL=mysql://admin:password@127.0.0.1:3306/sakata_db
> diesel setup
```
 - Running the Sakata API
```
> cargo run
```
