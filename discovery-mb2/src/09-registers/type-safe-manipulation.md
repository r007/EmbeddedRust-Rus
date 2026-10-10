# Типобезопасная работа с регистрами

Один из регистров `P0`, регистр `IN`, в документации описан как регистр только для чтения.

> 6.8.2.4 «IN» — стр. 145 и 146

Обратите внимание: в колонке «Access» таблицы для этого регистра указано только «R». Писать в него нельзя, иначе может случиться что-то плохое.

У регистров разные права доступа на чтение и запись. Одни доступны только для записи, другие — и для чтения, и для записи, а третьи, очевидно, только для чтения.

Работа с шестнадцатеричными адресами напрямую тоже чревата ошибками. Вы уже видели, что попытка обратиться к недопустимому адресу памяти вызвала исключение, которое нарушило выполнение программы.

Было бы неплохо иметь API для работы с регистрами «безопасным» способом. В идеале этот API должен учитывать три пункта, о которых я говорил: никакой возни с реальными адресами, соблюдение прав на чтение и запись и защита зарезервированных битов регистра от изменения.

Что ж, такой API у нас есть! `registers::init()` возвращает значение, которое предоставляет типобезопасный API для работы с регистрами портов `P0` и `P1`.

Как вы помните, группа регистров, связанная с периферией, называется блоком регистров и занимает непрерывную область памяти. В этом типобезопасном API каждый блок регистров описан как `struct`, где каждое поле соответствует регистру. Каждое поле регистра — это отдельный newtype над, например, `u32`, который предоставляет набор методов `read`, `write` или `modify` в зависимости от прав доступа. Наконец, эти методы принимают не примитивные значения вроде `u32`, а ещё один newtype, который собирается по шаблону «строитель» (builder) и не позволяет менять зарезервированные биты регистра.

Лучший способ познакомиться с этим API — перенести на него наш пример (`examples/type-safe.rs`).

```rust
{{#include examples/type-safe.rs}}
```

Первое, что вы заметите: никаких «магических» адресов здесь нет. Вместо них используется более понятная запись `p0.out` для обращения к регистру `OUT` в блоке регистров порта `P0`.

У блока регистров есть метод [`modify`], который принимает замыкание. Перед вызовом этого замыкания значение регистра `OUT` читается и передаётся в замыкание как параметр `r`. Зная значение `r`, с помощью методов `w` вы задаёте нужное новое значение регистра. Результат записывается в регистр, когда замыкание возвращает управление. В нашем случае текущее значение регистра также передаётся в параметр `w`, поэтому можно менять только `w`, когда нужно сохранить остальные биты регистра без изменений.

Метод `modify` определён для регистров, которые допускают и чтение, и запись. Если нужно только прочитать значение регистра, не меняя его, используйте метод [`read`]. А если хотите просто записать значение в регистр, не читая его, есть метод [`write`].

Регистры только для чтения предоставляют только `read`, а регистры только для записи — только `write`. Так пользователь не может обратиться к регистру недопустимым способом, и поэтому вызовы не нужно оборачивать в блок `unsafe`. И вам не нужно самим вычислять точный адрес регистра и позиции битов!

[`write`]: https://docs.rs/svd2rust/latest/svd2rust/#write
[`read`]: https://docs.rs/svd2rust/latest/svd2rust/#read
[`modify`]: https://docs.rs/svd2rust/latest/svd2rust/#modify

Запустим эту программу! Во время отладки можно сделать кое-что интересное.

`p0` — это ссылка на блок регистров порта `P0`. Команда `print p0` выведет базовый адрес блока регистров, а `print *p0` — его содержимое.

```
$ cargo run
(..)
Target halted
(gdb) set mem inaccessible-by-default off
(gdb) break main.rs:12
Breakpoint 4 at 0x162: main.rs:12. (2 locations)
(gdb) continue
Continuing.

Program received signal SIGINT, Interrupt.
cortex_m_rt::DefaultPreInit () at src/lib.rs:1058
1058	pub unsafe extern "C" fn DefaultPreInit() {}
(gdb) continue
Continuing.

Breakpoint 1, registers::__cortex_m_rt_main_trampoline () at src/07-registers/src/main.rs:7
7	#[entry]
(gdb) continue
Continuing.

Program received signal SIGINT, Interrupt.
registers::__cortex_m_rt_main () at src/07-registers/src/main.rs:8
8	fn main() -> ! {
(gdb) continue
Continuing.

Breakpoint 4.2, registers::__cortex_m_rt_main () at src/07-registers/src/main.rs:12
12	    p0.out.modify(|_, w| w.pin21().set_bit());
(gdb) print *p0                                               ; ⬅️ Выводим `*p0` здесь!
$1 = nrf52833_pac::p0::RegisterBlock {
  _reserved0: [0 <repeats 1284 times>],
  out: nrf52833_pac::generic::Reg<nrf52833_pac::p0::out::OUT_SPEC> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<nrf52833_pac::p0::out::OUT_SPEC>
  },
  outset: nrf52833_pac::generic::Reg<nrf52833_pac::p0::outset::OUTSET_SPEC> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<nrf52833_pac::p0::outset::OUTSET_SPEC>
  },
  outclr: nrf52833_pac::generic::Reg<nrf52833_pac::p0::outclr::OUTCLR_SPEC> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<nrf52833_pac::p0::outclr::OUTCLR_SPEC>
  },
  in_: nrf52833_pac::generic::Reg<nrf52833_pac::p0::in_::IN_SPEC> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<nrf52833_pac::p0::in_::IN_SPEC>
  },
  dir: nrf52833_pac::generic::Reg<nrf52833_pac::p0::dir::DIR_SPEC> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 3513288704
      }
    },
    _marker: core::marker::PhantomData<nrf52833_pac::p0::dir::DIR_SPEC>
  },
  dirset: nrf52833_pac::generic::Reg<nrf52833_pac::p0::dirset::DIRSET_SPEC> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 3513288704
      }
    },
    _marker: core::marker::PhantomData<nrf52833_pac::p0::dirset::DIRSET_SPEC>
  },
  dirclr: nrf52833_pac::generic::Reg<nrf52833_pac::p0::dirclr::DIRCLR_SPEC> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 3513288704
      }
    },
    _marker: core::marker::PhantomData<nrf52833_pac::p0::dirclr::DIRCLR_SPEC>
  },
  latch: nrf52833_pac::generic::Reg<nrf52833_pac::p0::latch::LATCH_SPEC> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<nrf52833_pac::p0::latch::LATCH_SPEC>
  },
  detectmode: nrf52833_pac::generic::Reg<nrf52833_pac::p0::detectmode::DETECTMODE_SPEC> {
    register: vcell::VolatileCell<u32> {
      value: core::cell::UnsafeCell<u32> {
        value: 0
      }
    },
    _marker: core::marker::PhantomData<nrf52833_pac::p0::detectmode::DETECTMODE_SPEC>
  },
  _reserved9: [0 <repeats 472 times>],
  pin_cnf: [nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    } <repeats 11 times>, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 3
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
--Type <RET> for more, q to quit, c to continue without paging--c
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 3
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 3
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 3
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 3
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 3
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 3
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 2
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 3
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }, nrf52833_pac::generic::Reg<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC> {
      register: vcell::VolatileCell<u32> {
        value: core::cell::UnsafeCell<u32> {
          value: 3
        }
      },
      _marker: core::marker::PhantomData<nrf52833_pac::p0::pin_cnf::PIN_CNF_SPEC>
    }]
}


```

Всё это обилие newtype и замыканий может показаться причиной раздутого кода. Однако если собрать программу в release-режиме с включённым [LTO], вы увидите в точности те же инструкции, что и в «небезопасной» версии, которая использовала `write_volatile` и шестнадцатеричные адреса!

[LTO]: https://en.wikipedia.org/wiki/Interprocedural_optimization

Используйте `cargo objdump`, чтобы сохранить ассемблерный код в `release.type-safe.dump`:
``` console
cargo objdump -q --release --example type-safe -- --disassemble --no-show-raw-insn  > release.type-safe.dump
```

Затем найдите `main` в `release.type-safe.dump`:

```
00000158 <main>:
     158:      	push	{r7, lr}
     15a:      	mov	r7, sp
     15c:      	bl	0x160 <registers::__cortex_m_rt_main::h0e9b57c6799332fd> @ imm = #0x0

00000160 <registers::__cortex_m_rt_main::h0e9b57c6799332fd>:
     160:      	push	{r7, lr}
     162:      	mov	r7, sp
     164:      	bl	0x192 <registers::init::hec71dddc40be11b5> @ imm = #0x2a
     168:      	movw	r0, #0x504
     16c:      	movt	r0, #0x5000
     170:      	ldr	r1, [r0]
     172:      	orr	r1, r1, #0x200000
     176:      	str	r1, [r0]
     178:      	ldr	r1, [r0]
     17a:      	orr	r1, r1, #0x80000
     17e:      	str	r1, [r0]
     180:      	ldr	r1, [r0]
     182:      	bic	r1, r1, #0x200000
     186:      	str	r1, [r0]
     188:      	ldr	r1, [r0]
     18a:      	bic	r1, r1, #0x80000
     18e:      	str	r1, [r0]
     190:      	b	0x190 <registers::__cortex_m_rt_main::h0e9b57c6799332fd+0x30> @ imm = #-0x4
```

Можно убедиться, что получается в точности тот же бинарник, что и при вызовах `ptr::read_volatile` и `ptr::write_volatile`.

Лучше всего то, что никому не пришлось написать ни строчки кода для реализации API GPIO. Весь код был автоматически сгенерирован из файла System View Description (SVD) с помощью инструмента [svd2rust]. Этот файл SVD — по сути XML-файл, который производители микроконтроллеров предоставляют и который содержит карту регистров их микроконтроллеров. В файле описаны расположение блоков регистров, базовые адреса, права на чтение и запись каждого регистра, структура регистров, наличие зарезервированных битов и множество другой полезной информации.

[svd2rust]: https://crates.io/crates/svd2rust
