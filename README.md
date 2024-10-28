# addin-zeromq

Внешняя компонента основанная на [ZeroMQ](https://zeromq.org/)

Реализованы паттерны:
- [Request-Reply](https://zeromq.org/socket-api/#request-reply-pattern) - это простой клиент-сервер, представлен объектами `ZeroMQ.Req` и `ZeroMQ.Rep`.
- [Publish-Subscribe](https://zeromq.org/socket-api/#publish-subscribe-pattern) - публикация сообщений для всех подписчиков, представлен объектами `ZeroMQ.Pub` и `ZeroMQ.Sub`. Этот паттерн не очень надежен, т.к. нет подтверждения получения, нет информации об подписчиках, сообщения отбрасываются при отсутствии соединения.
- [Pipeline](https://zeromq.org/socket-api/#pipeline-pattern) - это паттерн распределения задач, задачи по очереди получают все подписчики, представлен объектами `ZeroMQ.Push` и `ZeroMQ.Pull`.

## Для чего нужна
1. Для общения между фоновыми заданиями, для эффективной реализации параллельной обработки данных. Предполагается, что задания-воркеры будут получать задания у управляещего задания по мере их выполнения, таким образом реализуется более точное распределение задач между фоновыми заданиями.
2. Для общения 1С с другими сервисами, в т.ч. написанных на других языках.

## Ограничения
Библиотека не использует защищенное соединение, поэтому работа в общественных сетях опасна. Предполагается, что компонента будет использоваться в основном на одном сервере.

## API
Документация в разработке, см. код тестов:
- [ОбщийМодуль.Задания](conf/CommonModules/Задания/Ext/Module.bsl)
- [Обработка.Тесты](conf/DataProcessors/Тесты/Forms/Форма/Ext/Form/Module.bsl)

У всех объектов есть свойство `LastError` типа `Строка`, в нем будет содержаться ошибка в случае исключения при обращении к компоненте.

Компонента позволяет обмениваться сообщениями. Сообщение - это массив байт, в 1С это тип `ДвоичныеДанные`. Также поддерживаются составные (multipart) сообщения, это сообщения, которые состоят из нескольких частей, каждая из которых массив байт.

Для подключения можно использовать следующие виды транспортов (в порядке уменьшения скорости):
- `внутрипроцессное` - это конечные точки вида `"inproc://some_id"`, см. также [zmq_inproc](https://libzmq.readthedocs.io/en/latest/zmq_inproc.html). Чтобы убедиться, что это возможно, можно использовать свойство `Instance` объекта `ZeromMQ.Info`.
- `unix socket` - для взаимодействия между процессами на одном хосте, это конечные точки вида `"ipc://path/to/file"`. Поддержка в `Windows` есть начиная с определенных версий, ищите эту информацию самостоятельно. См. также описание [zmq_ipc](https://libzmq.readthedocs.io/en/latest/zmq_ipc.html).
- `tcp` - для взаимодействия между процессами на одном и разных хостах,  также если `unix socket` не поддерживается. Конечные точки могут выглядеть как `tcp://127.0.0.1:1234`, другие примеры смотри в [zmq_tcp](https://libzmq.readthedocs.io/en/latest/zmq_tcp.html). В случае использования подключения с разных хостов, рекомендуется обеспечить безопасность такого подключения самостоятельно (vpn/firewall/etc).

### ZeroMQ.Rep
Сокет `Rep` используется для обработки запросов от клиентов, при получении запроса нужно дать ответ клиенту. Все запросы клиентов выстраиваются в очередь и сервер их обслуживает по очереди.

Методы:
- `Bind(Endpoint: String)` - привязывает сокет к конечной точке и начинает принимать соединения, вызывается метод [zmq_bind](https://libzmq.readthedocs.io/en/latest/zmq_bind.html).
- `Unbind(Endpoint: String)` - отвязывает ранее привязанную конечную точку. Используется метод [zmq_unbind](https://libzmq.readthedocs.io/en/latest/zmq_unbind.html).
- `SetRecvTimeout(timeout: Число)` - задает таймаут получения сообщения, если метод не вызывался, то таймаут равен 60 сек, см. также [ZMQ_RCVTIMEO](https://libzmq.readthedocs.io/en/latest/zmq_setsockopt.html#_zmq_rcvtimeo_maximum_time_before_a_recv_operation_returns_with_eagain).
- `Send(data: ДвоичныеДанные)` - отправляет ответ клиенту, вызывается метод [zmq_msg_send](https://libzmq.readthedocs.io/en/latest/zmq_msg_send.html).
- `SendPart(data: ДвоичныеДанные)` - отправляет часть составного сообщения, при этом последняя часть должна быть отправлена с помощью метода `Send`.
- `Recv(): ДвоичныеДанные|Неопределено` - получает данные от клиента с ожиданием. Если таймаут вышел, то вернется Неопределено. Используется последовательный вызов методов: [zmq_poll](https://libzmq.readthedocs.io/en/latest/zmq_poll.html) и [zmq_msg_recv](https://libzmq.readthedocs.io/en/latest/zmq_msg_recv.html).
- `RecvMultipart(): Число|Неопределено` - получает составное сообщение от клиента. Возвращает количество частей, либо Неопределено, если таймаут вышел. Данные части можно получить с помощью метода `GetPart`.
- `GetPart(НомерЧасти: Число): ДвоичныеДанные` - получает выбранную часть составного сообщения.

### ZeroMQ.Req
Сокет `Req` используется в качестве клиента, после отправки запроса нужно ожидать получения ответа.

Методы:
- `Connect(Endpoint: String)` - выполняет подключение к конечной точке, после подключения можно отправлять запросы. Используется метод [zmq_connect](https://libzmq.readthedocs.io/en/latest/zmq_connect.html). 
- `Disconnect(Endpoint: String)` - выполняет отключение от конечной точки. Используется метод [zmq_disconnect](https://libzmq.readthedocs.io/en/latest/zmq_disconnect.html).
- `SetRecvTimeout(timeout: Число)` - задает таймаут получения сообщения, если метод не вызывался, то таймаут равен 60 сек, см. также [ZMQ_RCVTIMEO](https://libzmq.readthedocs.io/en/latest/zmq_setsockopt.html#_zmq_rcvtimeo_maximum_time_before_a_recv_operation_returns_with_eagain).
- `Send(data: ДвоичныеДанные)` - отправляет запрос серверу, используется метод [zmq_msg_send](https://libzmq.readthedocs.io/en/latest/zmq_msg_send.html).
- `SendPart(data: ДвоичныеДанные)` - отправляет часть составного сообщения, при этом последняя часть должна быть отправлена с помощью метода `Send`.
- `Recv(): ДвоичныеДанные|Неопределено` - получает ответ от сервера с ожиданием. Если таймаут вышел, то вернется Неопределено. Используется последовательный вызов методов: [zmq_poll](https://libzmq.readthedocs.io/en/latest/zmq_poll.html) и [zmq_msg_recv](https://libzmq.readthedocs.io/en/latest/zmq_msg_recv.html).
- `RecvMultipart(): Число|Неопределено` - получает составное сообщение от сервера. Возвращает количество частей, либо Неопределено, если таймаут вышел. Данные части можно получить с помощью метода `GetPart`.
- `GetPart(НомерЧасти: Число): ДвоичныеДанные` - получает выбранную часть составного сообщения.

### ZeroMQ.Pub
Публикует сообщения одному или нескольким `ZeroMQ.Sub`.

Методы:
- `Bind(Endpoint: String)` - описание см. в объекте `ZeroMQ.Rep`.
- `Unbind(Endpoint: String)` - описание см. в объекте `ZeroMQ.Rep`.
- `Connect(Endpoint: String)` - описание см. в объекте `ZeroMQ.Req`.
- `Disconnect(Endpoint: String)` - описание см. в объекте `ZeroMQ.Req`.
- `Send(data: ДвоичныеДанные)` - описание см. в объектах `ZeroMQ.Rep`/`ZeroMQ.Req`.
- `SendPart(data: ДвоичныеДанные)` - описание см. в объектах `ZeroMQ.Rep`/`ZeroMQ.Req`.


### ZeroMQ.Sub
Получает сообщения от `ZeroMQ.Pub`.

Методы:
- `Bind(Endpoint: String)` - описание см. в объекте `ZeroMQ.Rep`.
- `Unbind(Endpoint: String)` - описание см. в объекте `ZeroMQ.Rep`.
- `Connect(Endpoint: String)` - описание см. в объекте `ZeroMQ.Req`.
- `Disconnect(Endpoint: String)` - описание см. в объекте `ZeroMQ.Req`.
- `Subscribe(topic: ДвоичныеДанные)` - подписывается на определенный топик, для подписки на все топики можно передать пустой ДвоичныеДанные, см. также  [ZMQ_SUBSCRIBE](https://libzmq.readthedocs.io/en/latest/zmq_setsockopt.html#_zmq_subscribe_establish_message_filter).
- `SetRecvTimeout(timeout: Число)` - описание см. в объекте `ZeroMQ.Req`.
- `Recv(): ДвоичныеДанные|Неопределено` - описание см. в объектах `ZeroMQ.Rep`/`ZeroMQ.Req`.
- `RecvMultipart(): Число|Неопределено` - описание см. в объектах `ZeroMQ.Rep`/`ZeroMQ.Req`.
- `GetPart(НомерЧасти: Число): ДвоичныеДанные` - описание см. в объектах `ZeroMQ.Rep`/`ZeroMQ.Req`.


### ZeroMQ.Push
Публикует сообщения одному или нескольким `ZeroMQ.Pull`.

Методы:
- `Bind(Endpoint: String)` - описание см. в объекте `ZeroMQ.Rep`.
- `Unbind(Endpoint: String)` - описание см. в объекте `ZeroMQ.Rep`.
- `Connect(Endpoint: String)` - описание см. в объекте `ZeroMQ.Req`.
- `Disconnect(Endpoint: String)` - описание см. в объекте `ZeroMQ.Req`.
- `SetSendTimeout(timeout: Число)` - задает таймаут отправки сообщения, если метод не вызывался, то таймаут равен 60 сек, см. также [ZMQ_SNDTIMEO](https://libzmq.readthedocs.io/en/latest/zmq_setsockopt.html#_zmq_sndtimeo_maximum_time_before_a_send_operation_returns_with_eagain).
- `Send(data: ДвоичныеДанные): Булево` - отправляет сообщение в блокирующем режиме, при успешной отправке возвращает `Истина`, если таймаут вышел - `Ложь`, вызывается метод [zmq_msg_send](https://libzmq.readthedocs.io/en/latest/zmq_msg_send.html).
- `SendPart(data: ДвоичныеДанные)` - описание см. в объектах `ZeroMQ.Rep`/`ZeroMQ.Req`.


### ZeroMQ.Pull
Получает сообщения от `ZeroMQ.Push`.

Методы:
- `Bind(Endpoint: String)` - описание см. в объекте `ZeroMQ.Rep`.
- `Unbind(Endpoint: String)` - описание см. в объекте `ZeroMQ.Rep`.
- `Connect(Endpoint: String)` - описание см. в объекте `ZeroMQ.Req`.
- `Disconnect(Endpoint: String)` - описание см. в объекте `ZeroMQ.Req`.
- `SetRecvTimeout(timeout: Число)` - описание см. в объекте `ZeroMQ.Req`.
- `Recv(): ДвоичныеДанные|Неопределено` - описание см. в объектах `ZeroMQ.Rep`/`ZeroMQ.Req`.
- `RecvMultipart(): Число|Неопределено` - описание см. в объектах `ZeroMQ.Rep`/`ZeroMQ.Req`.
- `GetPart(НомерЧасти: Число): ДвоичныеДанные` - описание см. в объектах `ZeroMQ.Rep`/`ZeroMQ.Req`.


### ZeroMQ.Info
Информация о компоненте

Свойства:
- `Instance: Строка` - содержит идентификатор конкретного экземпляра в виде `guid`. Идентификатор генерируется при каждой загрузке динамической библиотеки. Если в двух разных сеансах он совпадает, то между этими сеансами можно использовать внутрипроцессное взаимодействие.
- `VersionZeroMQ: Строка` - содержит версию ZeroMQ.
- `VersionAddin: Строка` - содержит версию компоненты (содержится в поле `package.version` в файле [Cargo.toml](Cargo.toml)).

## Сборка
См. https://github.com/medigor/rust-build-scripts, но собрать на `Linux` для `Windows` не получится, т.к. исходная библиотека [libzmq](https://github.com/zeromq/libzmq/) это не позволяет.
