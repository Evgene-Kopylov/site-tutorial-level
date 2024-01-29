# tutorial level
 Обучающий уровень для "Сайта с играми на программирование"

# lvl_0
Уровень с шариком и несколькими врагами. Враги движутся к игроку.

https://evgene-kopylov.github.io/tutorial-level/lvl_0/

# Разработка, настройка среды.

Отслеживать изменения и при сохранении запускать тесты.
```console
cargo install cargo-watch
```
=> перейти в папку приложения
```console
cargo watch -x run
```

Сделать WASM
```console
cargo build --target wasm32-unknown-unknown; cp target/wasm32-unknown-unknown/debug/*.wasm
```