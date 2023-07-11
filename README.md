# tutorial level
 Обучающий уровень для "Сайта с играми на программирование"

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