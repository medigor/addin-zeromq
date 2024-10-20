﻿
&НаКлиенте
Процедура Тест1(Команда)
	Тест1НаСервере(ИмяФайла);
КонецПроцедуры

&НаСервереБезКонтекста
Процедура Тест1НаСервере(ИмяФайла)
	
	ВременныйКаталог = ПолучитьИмяВременногоФайла("zeromq") + ПолучитьРазделительПути();
	СоздатьКаталог(ВременныйКаталог);
	
	КонечныеТочки = Новый Массив;
	КонечныеТочки.Добавить("inproc://#1");
	КонечныеТочки.Добавить("ipc://" + ВременныйКаталог + "1");
	КонечныеТочки.Добавить("tcp://127.0.0.1:5556");
	
	ПараметрыЗадания = Новый Массив;
	ПараметрыЗадания.Добавить(ИмяФайла);
	ПараметрыЗадания.Добавить(КонечныеТочки);
	
	Задание = ФоновыеЗадания.Выполнить("Задания.Сервер1", ПараметрыЗадания);
	
	Если Не ПодключитьВнешнююКомпоненту(ИмяФайла, "MedIgor", ТипВнешнейКомпоненты.Native, 
		ТипПодключенияВнешнейКомпоненты.НеИзолированно) Тогда
		ВызватьИсключение "Не удалось подключить внешнюю компоненту";
	КонецЕсли;
	
	Socket = Новый ("Addin.MedIgor.ZeroMQ.Req");
	
	Для Каждого КонечнаяТочка Из КонечныеТочки Цикл
		ПроверитьКонечнуюТочку(Socket, КонечнаяТочка);
	КонецЦикла;
	
	Для Каждого КонечнаяТочка Из КонечныеТочки Цикл
		ТестБыстродействия(Socket, КонечнаяТочка);
	КонецЦикла;
	
	Socket.Connect(КонечныеТочки[0]);
	Socket.Send(ПолучитьДвоичныеДанныеИзСтроки("Стоп"));
	Ответ = Socket.Recv(1000);
	
	Задание = Задание.ОжидатьЗавершенияВыполнения(1);
	Если Задание.Состояние <> СостояниеФоновогоЗадания.Завершено Тогда
		ВызватьИсключение СтрШаблон("Задание %1", Задание.Состояние);
	КонецЕсли;
	
	УдалитьФайлы(ВременныйКаталог);
	
	Info = Новый ("Addin.MedIgor.ZeroMQ.Info");
	Попытка
		Instance = Новый УникальныйИдентификатор(Info.Instance);
	Исключение
		ВызватьИсключение "Не удалось получить свойство Info.Instance";
	КонецПопытки;
	
	Попытка
		Проверка = Новый УникальныйИдентификатор(Info.Instance);
	Исключение
		ВызватьИсключение "Не удалось получить свойство Info.Instance";
	КонецПопытки;
	
	Попытка
		Проверка = Info.VersionZeroMQ;
	Исключение
		ВызватьИсключение "Не удалось получить свойство Info.VersionZeroMQ";
	КонецПопытки;
	
	Попытка
		Проверка = Info.VersionAddin;
	Исключение
		ВызватьИсключение "Не удалось получить свойство Info.VersionAddin";
	КонецПопытки;
	
	Сообщить("Тест выполнен успешно!");
	
КонецПроцедуры

&НаСервереБезКонтекста
Процедура ПроверитьКонечнуюТочку(Socket, КонечнаяТочка)
	
	Socket.Connect(КонечнаяТочка);
	
	Попытка
		Socket.Send(ПолучитьДвоичныеДанныеИзСтроки("123"));
	Исключение
		ВызватьИсключение Socket.LastError;
	КонецПопытки;
	
	Попытка
		Ответ = Socket.Recv(1000);
	Исключение
		ВызватьИсключение Socket.LastError;
	КонецПопытки;
	ПроверитьОтвет(Ответ);
	
	Socket.Disconnect(КонечнаяТочка);
	
КонецПроцедуры

&НаСервереБезКонтекста
Процедура ТестБыстродействия(Socket, КонечнаяТочка)
	
	Socket.Connect(КонечнаяТочка);
	Socket.Send(ПолучитьДвоичныеДанныеИзСтроки("123"));
	Ответ = Socket.Recv(1000);
	
	Количество = 10 * 1000;
	
	Начало = ТекущаяУниверсальнаяДатаВМиллисекундах();
	Для К = 1 По Количество Цикл
		Socket.Send(ПолучитьДвоичныеДанныеИзСтроки("123"));
		Ответ = Socket.Recv(1000);
	КонецЦикла;
	Конец = ТекущаяУниверсальнаяДатаВМиллисекундах();
	
	Socket.Disconnect(КонечнаяТочка);
	Сообщить(СтрШаблон("Конечная точка: ""%1"", длительность: %2 мс, кол-во запросов: %3", КонечнаяТочка, Конец - Начало, Количество));
	
КонецПроцедуры

&НаСервереБезКонтекста
Процедура ПроверитьОтвет(Ответ)
	
	Если Ответ = Неопределено Тогда
		ВызватьИсключение "Не удалось получить ответ";
	ИначеЕсли ПолучитьСтрокуИзДвоичныхДанных(Ответ) <> "Получено: 123" Тогда
		ВызватьИсключение "Получить неверный ответ";
	КонецЕсли;
	
КонецПроцедуры
