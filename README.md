# Portal Explorer

**Portal Explorer** — это веб-визуализатор невероятных порталов, который позволяет исследовать их свойства и конструкцию в интерактивном режиме.

## 🚀 Описание

Portal Explorer — это программа для просмотра и создания сцен с уникальными и необычными порталами. Главной особенностью программы является то, что она запускается в браузере, благодаря чему любой пользователь может быстро и легко посмотреть на самые необычные порталы.

### Основные возможности

- **Визуальный редактор** — интуитивно понятный интерфейс для конструирования сцен
- **Интерактивное исследование** — перемещение и вращение порталов для изучения их свойств
- **Реальное время** — все изменения отображаются мгновенно
- **Трассировка лучей** — рендеринг порталов выполняется через шейдеры, что позволяет визуализировать порталы невероятной сложности без аппроксимаций

### Технические детали

Порталы рендерятся с помощью технологии трассировки лучей, которая исполняется внутри шейдеров. Благодаря этому можно визуализировать порталы невероятной сложности, а код для их рендеринга остается простым. Порталы отображаются в их истинном обличии, без упрощений.

Из-за того, что рендеринг происходит через шейдеры, единственным способом представить сцену с множеством разных объектов является генерация и компиляция кода шейдеров в реальном времени. Поэтому для некоторых действий требуется перекомпиляция проекта. Однако для таких вещей, как настройка матриц объектов, перекомпиляция не требуется, так как они передаются через uniform'ы.

> ⚠️ **Примечание:** Программа лучше всего работает на ПК. На мобильных устройствах производительность может быть ограничена.

## 📺 Видео

Я создал YouTube-видео о том, что произойдет, если поместить один портал в другой, с подробным объяснением принципов работы.

Видео имеет английские субтитры (основной язык — русский).

🔗 [Смотреть видео на YouTube](https://youtu.be/1q0sHf_n_2Y)

## 🎮 Доступные сцены

### Основы
- [Основы](https://optozorax.github.io/portal/?scene=basics)
- [Движущийся проход](https://optozorax.github.io/portal/?scene=moving_doorway)
- [Модель скорости](https://optozorax.github.io/portal/?scene=speed_model)
- [Неравномерное движение](https://optozorax.github.io/portal/?scene=non_linear)
- [Одинаковая форма](https://optozorax.github.io/portal/?scene=same_shape)
- [Разрезание призмы](https://optozorax.github.io/portal/?scene=cut_prism)
- [Разрезание плоскости](https://optozorax.github.io/portal/?scene=cut_plane)

### Монопорталы
- [Монопортал](https://optozorax.github.io/portal/?scene=monoportal)
- [Вращающийся монопортал](https://optozorax.github.io/portal/?scene=monoportal_rotating)
- [N-монопортал](https://optozorax.github.io/portal/?scene=monoportal_n)
- [Смещающийся монопортал](https://optozorax.github.io/portal/?scene=monoportal_offset)
- [Смещающийся монопортал 2](https://optozorax.github.io/portal/?scene=monoportal_offset2)
- [Масштабирующийся монопортал](https://optozorax.github.io/portal/?scene=monoportal_scale)
- [Масштабирующийся монопортал 2](https://optozorax.github.io/portal/?scene=monoportal_scale2)
- [Логарифмическая спираль](https://optozorax.github.io/portal/?scene=monoportal_log)

### Тройной портал
- [Тройной портал](https://optozorax.github.io/portal/?scene=triple_portal)

### Лента Мёбиуса
- [Портал Мёбиуса](https://optozorax.github.io/portal/?scene=mobius)
- [Монопортал Мёбиуса](https://optozorax.github.io/portal/?scene=mobius_monoportal)

### Другие порталы
- [Портал Хопфа](https://optozorax.github.io/portal/?scene=hopf_link)

### Портал в портале
- [Портал в портале: две пары](https://optozorax.github.io/portal/?scene=portal_in_portal_two_pairs)
- [Эллиптический портал в портале](https://optozorax.github.io/portal/?scene=portal_in_portal)

## 🖼️ Скриншоты

Скриншоты различных порталов и интерфейса программы доступны в онлайн-версии на [GitHub Pages](https://optozorax.github.io/portal/).

## 🛠️ Разработка

Программа написана на Rust с использованием:
- [macroquad](https://github.com/not-fl3/macroquad) — для рендеринга
- [egui](https://github.com/emilk/egui) — для пользовательского интерфейса
- WebAssembly — для запуска в браузере

Подпись относится к текущей версии проекта.

---

Автор: legenda  
Telegram: [@LEGENDA_SD](https://t.me/LEGENDA_SD)
