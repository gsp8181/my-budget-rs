# my-budget-rs
Personal project for calculating how much money I have. Gives you an alloted daily allowance with an option to reserve a portion from monday-thursday for the weekend. Categories assets and liabilities with set transaction categories.


```yaml
services:
  my-bud-get-rs:
    container_name: budget-rs
    image: ghcr.io/gsp8181/my-budget-rs:latest
    ports:
      - 8000:8000/tcp
    restart: unless-stopped
    volumes:
      - <storage location>:/my-budget/storage
```
