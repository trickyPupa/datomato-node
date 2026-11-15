# Происхождение фиксаций

Actions проверены по официальным release/commit страницам:
- https://github.com/actions/checkout/commit/11bd71901bbe5b1630ceea73d27597364c9af683 (v4.2.2)
- https://github.com/actions/upload-artifact/commit/ea165f8d65b6e75b540449e92b4886f43607fa02 (v4.6.2)

Базовый образ: официальный library/rust, manifest digest для 1.93.0:
https://hub.docker.com/layers/library/rust/1.93.0/images/sha256-484f5078e589013ba67ef76a861ef04de926a5df7cf66282999b5abc8e836ac9
Digest manifest указан на странице; digest image/config из URL — другое значение.
Pull/build пока не проверены из-за отсутствия DNS и доступа к Docker daemon.
Обновление фиксаций требует проверки upstream, локального check и чистого прогона system.
