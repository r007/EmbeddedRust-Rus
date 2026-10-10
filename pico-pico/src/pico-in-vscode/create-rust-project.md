{{#title Создание проекта Rust для Raspberry Pi Pico 2 в VS Code}}

# Создание проекта на Rust для Raspberry Pi Pico в VS Code (с расширением)

Проект Rust для Pico мы уже создавали вручную и через шаблон. Теперь попробуем другой подход: расширение Raspberry Pi Pico для VS Code.

## Использование расширения Pico

В Visual Studio Code найдите расширение «Raspberry Pi Pico» и убедитесь, что устанавливаете официальное: у него должен быть значок проверенного издателя и ссылка на официальный сайт Raspberry Pi. Установите это расширение.

<div class="image-with-caption" style="text-align:center; display:inline-block;">
    <img src="./images/Raspberry Pi Pico Vscode extension.png" alt="Расширение VS Code для Raspberry Pi Pico" style="max-width:100%; height:auto; display:block; margin:0 auto;"/>
    <div class="caption" style="font-size:0.9em; color:#555; margin-top:6px;">Расширение VS Code для Raspberry Pi Pico</div>
</div>

Одной установки расширения может не хватить — всё зависит от того, что уже установлено в вашей системе. В Linux вам, скорее всего, понадобятся базовые зависимости:

```sh
sudo apt install build-essential libudev-dev
```

## Создание проекта

Создадим проект на Rust с помощью расширения Pico в VS Code. Откройте панель Activity Bar слева и нажмите значок Pico. Затем выберите «New Rust Project».

<div class="image-with-caption" style="text-align:center; display:inline-block;">
    <img src="./images/Create Project Raspberry Pi Pico Vscode extension.png" alt="Создание проекта через расширение Raspberry Pi Pico для VS Code" style="max-width:100%; height:auto; display:block; margin:0 auto;"/>
    <div class="caption" style="font-size:0.9em; color:#555; margin-top:6px;">Создание проекта</div>
</div>

Так как это первая настройка, расширение загрузит и установит необходимые инструменты: Pico SDK, picotool, OpenOCD, а также тулчейны ARM и RISC-V для отладки.

## Структура проекта

Если проект создан успешно, вы увидите папки и файлы примерно такого вида:

<div class="image-with-caption" style="text-align:center; display:inline-block;">
    <img src="./images/Raspberry Pi Pico Rust Project Created With VS Code Extension.png" alt="Проект Rust для Raspberry Pi Pico, созданный через расширение VS Code" style="max-width:100%; height:auto; display:block; margin:0 auto;"/>
    <div class="caption" style="font-size:0.9em; color:#555; margin-top:6px;">Папка проекта</div>
</div>

## Запуск программы

Теперь достаточно нажать «Run Project (USB)», чтобы прошить программу на Pico и запустить её. Не забудьте нажать кнопку BOOTSEL, когда подключаете Pico к компьютеру, иначе эта опция будет неактивна.

<div class="image-with-caption" style="text-align:center; display:inline-block;">
    <img src="./images/Running Rust Project with Vscode for Raspberry Pi Pico 2 (RP2350).png" alt="Запуск проекта Rust для Raspberry Pi Pico 2 (RP2350) в VS Code" style="max-width:100%; height:auto; display:block; margin:0 auto;"/>
    <div class="caption" style="font-size:0.9em; color:#555; margin-top:6px;">Прошивка Rust-программы в Raspberry Pi Pico</div>
</div>

Когда прошивка завершится, программа сразу начнёт работать на вашем Pico. Вы должны увидеть, как встроенный светодиод мигает.
