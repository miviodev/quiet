Эта программа позволит вам скрыть ненужные логи, но при ошибке выведет ее:
```bash
quiet gut clone # вместо git clone
```
вывод: [ERROR] No such file or directory
# Сборка
для сборки нам понадобится cargo:
```bash
cargo build -r
```
бинарник будет лежать в ./target/release/
