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
	
	Если Не ПодключитьВнешнююКомпоненту(ИмяФайла, "MedIgor", ТипВнешнейКомпоненты.Native, ТипПодключенияВнешнейКомпоненты.НеИзолированно) Тогда
		ВызватьИсключение "Не удалось подключить внешнюю компоненту";
	КонецЕсли;
	
	Socket = Новый ("Addin.MedIgor.ZeroMQ.Req");
	
	Для Каждого КонечнаяТочка Из КонечныеТочки Цикл
		ПроверитьКонечнуюТочку(Socket, КонечнаяТочка);
	КонецЦикла;
	
	Socket.Connect(КонечныеТочки[0]);
	Socket.Send(ПолучитьДвоичныеДанныеИзСтроки("Стоп"));
	Ответ = Socket.Recv(1000);
	
	Задание = Задание.ОжидатьЗавершенияВыполнения(1);
	Если Задание.Состояние <> СостояниеФоновогоЗадания.Завершено Тогда
		ВызватьИсключение СтрШаблон("Задание %1", Задание.Состояние);
	КонецЕсли;
	
	УдалитьФайлы(ВременныйКаталог);
	
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
Процедура ПроверитьОтвет(Ответ)
	
	Если Ответ = Неопределено Тогда
		ВызватьИсключение "Не удалось получить ответ";
	ИначеЕсли ПолучитьСтрокуИзДвоичныхДанных(Ответ) <> "Получено: 123" Тогда
		ВызватьИсключение "Получить неверный ответ";
	КонецЕсли;
	
КонецПроцедуры
