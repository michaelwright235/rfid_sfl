# RFID Server For Libraries

**RFID Server For Libraries** (RFID SFL) – программа-сервер, предоставляющая мост между считывателем библиотечных карт на компьютере пользователя и сторонними сайтами.
Является заменой проприетарного аналога от компании DITM. Проверена работоспособность на Windows, Linux и macOS.

В данный момент программа работает только со считывателем [Chafon CF-RH320U-93](https://sl.aliexpress.ru/p?key=Zucn0R).

## Описание работы

RFID SFL создает локальный сервер (по-умолчанию – 127.0.0.1:21646). Сервер отправляет заголовок `Access-Control-Allow-Origin=*`, поэтому к нему могут обращаться сайты прямо из браузера.

- `/` и `/rfid` (GET)– HTML-страница для проверки работоспособности.
- `/rfid/?action=getDevicesList` (GET)– JSON-ответ со списком доступных считывателей.
- `/rfid/?action=getItemsList&deviceId=<deviceId>` (GET) – JSON-ответ с данными карт, которые находятся в зоне доступа указанного считывателя. Карта должна содержать данные, соответствующие стандарту [RFID Data Model for Libraries (Danish Standard)](https://biblev.no/RFID/dansk_rfid_datamodel.pdf).
- `/rfid` (POST, `action=writeTags&<params..>`) – записывает на карту данные.

## Конфигурация

В папке с исполняемым файлом при первом запуске создается файл настроек `config.json`. При ошибке чтения/парсинга будет использоваться конфигурация по-умолчанию.

|Название|Формат|Значение по-умолчанию| Описание |
|--|--|--|--|
|address|String|127.0.0.1|Адрес, по которому будет запущен сервер|
|port|u16|21646|Порт адреса|
|log_to_file|bool|true|Вести ли лог в файл `rfid_sfl.log`. В любом случае лог выводится в stdout|
|log_level|String|Info|Уровень лога. Доступные значения: Off, Error, Warn, Info, Debug, Trace|
|max_log_size|u16|5|Максимальный размер файла лога в мегабайтах. После достижения лимита файл удаляется, и создается новый|
|ask_when_writing|bool|false|Выводить ли диалоговое окно с вопросом о записи карты.|

## Безопасность

Первоначальная программа была создана с серьезной уязвимостью, которую пришлось перенести в RFID SFL, иначе существующие сайты не могли бы пользоваться ПО. Главная проблема состоит в том, что по факту любой сайт без какой-либо проверки может считывать и записывать карты. Это открывает возможность злоумышленникам создавать сайты с подобным функционалом. RFID SFL пытается компенсировать это благодаря параметру `ask_when_writing` в конфигурации. При его включении пользователь должен будет подтвердить операцию записи данных на карту.

## Сборка

Установите среду Rust, согласно [официальной инструкции](https://www.rust-lang.org/tools/install). Затем скачайте исходный код и в папке выполните следующую команду:

```bash
cargo build --release
```

Готовый исполняемый файл будет находится в папке `target/release`.
