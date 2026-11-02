# Как проверить ChatHeads в игре

Сборку и запуск делает родитель (лаунчер). Здесь — что уже проверено, что
нужно сделать и что должно быть видно.

## Уже проверено (на этой машине)

```
horizon-mixins plan  --workspace <source> --mods E:/Projects/hz-mods/chatheads/mixins --out E:/Projects/hz-mods/chatheads/build/plan.json
{"bridges":0,"injections":6,"modules":3,"status":"ok"}

horizon-mixins apply --workspace <source> --mods E:/Projects/hz-mods/chatheads/mixins --out E:/Projects/hz-mods/chatheads/build/stage2
{"injections":6,"modules":3,"stage_changed":true,"status":"ok"}
```

где `<source>` = `%APPDATA%/horizon-launcher/native/source/4aa6854a886183440256f4f4a4a9d1ed853227a1`.

- unit-тесты чистой логики: `cargo test` в `E:/Projects/hz-mods/chatheads` — 6/6
  зелёные;
- `rustfmt --check` по всем `mixins/chatheads/*.rs` — чисто; `cargo clippy
  --all-targets -- -D warnings` по standalone-крейту (`runtime.rs`) — чисто.
  `glue.rs`, `shift.rs` и хелперы вне игры не компилируются (нужны типы игры),
  поэтому clippy по ним не гонялся.

На Windows `apply` в **существующую** папку падает с `E_ATOMIC_STAGE` — нужен
свежий `--out` (поэтому `stage2`).

## Что сделать

1. Сборка мода (решает человек, не агент):
   `horizon-launcher native-build E:/Projects/hz-mods/chatheads --instance <имя> --trust-native-code`
2. Запустить инстанс: `horizon-launcher play --instance <имя>` (или через окно).
3. Зайти в мир/на сервер.

## Что должно быть видно

1. **Options → Mods** → строка `ChatHeads 0.1.1` с описанием; на странице мода
   одна кнопка `Chat heads: ON`.
2. В чате (клавиша `T`) — **голова слева от каждой строки игрока**: 8×8,
   вплотную к тексту с отступом 2 px, текст строки сдвинут вправо, левый край
   строк ровный. Голова того же цвета/скина, что скин игрока (на сервере — его
   настоящий скин, в одиночной игре — стандартная голова).
3. Системные строки (`[Server] ...`, сообщения команд, «Saved screenshot as …»)
   — **без головы**, но тоже сдвинуты вправо.
4. Строки гаснут по времени — голова гаснет вместе со строкой (фон чата
   ползёт, головы едут вместе с ним при прокрутке `Page Up`).
5. Открытый чат (Enter/T) — головы видны и там, включая прокрутку истории.
6. Настройка: клик `Chat heads: OFF` → в следующем кадре головы пропадают и
   текст возвращается на x = 0 (никаких пустых отступов); `ON` — обратно.
   Значение переживает перезапуск игры (файл `heads` в
   `horizon-mods/.horizon-mod-data/chatheads/`).
7. F1 (скрытый HUD) — чата нет, голов тоже.
8. Масштаб чата (Options → Chat → Scale: 1.0/1.5/2.0) — голова масштабируется
   вместе со строкой и остаётся выровненной по её верхнему краю.

## Что НЕ проверено

- Компиляция вставленного кода в крейтах игры и запуск: здесь запрещено
  собирать игру. `glue.rs` (типы `Game`, `HudBatch`, `HudAtlas`, `Chat::layout`,
  `native_setting`, `tab_list::default_skin`) проверен только чтением
  исходников игры на этой ревизии.
- Что `atlas.object_skins` действительно содержит ключ `name:<ник>` для игроков
  сервера в момент отрисовки: ключи ставятся в `remote.rs`
  (`set_object_skin("name:{…}")`), падение — на стандартный скин.
- Совместимость с другими нативными модами (других сдвигов текста чата нет;
  порядок `MODIFY_VARIABLE` и `HEAD/RETURN` на `draw_chat` — по приоритетам).

## Если что-то не так

- Головы есть, но пустые/белые квадраты → скин-спрайт в атласе не тот путь;
  смотреть `atlas.object_skins`/`atlas.sprites` (ключ `skins/<sha1>` против
  `entity/player/...`).
- Головы съехали по вертикали → `Chat::layout` на этой ревизии считает `y` не
  как верх строки; проверить `bottom_to_text` в `crates/horizon-client-render/src/chat.rs`.
- Текст не сдвинулся, головы поверх букв → инъекция `shift` не сработала
  (статик `ON` не выставлен: смотреть `mark`, порядок хуков `draw_chat`).

## Measured (2026-10-30)

- Built with `horizon-launcher native-build … --trust-native-code` on
  `a46ce6898bd72016e061aaec1afc1375b384deac`; `horizon-mixins apply` reported
  `injections: 6, modules: 3, status: ok`.
- Scene: scratch world, `horizon-ctl chat "hello from chatheads"` (a real player chat line, not a
  system message). The screenshot shows `<Player88145> hello from chatheads` with an 8×8 head drawn
  to its left and the line's text shifted right by 10 px; the three system messages above it have no
  head.
- The head came from the built-in fallback (a default skin by name hash): single player has no
  downloaded skin for this profile. A real multiplayer skin was not observed.
