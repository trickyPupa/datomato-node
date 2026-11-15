# Разработка

Linux: Rust 1.93.0, rustfmt, clippy, Git, Bash; для инструментов system/context — Python >=3.11.
Fedora: `sudo dnf install git gcc gcc-c++ make python3`; установите rustup из официального
источника, затем `rustup toolchain install 1.93.0 --profile minimal --component rustfmt,clippy`.
WSL2: работайте внутри Linux, например Ubuntu, установите git/build-essential/python3,
храните checkout в Linux-ФС (`~/src`), не в `/mnt/c`. Нативный Windows не проверен.
Запускайте `tools/check` из репозитория. CI вызывает тот же скрипт.
`cargo fmt --all` применяет форматирование; Cargo.lock коммитится в каждом workspace.
На машине с 16 ГБ ОЗУ запускайте workspace последовательно; по умолчанию два build jobs.
Побайтовая воспроизводимость бинарников не заявляется.

Контейнер: `tools/container-check` использует Dockerfile и build-image.txt этого репозитория.
Окружение одинаково по закреплённому базовому digest во всех Rust-репозиториях.
Нужны доступный Docker daemon и сеть для первого pull и rustfmt/clippy; после сборки
образа никаких внешних сервисов, RPC, секретов или MCP для проверок не требуется.
Системный компилятор/линкер в контейнере фиксируется digest. На хосте фиксирован только Rust.
Контейнерная проверка на текущей машине пока не выполнена: доступ к daemon запрещён.

Лицензия остаётся открытым решением B08. LICENSE намеренно не добавлен.

## Временное подключение contracts

Remote пока не создан; владелец GitHub — trickyPupa, имя/доступность нужно подтвердить.
Cargo.toml использует относительный path `../datomato-contracts/crates/protocol`.
Это временный режим разработки, НЕ доказательство самостоятельной сборки продукта.
Перед проверкой: `tools/prepare-contracts /путь/к/локальному/datomato-contracts`.
Скрипт получает чистую копию строго SHA из contracts-pin.toml рядом с продуктом;
существующий checkout не меняет, а проверяет SHA и отсутствие изменений.
Если продукт склонирован под произвольным именем, имя соседнего contracts всё равно datomato-contracts.
Не редактируйте исходники через этот checkout: изменяйте отдельную ветку contracts.

После создания доступного remote замените path на
`datomato-protocol = { git = "<подтверждённый URL>", rev = "<полный SHA>", version = "=0.1.0" }`,
обновите contracts-pin.toml и Cargo.lock; удалите обязательную подготовку соседа из CI.
Проверьте сборку только продукта в чистом checkout с пустым CARGO_HOME, затем system.
Нельзя записывать абсолютный file URL, плавающую ветку или выдуманный SHA.

## Контекст агента

`tools/get-context <Git URL или локальный repo system> <полный SHA system>` получает
чистую ревизию в игнорируемый `.context/system`. SHA назначает планировщик в задаче.
Читайте docs/spec/core-spec.md, docs/plans/technical-plan-v0.1.md,
docs/decisions/design-decisions-v0.1.md и docs/architecture.md этой копии.
Соседний checkout system не подразумевается и для сборки не нужен.

## Установленный toolchain на текущей машине

Здесь Rust 1.93.0 установлен под rustup-именем `stable`, а не `1.93.0`.
До установки именованного toolchain можно выполнить `RUSTUP_TOOLCHAIN=stable tools/check`:
скрипт дополнительно проверяет точную версию rustc 1.93.0 и отвергает другую.
Для сквозного прогона используется такое же явное окружение. Это локальный обход
отсутствия сети для rustup, а не замена закреплённого toolchain на плавающий stable в CI.
