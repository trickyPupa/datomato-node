# Происхождение фиксаций

Actions проверены по официальным release/commit страницам:
- https://github.com/actions/checkout/commit/11bd71901bbe5b1630ceea73d27597364c9af683 (v4.2.2)
- https://github.com/actions/upload-artifact/commit/ea165f8d65b6e75b540449e92b4886f43607fa02 (v4.6.2)

Toolchain: Rust 1.100.0 и поставляемый вместе с ним Cargo 1.100.0.
Последний стабильный выпуск проверен 15 ноября 2026:
https://blog.rust-lang.org/2026/11/12/Rust-1.100.0/

Базовый образ: официальный library/rust 1.100.0, linux/amd64.
Manifest digest: sha256:ebc54bef33ec8f9a5cd05799366010f35efbb73b354fb4f2cb086ec6b9460ef5
Источник: https://hub.docker.com/layers/library/rust/1.100.0/images/sha256-ebc54bef33ec8f9a5cd05799366010f35efbb73b354fb4f2cb086ec6b9460ef5?tab=layers
Используется manifest платформы linux/amd64, а не плавающий тег или index другого образа.
Pull/build пока не проверены: нет DNS и доступа к Docker daemon.
Обновление фиксаций требует проверки upstream и нового чистого прогона system.
