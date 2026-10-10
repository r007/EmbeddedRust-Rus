# Чек-лист паттернов проектирования HAL

- **Именование** *(крейт соответствует соглашениям Rust об именовании)*
  - [ ] Крейт назван надлежащим образом ([C-CRATE-NAME])
- **Совместимость** *(крейт корректно взаимодействует с другой библиотечной функциональностью)*
  - [ ] Типы-обёртки предоставляют метод деструктора ([C-FREE])
  - [ ] HAL реэкспортируют свой крейт доступа к регистрам ([C-REEXPORT-PAC])
  - [ ] Типы реализуют трейты `embedded-hal` ([C-HAL-TRAITS])
- **Предсказуемость** *(крейт делает код понятным, а его поведение соответствует внешнему виду кода)*
  - [ ] Используются конструкторы вместо extension-трейтов ([C-CTOR])
- **Интерфейсы GPIO** *(интерфейсы GPIO следуют общему шаблону)*
  - [ ] Типы выводов по умолчанию имеют нулевой размер ([C-ZST-PIN])
  - [ ] Типы выводов предоставляют методы стирания типа вывода и порта ([C-ERASED-PIN])
  - [ ] Состояние вывода должно кодироваться как параметры типа ([C-PIN-STATE])

[C-CRATE-NAME]: naming.html#c-crate-name

[C-FREE]: interoperability.html#c-free
[C-REEXPORT-PAC]: interoperability.html#c-reexport-pac
[C-HAL-TRAITS]: interoperability.html#c-hal-traits

[C-CTOR]: predictability.html#c-ctor

[C-ZST-PIN]: gpio.md#c-zst-pin
[C-ERASED-PIN]: gpio.md#c-erased-pin
[C-PIN-STATE]: gpio.md#c-pin-state
