# AGENTS.md — cwp-idp-rauthy (форк Rauthy)

> Роль, принципы и ограничения работы в `cwp-idp-rauthy` — форке Rauthy
> (`sebadob/rauthy`), используемом как Identity Provider платформы CleanAI Cloud.
> Применяется ко всей работе в этой директории.

## 1. Роль

- **Identity Provider (Rauthy, Rust) + расширение консентов (152-ФЗ).**
- Форк базируется на релизном теге `v0.36.2`; кастомная функциональность живёт
  изолированно (см. §4), общая кодовая база постоянно синхронизируется с апстримом.
- Документация Rauthy (книга): `https://sebadob.github.io/rauthy/`. Контекст
  платформы — через `cwp-docs` (профиль `infra-devops`), см. монорепо `cwp-docs`.

## 2. Веточная модель (жёстко)

- **`main` — зеркало апстрима `sebadob/rauthy`.** Никаких PR в `main`, никаких прямых
  коммитов. Обновляется ТОЛЬКО владельцем при синхронизации с апстримом (merge
  `upstream/main` в `main`).
- **Версионная (релизная) ветка форка** (`v{MAJOR}.{MINOR}`; текущая — см. `CWP_INFRA_REF`/
  деплой-манифест cwp-ci-cd, НЕ хардкодить). Вливание — только через pull-request из `t/<задача>`.
- **Все задачи — во временных ветках `t/<задача>`**, вливаются в целевую версионную ветку
  только через PR. Никаких прямых коммитов в `main`/целевую ветку; без force-push
  в общие ветки; теги — только по владельцу.
- Защита ветки `v0.2`: GitHub branch protection «require a pull request before merging»
  (без обязательного ревью) — включена.

## 3. Контракты / генерация

- Форк — IdP, контрактов `cwp-specs` не имеет. Wire-формат — OIDC/OAuth2 (стандарт).
- Сущности/миграции следуют конвенциям Rauthy (см. §4): dual-write hiqlite/postgres,
  `FromRow`+`FromPgRow`, миграции `migrations/{postgres,hiqlite}`.
- Генерация фронтенд-типов и wasm — штатными шагами Rauthy (`just build-wasm`,
  `svelte-kit sync`).

## 4. Изоляция кастома от апстрима (минимизация конфликтов синка)

**Вся наша логика — в изолированных точках; апстрим не знает о них:**

| Точка | Что здесь | Конфликт с апстримом |
| :--- | :--- | :--- |
| `src/rauthy-consents/` | Новый крэйт: сущности, API `/auth/v1/consents/*`, логика консентов | Нет (отсутствует в апстриме) |
| `migrations/postgres/V90__consents.sql`, `migrations/hiqlite/90_consents.sql` | Таблицы `consent_doc`, `user_consent` | Нет (номера выше верхушки апстрима) |
| `frontend/src/lib/ConsentGate.svelte`, `frontend/src/lib/ConsentCheckboxes.svelte`, `frontend/src/routes/admin/config/consents/` | UI согласий (вход/регистрация/админка) | Нет (новые файлы) |
| `frontend/src/api/types/consents.ts` | TS-типы консентов | Нет (новый файл) |

**Ограниченный набор правок общих файлов (единственная поверхность конфликта):**
`src/api/src/users.rs`, `src/api_types/src/users.rs`, `src/service/src/oidc/authorize.rs`,
`src/bin/src/server.rs`, `src/{api,service,bin}/Cargo.toml`,
`frontend/src/routes/users/register/+page.svelte`,
`frontend/src/routes/oidc/authorize/+page.svelte`,
`frontend/src/routes/admin/config/+layout.svelte`,
`frontend/src/i18n/common/*`, `Cargo.lock`.

**Правила правок общих файлов:**
- Только аддитивные и минимальные правки; никаких реформатов/переписываний.
- Новое поведение — новый файл/крэйт (OCP), не правка существующего.
- Не менять логику апстрима в общих файлах; хуки — по месту существующих вызовов
  (ToS-блок в регистрации, `need_tos_accept` в authorize).

## 5. Синхронизация с апстримом (владелец)

1. `git fetch upstream`
2. `main`: fast-forward до `upstream/main` (или релизного тега) — зеркало.
3. `v0.2`: merge/rebase на новый `main`. Конфликты возможны ТОЛЬКО в общих файлах из
   §4; `src/rauthy-consents/`, миграции `V90/90` и новые фронтенд-файлы не трогаются.
4. `cargo check` (перегенерация/разрешение `Cargo.lock`).
5. Верификация: `just extract-ui-archive`, `just build-wasm`, `cargo check`,
   `cd frontend && npm install && npm run check`.
6. `t/*` ветки ребейзятся на обновлённый `v0.2`.

## 6. Команды

- `cargo check` / `cargo build` — бэкенд (весь workspace, включая `rauthy-consents`).
- `cargo test` / `cargo clippy -- -D warnings` / `cargo fmt --check`.
- `just extract-ui-archive` — распаковка статичного UI-архива (нужно до `cargo build`).
- `just build-wasm` — сборка wasm-модулей (`spow`, `md`) — нужна для фронтенда.
- `cd frontend && npm install && npm run check` — svelte-check (типы/Svelte).
- `cd frontend && npm run format-check` — prettier.

## 7. Обозреваемость

- Rauthy: JSON-логи (`LOG_FMT=json`), события (`/auth/v1/events`, admin UI), метрики
  (`/metrics`). В OpenObserve платформы — как у прочих сервисов.
- Наш код логирует штатные операции на уровне info, ошибки — явно, без маскировок.

## 8. Обработка ошибок (наш код)

- Маскировки ЗАПРЕЩЕНЫ. Ошибка обрабатывается явно: штатное ветвление или
  fail-fast (crash/panic/exit), если ошибка невозможна. Все ошибки логируются.
- Пустой `catch`/`unwrap_or_default()` в обход ошибки недопустим.

## Git-правила (обязательно)

- ПУШИТЬ В `main` ЗАПРЕЩЕНО НАВСЕГДА. `main` — зеркало апстрима.
- Целевая (релизная) ветка — версионная (`v{MAJOR}.{MINOR}`; текущая — см. `CWP_INFRA_REF`/
  деплой-манифест cwp-ci-cd, НЕ хардкодить). Вливание — только через PR.
- Все задачи — во временных ветках `t/<задача>` от целевой версионной ветки.
- Никаких прямых коммитов в `main`/целевую ветку; без force-push; теги — по владельцу.
