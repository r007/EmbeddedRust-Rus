# Адрес `0xBAAAAAAD`

Не вся периферийная память доступна для чтения и записи. Посмотрите на эту программу (`examples/bad.rs`).

```rust
{{#include examples/bad.rs}}
```

Этот адрес находится рядом с адресом `OUT`, который мы использовали раньше, но он *недействителен* — по этому адресу нет никакого регистра.

Теперь попробуем.

``` console
$ cargo run
(..)
Resetting and halting target
Target halted
(gdb) continue
Continuing.

Breakpoint 1, registers::__cortex_m_rt_main_trampoline () at src/07-registers/src/main.rs:9
9	#[entry]
(gdb) continue
Continuing.

Program received signal SIGINT, Interrupt.
registers::__cortex_m_rt_main () at src/07-registers/src/main.rs:10
10	fn main() -> ! {
(gdb) continue
Continuing.

Breakpoint 3, cortex_m_rt::HardFault_ (ef=0x2001ffb8) at src/lib.rs:1046
1046	    loop {}
(gdb) 
```

Мы попытались выполнить недопустимую операцию — чтение несуществующей памяти, поэтому процессор вызвал *исключение* — *аппаратное* исключение.

В большинстве случаев исключения возникают, когда процессор пытается выполнить недопустимую операцию. Исключения прерывают обычный ход программы и заставляют процессор выполнить *обработчик исключения* — обычную функцию (подпрограмму).

Существуют разные виды исключений. Каждый вид возникает при своих условиях и обрабатывается своим обработчиком.

Крейт `registers` зависит от крейта `cortex-m-rt`, который определяет обработчик *hard fault* по умолчанию — `HardFault_` — и обрабатывает исключение «недопустимый адрес памяти». `embed.gdb` поставил точку останова на `HardFault`, поэтому отладчик остановил программу во время выполнения обработчика исключения. Подробности об исключении можно получить из отладчика. Смотрим:

```
(gdb) list
1040  #[allow(unused_variables)]
1041	#[doc(hidden)]
1042	#[cfg_attr(cortex_m, link_section = ".HardFault.default")]
1043	#[no_mangle]
1044	pub unsafe extern "C" fn HardFault_(ef: &ExceptionFrame) -> ! {
1045	    #[allow(clippy::empty_loop)]
1046	    loop {}
1047	}
1048	
1049	#[doc(hidden)]
1050	#[no_mangle]
```

`ef` — это снимок состояния программы непосредственно перед возникновением исключения. Посмотрим на него:

```
(gdb) print/x *ef
$1 = cortex_m_rt::ExceptionFrame {
  r0: 0x5000a784,
  r1: 0x3,
  r2: 0x2001ff24,
  r3: 0x0,
  r12: 0x1,
  lr: 0x4403,
  pc: 0x43ea,
  xpsr: 0x1000000
}
```

Здесь несколько полей, но самое важное — `pc`, регистр счётчика команд (Program Counter). Адрес в этом регистре указывает на инструкцию, которая вызвала исключение. Дизассемблируем программу вокруг неверной инструкции.

```
(gdb) disassemble /m ef.pc
Dump of assembler code for function core::ptr::read_volatile<u32>:
1654	pub unsafe fn read_volatile<T>(src: *const T) -> T {
   0x000043d2 <+0>:	push	{r7, lr}
   0x000043d4 <+2>:	mov	r7, sp
   0x000043d6 <+4>:	sub	sp, #16
   0x000043d8 <+6>:	str	r0, [sp, #4]
   0x000043da <+8>:	str	r0, [sp, #8]

1655	    // SAFETY: the caller must uphold the safety contract for `volatile_load`.
1656	    unsafe {
1657	        assert_unsafe_precondition!(
   0x000043dc <+10>:	b.n	0x43de <core::ptr::read_volatile<u32>+12>
   0x000043de <+12>:	ldr	r0, [sp, #4]
   0x000043e0 <+14>:	movs	r1, #4
   0x000043e2 <+16>:	bl	0x43f4 <core::ptr::read_volatile::precondition_check>
   0x000043e6 <+20>:	b.n	0x43e8 <core::ptr::read_volatile<u32>+22>

1658	            check_language_ub,
1659	            "ptr::read_volatile requires that the pointer argument is aligned and non-null",
1660	            (
1661	                addr: *const () = src as *const (),
1662	                align: usize = align_of::<T>(),
1663	            ) => is_aligned_and_not_null(addr, align)
1664	        );
1665	        intrinsics::volatile_load(src)
   0x000043e8 <+22>:	ldr	r0, [sp, #4]
   0x000043ea <+24>:	ldr	r0, [r0, #0]          ; <-- That's the one!
   0x000043ec <+26>:	str	r0, [sp, #12]
   0x000043ee <+28>:	ldr	r0, [sp, #12]

1666	    }
1667	}
   0x000043f0 <+30>:	add	sp, #16
   0x000043f2 <+32>:	pop	{r7, pc}

End of assembler dump.

```

Исключение вызвала инструкция `ldr r0, [r0, #0]` — инструкция чтения. Она пыталась прочитать память по адресу, который находился в *регистре процессора* `r0`. Кстати, регистр процессора — это не регистр, отображённый в память: у него нет адреса, как, например, у `OUT`.

Было бы неплохо проверить значение регистра `r0` в тот самый момент, когда возникло исключение. Но мы уже это сделали! Поле `r0` в значении `ef`, которое мы выводили раньше, — это значение регистра `r0` в момент исключения. Вот оно ещё раз:

```
(gdb) print/x *ef
$1 = cortex_m_rt::ExceptionFrame {
  r0: 0x5000a784,
  r1: 0x3,
  r2: 0x2001ff24,
  r3: 0x0,
  r12: 0x1,
  lr: 0x4403,
  pc: 0x43ea,
  xpsr: 0x1000000
}
```

`r0` содержит значение `0x5000_A784` — тот недопустимый адрес, с которым мы вызвали функцию `read_volatile`.
