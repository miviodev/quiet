# <img src="https://gitea.com/miviodev/quiet/raw/branch/master/quiet_logo.png  " width="32"> quiet

![English README.md](./README.md)

[![crates.io](https://img.shields.io/badge/crates-io-yellow.svg)](https://crates.io/crates/quiet-cli)
[![License: MIT](https://img.shields.io/badge/License-MIT-blue.svg)](https://opensource.org/licenses/MIT)
[![Gitea](https://img.shields.io/badge/gitea-logo)](https://gitea.com/miviodev/quiet)
[![GitHub](https://img.shields.io/badge/github-logo)](https://github.com/miviodev/quiet)


Эта программа позволит вам скрыть ненужные логи, но при ошибке выведет ошибки:
```bash
quiet gut clone # вместо git clone
```
вывод: 
```
[ERROR]: No such file or directory (os error 2)
```
Так-же в последней версии я сделал обработку ошибок не только в shell (например не найдена команда), а еще и в командах:
```
quiet git lig # вместо git log
```
вывод:
```
[ERROR]: git: «lig» не является командой git. Смотрите «git --help».
Самые похожие команды:
	log
```
# Установка
для сборки нам понадобится `cargo`:
```bash
cargo install quiet-cli
```
если у вас `Команда «quiet» не найдена.`, то вам надо добавить ~/.cargo/bin в path:
```bash
echo "export PATH=$HOME/.cargo/bin:$PATH" >> ~/.bashrc # или ~/.zshrc смотря какой shell вы используете
```
# Сборка
для сборки нам понадобится `cargo`:
```bash
cargo build -r
```
бинарник будет лежать в `./target/release/`
